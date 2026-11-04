use crate::{
    base::{self, Res},
    distribution::io,
};
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path, time::Duration};
pub fn command(program: &str, args: &[&str], cwd: &Path) -> Res<String> {
    let mut c = base::proc::command(&[program.into()]);
    c.args(args).current_dir(cwd);
    let out = crate::process::capture(&mut c, None, Duration::from_secs(120))?;
    if out.exit_code != 0 {
        return Err(io(format!("{program}: {}", out.stderr)));
    }
    Ok(out.stdout.trim().into())
}
pub fn frontier(repo: &Path, forge: &str, pin: Option<&str>) -> Res<Vec<Value>> {
    if !["gh", "origin"].contains(&forge) {
        return Err(io("forge must be gh or origin"));
    }
    let trunk = command(
        "git",
        &["symbolic-ref", "--short", "refs/remotes/origin/HEAD"],
        repo,
    )
    .unwrap_or_else(|_| "origin/main".into())
    .trim_start_matches("origin/")
    .to_string();
    let current = command("git", &["symbolic-ref", "--short", "HEAD"], repo)?;
    if current == trunk {
        if pin.is_some_and(|p| !p.is_empty()) {
            return Err(io("frontier pin mismatch"));
        }
        return Ok(vec![]);
    }
    let query = |filter: &str, branch: &str| -> Res<Vec<Value>> {
        let fields = if forge == "origin" {
            "number,status,headRef,baseRef"
        } else {
            "number,state,headRefName,baseRefName"
        };
        let text = command(
            forge,
            &[
                "pr", "list", filter, branch, "--state", "all", "--json", fields,
            ],
            repo,
        )?;
        let mut v: Vec<Value> = serde_json::from_str(&text).map_err(io)?;
        if forge == "origin" {
            for p in &mut v {
                p["headRefName"] = p["headRef"].clone();
                p["baseRefName"] = p["baseRef"].clone();
                p["state"] = json!(match p["status"].as_str().unwrap_or("") {
                    "open" | "draft" => "OPEN",
                    "merged" => "MERGED",
                    "closed" => "CLOSED",
                    _ => "INVALID",
                });
            }
        }
        for p in &v {
            if p["number"].as_u64().unwrap_or(0) == 0
                || !p["headRefName"].is_string()
                || !p["baseRefName"].is_string()
                || !["OPEN", "CLOSED", "MERGED"].contains(&p["state"].as_str().unwrap_or(""))
            {
                return Err(io("invalid PR row"));
            }
        }
        Ok(v)
    };
    let get = |branch: &str| -> Res<Value> {
        let all = query("--head", branch)?;
        let active: Vec<_> = all
            .iter()
            .filter(|p| p["state"] == "OPEN")
            .cloned()
            .collect();
        let rows = if active.is_empty() { all } else { active };
        if rows.len() != 1 {
            return Err(io(format!(
                "branch {branch} has {} candidate PRs",
                rows.len()
            )));
        }
        Ok(rows[0].clone())
    };
    let mut seen = std::collections::BTreeSet::new();
    seen.insert(current.clone());
    let mut rows = vec![get(&current)?];
    loop {
        let base = rows[0]["baseRefName"].as_str().unwrap().to_string();
        if base == trunk {
            break;
        }
        if !seen.insert(base.clone()) {
            return Err(io("PR base chain cycle"));
        }
        rows.insert(0, get(&base)?);
    }
    let mut tip = current;
    loop {
        let rows_next: Vec<_> = query("--base", &tip)?
            .into_iter()
            .filter(|p| p["state"] == "OPEN")
            .collect();
        if rows_next.is_empty() {
            break;
        }
        if rows_next.len() != 1 {
            return Err(io("stack is not linear"));
        }
        tip = rows_next[0]["headRefName"].as_str().unwrap().into();
        if !seen.insert(tip.clone()) {
            return Err(io("PR chain cycle"));
        }
        rows.push(rows_next[0].clone());
    }
    if let Some(pin) = pin {
        let expected: Vec<u64> = pin
            .split(',')
            .map(|s| s.trim().parse().map_err(io))
            .collect::<Res<_>>()?;
        let actual: Vec<_> = rows.iter().map(|p| p["number"].as_u64().unwrap()).collect();
        if expected != actual {
            return Err(io("frontier pin mismatch"));
        }
    }
    rows.iter()
        .map(|p| {
            let branch = p["headRefName"].as_str().unwrap();
            let sha = command("git", &["rev-parse", "--verify", branch], repo)?;
            if !(40..=64).contains(&sha.len()) || !sha.bytes().all(|c| c.is_ascii_hexdigit()) {
                return Err(io("invalid branch SHA"));
            }
            Ok(json!({"pr":p["number"],"branches":branch,"sha":sha,"state":p["state"]}))
        })
        .collect()
}
pub fn run(name: &str, a: &[String]) -> Res<Value> {
    match name {
        "work-log" => {
            if a.len() != 6 {
                return Err(io("work-log FILE PHASE DECISION WHY EVIDENCE RESULT"));
            }
            let path = Path::new(&a[0]);
            crate::distribution::no_links(path)?;
            if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
                fs::create_dir_all(parent).map_err(io)?;
            }
            let mut f = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .read(true)
                .open(path)
                .map_err(io)?;
            fs2::FileExt::lock_exclusive(&f).map_err(io)?;
            if f.metadata().map_err(io)?.len() == 0 {
                writeln!(f, "ts\tphase\tdecision\twhy\tevidence\tresult").map_err(io)?;
            }
            let mut cells = vec![base::now_iso()];
            for x in &a[1..] {
                let mut x = x.replace(['\t', '\n', '\r'], " ");
                if x.starts_with(['=', '+', '-', '@']) {
                    x.insert(0, '\'');
                }
                cells.push(x);
            }
            writeln!(f, "{}", cells.join("\t")).map_err(io)?;
            Ok(json!({"status":"completed","log":path}))
        }
        "worktree-audit" => {
            let cwd = std::env::current_dir().map_err(io)?;
            let listing = command("git", &["worktree", "list", "--porcelain"], &cwd)?;
            let mut trees = Vec::new();
            for block in listing.split("\n\n") {
                let lines: Vec<_> = block.lines().collect();
                if let Some(path) = lines.first().and_then(|s| s.strip_prefix("worktree ")) {
                    trees.push(json!({"path":path,"head":lines.iter().find_map(|s|s.strip_prefix("HEAD ")),"branch":lines.iter().find_map(|s|s.strip_prefix("branch ")),"changes":command("git",&["status","--porcelain"],Path::new(path))?}));
                }
            }
            Ok(json!({"worktrees":trees}))
        }
        "check-plan" => check_plan(Path::new(
            a.first()
                .ok_or_else(|| io("check-plan requires a Markdown plan"))?,
        )),
        _ => Err(io("unknown helper")),
    }
}
pub fn check_plan(path: &Path) -> Res<Value> {
    const RULE:&str="Tests alone are not sufficient verification. A PR is verified only when its unit, live, and perf boxes are all checked.";
    const BLOCKS: [&str; 9] = [
        "Depends on.",
        "Files.",
        "Build.",
        "You see.",
        "Verify, unit.",
        "Verify, live.",
        "Verify, perf.",
        "Review gate.",
        "Merge.",
    ];
    let text = fs::read_to_string(path).map_err(io)?;
    let mut errors = Vec::new();
    let mut lines = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    let mut front = false;
    let inline = regex::Regex::new(r"!\[[^\]]*\]\([^)]*\)|\]\([^)]*\)").unwrap();
    let colon = regex::Regex::new(r": \S").unwrap();
    let lane_model = regex::Regex::new(r"Ten lanes on `[^`<>]+` at the PR head").unwrap();
    let lane = regex::Regex::new(r"^Lane (\d+)\. ").unwrap();
    let saved = regex::Regex::new(r"Save `[^`]+`").unwrap();
    for (i, l) in text.lines().enumerate() {
        if i == 0 && l == "---" {
            front = true;
            continue;
        }
        if front {
            if l == "---" {
                front = false;
            }
            continue;
        }
        let t = l.trim_start_matches(' ');
        let fence_indent = l.len() - t.len() <= 3;
        let ch = t.chars().next().unwrap_or(' ');
        let n = t.chars().take_while(|c| *c == ch).count();
        let mut code = fence.is_some();
        if let Some((c, len)) = fence {
            if fence_indent && c == ch && n >= len && t[n..].trim().is_empty() {
                fence = None;
                code = false;
            }
        } else if fence_indent
            && (ch == '`' || ch == '~')
            && n >= 3
            && (ch != '`' || !t[n..].contains('`'))
        {
            fence = Some((ch, n));
            code = true;
        }
        lines.push((i + 1, l, code));
        if !code {
            let spans = strip_code_spans(l);
            let p = inline.replace_all(&spans, "");
            for (cond, msg) in [
                (p.contains(['–', '—']), "long dash"),
                (p.contains(['‘', '’', '“', '”']), "curly quote"),
                (colon.is_match(&p), "mid-sentence colon"),
            ] {
                if cond {
                    errors.push(format!("{}:{}: {msg}", path.display(), i + 1));
                }
            }
        }
    }
    let mut sections: Vec<(&str, usize, Vec<&str>)> = Vec::new();
    for (n, l, code) in &lines {
        if !code && l.starts_with("## ") {
            sections.push((l[3..].trim(), *n, vec![]));
        } else if let Some(s) = sections.last_mut() {
            if !code {
                s.2.push(l);
            }
        }
    }
    if !lines.iter().any(|(_, l, c)| !*c && l.starts_with("# ")) {
        errors.push("no H1 title".into());
    }
    let find = |title: &str| sections.iter().position(|s| s.0 == title);
    for title in ["How to read this", "Program checklist", "Close the program"] {
        if find(title).is_none() {
            errors.push(format!("missing section {title}"));
        }
    }
    if let Some(i) = find("How to read this") {
        if let Some(h1) = lines
            .iter()
            .position(|(_, l, c)| !*c && l.starts_with("# "))
        {
            if lines
                .iter()
                .skip(h1 + 1)
                .filter(|(n, l, _)| *n < sections[i].1 && !l.trim().is_empty())
                .count()
                >= 10
            {
                errors.push("intro must be under ten lines".into());
            }
        }

        let t = sections[i].2.join("\n");
        for marker in [
            "One box is one unit of work",
            "names the evidence",
            "Check a box only when its evidence exists",
            "playbooks/",
            RULE,
        ] {
            if !t.contains(marker) {
                errors.push(format!("How to read this lacks {marker}"));
            }
        }
    }
    if let Some(i) = find("Program checklist") {
        let text = sections[i].2.join("\n");
        let mut pos = 0;
        for h in [
            "Arm the program",
            "Spawn owners",
            "PR mechanics",
            "Verdict and merge",
            "Boot recipe",
        ] {
            if let Some(n) = text[pos..].find(&format!("### {h}")) {
                pos += n + 4;
            } else {
                errors.push(format!("Program checklist lacks {h} in order"));
            }
        }
        for m in ["git show origin/main:", "/loop 1h", "status message"] {
            if !text.contains(m) {
                errors.push(format!("Program checklist lacks {m}"));
            }
        }
    }
    let box_re = regex::Regex::new(r"^\s*- \[[ x]\] (.*)$").unwrap();
    let mut prs = 0;
    if let (Some(start), Some(end)) = (find("Program checklist"), find("Close the program")) {
        for (title, _, body) in sections.iter().take(end).skip(start + 1) {
            prs += 1;
            let mut blocks: Vec<(&str, &str, Vec<&str>)> = Vec::new();
            for l in body {
                if let Some(t) = l.strip_prefix("**") {
                    if let Some((head, rest)) = t.split_once("**") {
                        if BLOCKS.contains(&head) {
                            blocks.push((head, rest.trim(), vec![]));
                            continue;
                        }
                    }
                }
                if let Some(b) = blocks.last_mut() {
                    b.2.push(l);
                }
            }
            if blocks.iter().map(|b| b.0).collect::<Vec<_>>() != BLOCKS {
                errors.push(format!("{title}: missing or reordered sub-blocks"));
            }
            for (head, rest, body) in blocks {
                let boxes: Vec<_> = body
                    .iter()
                    .filter_map(|l| box_re.captures(l).map(|c| c[1].to_string()))
                    .collect();
                if head == "Depends on." && rest.is_empty() {
                    errors.push(format!("{title}: empty dependency"));
                }
                if ["Files.", "Build.", "You see.", "Verify, unit.", "Merge."].contains(&head)
                    && boxes.is_empty()
                {
                    errors.push(format!("{title}: {head} has no box"));
                }
                if head.starts_with("Verify,") && !rest.starts_with(RULE) {
                    errors.push(format!("{title}: {head} lacks rule"));
                }
                if head == "Verify, live." {
                    if !lane_model.is_match(rest) {
                        errors.push(format!("{title}: missing lane model"));
                    }
                    let mut ns = Vec::new();
                    for b in &boxes {
                        if let Some(c) = lane.captures(b) {
                            ns.push(c[1].parse::<usize>().unwrap_or(0));
                        } else {
                            errors.push(format!("{title}: live box is not a lane"));
                        }
                        if !saved.is_match(b) || !b.contains("Pass when") {
                            errors.push(format!("{title}: lane lacks evidence or predicate"));
                        }
                    }
                    ns.sort();
                    if ns != (1..=10).collect::<Vec<_>>() {
                        errors.push(format!("{title}: expected lanes 1 to 10"));
                    }
                }
                if head == "Verify, perf."
                    && boxes
                        .iter()
                        .map(|b| b.split(' ').next().unwrap_or(""))
                        .collect::<Vec<_>>()
                        != ["Metric.", "Probe.", "Baseline.", "Rule."]
                {
                    errors.push(format!("{title}: invalid perf boxes"));
                }
                if head == "Review gate." {
                    if rest.starts_with("None.") {
                        if !boxes.is_empty() {
                            errors.push(format!("{title}: None gate has boxes"));
                        }
                    } else {
                        if boxes.is_empty() {
                            errors.push(format!("{title}: empty review gate"));
                        }
                        for m in ["screenshot", "video", "operator"] {
                            if !body.join("\n").contains(m) {
                                errors.push(format!("{title}: gate lacks {m}"));
                            }
                        }
                    }
                }
            }
        }
        if !sections
            .iter()
            .skip(end + 1)
            .any(|s| s.0.starts_with("Appendix") && s.0.contains("Prototype evidence"))
        {
            errors.push("missing Prototype evidence appendix".into());
        }
        for s in sections.iter().skip(end + 1) {
            if !s.0.starts_with("Appendix") {
                errors.push(format!("unexpected trailing section {}", s.0));
            }
        }
    }
    if prs == 0 {
        errors.push("no PR sections".into());
    }
    let report = json!({"pr_sections":prs,"problems":errors});
    if !report["problems"].as_array().unwrap().is_empty() {
        return Err(io(report.to_string()));
    }
    Ok(report)
}

fn strip_code_spans(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::new();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] != b'`' {
            let ch = text[at..].chars().next().unwrap();
            out.push(ch);
            at += ch.len_utf8();
            continue;
        }
        let count = bytes[at..].iter().take_while(|b| **b == b'`').count();
        let mut scan = at + count;
        let mut end = None;
        while scan < bytes.len() {
            if bytes[scan] == b'`' {
                let n = bytes[scan..].iter().take_while(|b| **b == b'`').count();
                if n == count {
                    end = Some(scan + n);
                    break;
                }
                scan += n;
            } else {
                scan += 1;
            }
        }
        if let Some(end) = end {
            out.push('`');
            at = end;
        } else {
            out.push_str(&text[at..at + count]);
            at += count;
        }
    }
    out
}

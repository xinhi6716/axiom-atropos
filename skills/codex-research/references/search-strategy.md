# Search Strategy

## Search as progressive refinement

Use the least expensive source capable of answering the current question. Increase search depth when it is needed to meet the agreed task or verify a consequential claim, within existing scope and authorization. Seek a user decision only when an unresolved material choice prevents that work.

A typical progression is:

1. conversation and supplied materials;
2. Web Search and Web Fetch for terminology and field orientation;
3. a small academic pilot search;
4. focused paper retrieval from the selected sources;
5. full-text acquisition and reading;
6. evidence-gap or contradiction-driven retrieval.

This progression is adaptive. Do not restart from the beginning when the user provides a precise question, known papers, or an existing research state.

## Search intent

Infer the least demanding search intent that satisfies the request:

- `EXPLORATORY`: discover vocabulary, branches, and candidate questions;
- `FOCUSED`: answer a defined question with decision-relevant evidence;
- `COVERAGE`: reduce missed literature within declared source, date, language, and publication-type boundaries.

Do not force these labels into user-facing output. In `COVERAGE`, record the important query families and sources, use known-paper recall and citation or related-paper expansion when relevant, and state the remaining coverage limits. This is still not a formal systematic or scoping review unless the required protocol and review procedures were completed.

For each substantial `COVERAGE` query, retain the exact query, source, search date, filters, sorting, requested result limit, pages or cursors covered, and any reported total or truncation. Record these in working notes or the existing research state; no separate logging system is required. Distinguish the publication-date filter from the date the search was run. Mark unknown totals or pagination support as unknown.

## Capability negotiation

Before relying on an academic connector, inspect the tools actually available in the current Codex environment.

Distinguish capabilities rather than assuming that one MCP name guarantees them:

- paper discovery;
- stable identifiers and metadata;
- explicit abstracts;
- PDF or XML download;
- readable full text;
- located passages, tables, figures, or equations;
- citation relationships;
- source-specific filters.

For `paper-search-mcp`, choose unified `search_papers` with explicitly selected sources or source-specific tools according to the question and the capabilities needed. Neither unified search nor a fixed source bundle is mandatory. Prefer source-native and open-access retrieval paths. Do not call a Sci-Hub tool. When `download_with_fallback` or an equivalent tool exposes a `use_scihub` option, set it explicitly to `false`. Otherwise use a fallback downloader only when its current configuration and tool description make clear that unauthorized sources are disabled or excluded. Tool names and capabilities may change; inspect the current tool metadata rather than assuming this exact list.

Always pass `sources` explicitly to unified `search_papers`; do not rely on its default source bundle or treat merged position as relevance ranking. A category/date listing is not keyword search: inspect the installed bioRxiv/medRxiv descriptions for category, interval or exact-DOI support, and do not use a free-text question as a category. Check returned errors and source statistics. A zero count establishes only that no records were returned, not that the source ran successfully or that relevant studies do not exist. Apply [practical MCP retrieval](mcp-retrieval.md) for source-specific query semantics, failure/credential handling, citation tools, and coverage checks; a compatibility lock does not identify the actual running package.

If the agreed evidence level depends on a capability that the current tools do not provide:

1. Explain which capability is unavailable and how that limits the current task.
2. Ask whether the user wants Codex to install or configure a suitable connector, or to continue with existing tools under an explicit coverage or evidence limitation. Present both routes in one checkpoint and recommend one.
3. If the user has not already selected a route, wait for the answer. Before approval, do not install software, edit MCP configuration, start authentication, or request credentials.
4. After approval, inspect the current official instructions and existing configuration, preserve user customizations, add only user-provided credentials through an appropriate secret mechanism, complete only the approved setup, restart when required, and verify with one harmless real tool call.
5. If the user declines, stop this MCP-dependent path and do not call the connector. Continue with existing tools only when the user selected that route in the same checkpoint or had already requested it; otherwise report the coverage limitation and wait. If setup is unavailable for another reason, follow the same boundary. Cleanup is limited to temporary files created by the attempted setup and requires existing authorization; otherwise explain the targets and ask first. Preserve existing user files, credentials, and configuration.

The official project reference is:

https://github.com/openags/paper-search-mcp

This Skill does not bundle, host, fork, or maintain the connector. Use only public/open access, the user's lawful institutional access, or user-supplied files. Do not invoke or recommend options that bypass paywalls or access controls.

## Evidence access states

Before classifying retrieved material, read and apply [evidence boundaries](evidence-reasoning.md#preserve-evidence-boundaries) for the access-state definitions and paper-identity gate. Never infer a stronger state from a tool's name or a successful return status.

A field named `abstract` may contain only a search snippet; inspect its content and provenance before assigning `ABSTRACT_READ`. Missing fields and connector defaults, including zero citation counts or blank venue names, are unknown unless verified at the responsible source.

## Web orientation

Use Web Search to discover language and candidate sources. Use Web Fetch to inspect authoritative pages.

Select authoritative sources suited to the fact being checked; these are options, not a fixed ranking:

1. standards bodies, government agencies, scientific organizations, and official documentation;
2. universities and research institutes;
3. publisher, journal, repository, and database pages;
4. high-quality reviews for vocabulary and citation discovery;
5. general pages only for orientation.

Search snippets are leads. They are not abstracts or scientific evidence. Web orientation may propose vocabulary and branches, but it must not seed an assumed scientific conclusion that the paper search merely confirms.

## Concept and query evolution

Maintain a concept model rather than a flat keyword list. Concepts may represent:

- object, population, material, system, or phenomenon;
- process, exposure, intervention, or method;
- context, environment, boundary condition, or application;
- outcome, response, performance metric, or failure mode;
- mechanism, mediator, moderator, or explanatory factor;
- comparison, exclusion, or competing interpretation.

Within a concept, expand synonyms, abbreviations, spelling variants, controlled vocabulary, formulas, legacy terms, and field-specific phrases. Across concepts, combine only meaningful intersections.

Keep true synonyms separate from broader/narrower concepts and adjacent but different objects. Similar names, formulas, abbreviations or applications do not establish equivalence; different phases, species, versions or measurement definitions may need separate branches. When that distinction could change retrieval or interpretation, check a responsible terminology source or the original study's definitions and retain the intended meaning. Broadening a query for discovery does not broaden the eligible evidence or authorize transferring results between those objects.

For a source with controlled vocabulary, inspect the relevant heading's scope and mapping/expansion behavior when consequential; combine appropriate headings with free-text names for terminology or records the headings may miss. Do not assume a broader heading searches only the desired subtype or that every current record has completed subject indexing. Translate this choice to the actual interface, rather than imposing one database's vocabulary on all sources.

Translate the concept model into the selected interface's declared syntax, searchable fields, and filter scope, using observed behavior where available. Do not assume identical Boolean, phrase, field, or date-filter behavior across sources, including sources behind one unified tool. A capability declaration is not proof that it executed correctly. When an unknown or unexpected behavior could affect the conclusion, consult the relevant tool/source documentation or use a small diagnostic query; retain unresolved behavior as unknown. Filtering returned records cannot recover records omitted during retrieval or establish equivalent coverage.

As research proceeds:

- the vocabulary pool may expand;
- individual queries should become more discriminating or broader as needed to preserve relevant coverage;
- exclusions should remove known ambiguities only after considering relevant records that also mention the excluded concept;
- evidence gaps should generate targeted queries;
- ineffective terms should be retired or marked uncertain.

If a batch is dominated by records that share vocabulary but address a different question, diagnose the mismatch before bulk reading. Check source suitability, actual query interpretation, searchable fields, ranking and failures; then choose a discriminating query, another source, or expansion from a verified in-scope seed. Do not assume every noisy batch is a query-construction defect or require narrowing when a different correction fits the evidence. Inspect representative records to establish the mismatch, and assess whether the corrected route yields eligible candidates; non-empty results alone are not success.

Record where an important term came from and what ambiguity or retrieval gap it resolves.

Distinguish retrieval concepts from screening criteria. Outcomes, mechanisms, and context may be absent from searchable fields even when a study is eligible; do not automatically require all of them in a query. Before relying on a restrictive query to declare coverage complete or a research gap, assess whether optional concepts, field limits, or exclusions could hide relevant work. Choose a proportionate diagnostic, such as relaxing a restriction or checking a known in-scope paper, when it can change the judgment. Sparse results alone do not require another search. First establish that a diagnostic paper fits the scope; a miss may reflect source coverage or indexing rather than query construction, and recovering it does not prove completeness.

## Search rounds as purposes

Do not require fixed R1-R5 labels. Select search purposes dynamically:

- vocabulary discovery;
- strict intersection of core concepts;
- known-paper verification;
- citation or related-paper expansion;
- method or measurement retrieval;
- contradictory or null-result retrieval;
- recent update search;
- evidence-gap search.

Every query should have a purpose. Avoid exhaustive keyword permutations.

## Tool-call execution

- Batch independent discovery queries in one call when the tool supports it.
- Screen and deduplicate using available identities and version information before bulk full-text retrieval. When reading a candidate is necessary to establish identity or eligibility, inspect it first; do not require unavailable metadata before opening it. Keep its identity unresolved until the match is established.
- Prioritize full-text reading by decision value; obtain and read original material for papers used to support substantive scientific conclusions. Use [full-text reading](fulltext-reading.md) to check content, handle truncation, and connect results to methods and qualifications. Discovery-only candidates need not all be downloaded.
- Retry a failed source at most once without a changed reason, then switch to a lawful alternative or record the coverage gap.
- After each substantial batch, reconcile newly added independent, high-relevance evidence with the current claims and remaining gaps before deciding to continue, refine, or stop.

## Source selection

Choose sources according to the question and available evidence, not a universal database ranking. Broad indexes, disciplinary databases, repositories, preprint servers, standards databases, and patent sources play different roles.

Preserve the source and query path for important papers. A source failure is a coverage limitation, not evidence of absence.

Characterize available sources by their coverage, indexed content, evidence depth, retrieval features, freshness, provenance, and access limits. Infer which capabilities the current question requires, then select and combine only the sources that materially help resolve it. Broad discovery, disciplinary search, citation expansion, and source-native verification are possible roles, not a required sequence or fixed bundle.

Let the question, emerging evidence, and actual tool capabilities determine the route. Revise the selection when a source proves redundant, insufficient, or unable to address a material gap. For consequential source choices, preserve a concise reason and the important coverage limits; do not require a fixed database map or treat an aggregator as authoritative for facts that should be verified at the responsible primary source. When language, region, or local practice could change the conclusion, check sources that index that literature and state the resulting coverage limitation if it stays uncovered; name source categories rather than prescribing a fixed database bundle.

## Candidate identity and deduplication

Prefer stable identities:

1. DOI;
2. PMID, PMCID, arXiv ID, or another source-stable identifier;
3. normalized title with authors and year.

Recognize that a preprint, conference paper, journal extension, correction, and repository copy may represent related publications or the same underlying study. Merge database duplicates while preserving version relationships.

Do not count multiple records, reports, reviews, or versions as independent evidence.

## Screening decisions

For a multi-paper synthesis or coverage task, make the eligibility criteria explicit enough to apply consistently: the relevant system, question, study types, comparisons, and scope limits. Reuse the agreed scope; do not add an intake gate or require a clinical question framework for unrelated disciplines.

For any candidate used to answer a scientific question, identify what it actually studies and why that bears on the current question. Judge the dimensions material to that use, not keyword overlap, source order, or a compulsory object/intervention/outcome/mechanism checklist. Distinguish directly relevant work, contextual or methodological references, unresolved fit, and unrelated records. Mere lexical neighbors cannot become key evidence; a relevant method or transferable observation may remain useful in its stated role. Missing access leaves support pending, and directly relevant contrary or null findings remain eligible. Apply [claim-specific support](evidence-reasoning.md#claim-specific-relevance-and-support) before using a finding in the synthesis.

Use titles and abstracts for initial triage, then inspect the relevant full text when eligibility depends on missing details and before using findings as scientific support. Distinguish eligible candidates, evidence actually verified, contextual or methodological background, excluded work, and unresolved eligibility. Missing abstracts or unavailable full text do not establish ineligibility; keep access limitations separate from scientific exclusion. Negative or inconvenient findings remain eligible under the same criteria as positive findings.

For plausible key candidates, use [question-specific fit](evidence-reasoning.md#assess-fit-to-the-actual-question) and [separate reliability appraisal](evidence-reasoning.md#appraise-reliability-separately-from-relevance). Choose [reading depth](fulltext-reading.md#choose-reading-depth), inspect the original structure when needed, and locate the result with its controlling methods and limitations. Screening priority is provisional; a highly relevant abstract does not predetermine evidential weight after reading.

Retain the reason for consequential exclusions or pending decisions in existing notes, alongside the stable identifier and scope used. For formal-review support, preserve the screening stage and actual record/report/study counts when required; never reconstruct flow counts from memory. If criteria change, record why and revisit affected earlier decisions rather than applying a favorable exception to one paper. Do not turn a paper's failure to support one claim into exclusion from every other use.

## Citation expansion

Use backward references to trace original methods or findings, and forward citations to look for later tests, corrections, alternative explanations, or transfer evidence. Select seeds that represent materially different branches of the question when coverage across those branches matters; a convenient or highly cited seed can overrepresent its own community. A fixed number of seeds or citation hops is not required.

For a consequential candidate, retain the seed, direction, and reason it matters. A citation edge or co-citation is a discovery relationship, not endorsement or independent replication; inspect what the citing work actually does. Deduplicate before expanding again and stop a branch when it no longer adds evidence needed for the question or declared coverage. Record unavailable citation directions or truncated lists as coverage limits; one source can establish identity without establishing broad retrieval coverage.

## Known-paper validation

Use diagnostic papers whose in-scope status is already established — supplied by the user or already verified within the current scope — and check whether retrieval returns them and identifies them correctly. Confirm first that the diagnostic paper really fits the scope; a paper outside it neither measures recall nor establishes a coverage problem. If an in-scope paper is missing, investigate the cause:

- terminology or spelling;
- date or document-type filter;
- indexing delay;
- source coverage;
- title or identifier mismatch;
- query depth or ranking.

Prefer an existing verified record over a new search, and run a specific diagnostic when it could change a coverage or gap judgment rather than as a routine extra pass every round. Use the result to improve the search strategy. Do not guarantee completeness from known-paper recall alone.

## Prioritizing papers for reading

Prioritize by decision value:

- direct relevance to the current claim;
- ability to distinguish competing explanations;
- methodological adequacy for the question;
- access to conditions, data, or limitations;
- independence from existing evidence;
- role as an original study, method, replication, boundary case, or contradiction;
- recency when the field changes rapidly;
- venue, publisher, peer-review status, article type, and scholarly influence as screening priors when they help allocate reading effort.

Establish relevance to the intended question before using publication or citation context. For relevant candidates, venue standing, peer-review context and citation influence may inform initial credibility and reading priority under [publication and citation context](evidence-reasoning.md#publication-and-citation-context-after-relevance). Consider article age, discipline and actual citation use. Article-level directness, method, internal consistency, independence, comparability and verified support control the evidential judgment; prestige or popularity cannot override a mismatch or known defect.

## Gap-driven retrieval

After each substantial retrieval batch, revise the affected claims before choosing another retrieval action. Record what the new evidence supports, weakens, contradicts, or leaves unresolved. Target gaps capable of changing the answer or meeting the declared coverage requirements, such as:

- a missing research design;
- an untested condition or population;
- conflicting outcomes;
- an alternative mechanism;
- missing negative or null results;
- uncertain independence;
- a key inaccessible full text;
- outdated coverage.

For each material unresolved claim or question, connect the missing evidence to the next targeted action and explain how its result could change the judgment or close a coverage gap. Choose the most informative feasible action; this may be a source read, an identity or independence check, or a focused query rather than another broad search.

Keep this connection in existing working notes or `research_state.md`; no separate ledger or fixed round sequence is required. Reuse claim identifiers when present. After the batch, either continue on a material gap, synthesize at the agreed evidence level, or pause for a necessary user decision. Record the reason when stopping. Apply the search-intent-specific conditions below: a sufficient answer alone does not complete a `COVERAGE` task.

Before promoting an apparent gap into an opportunity, try to disconfirm it using the most relevant feasible evidence: alternative terminology, adjacent disciplines, original studies behind reviews, later versions, counterexamples, or missing comparisons. Choose checks that address the actual uncertainty rather than running every route mechanically. An author's future-work statement is a lead, not proof that the question remains open. If evidence answers the question, retire that candidate and update the field map; do not preserve it merely to offer a novel direction.

Separate a gap in the current search or access from a substantive uncertainty in the research. For a remaining candidate, state the specific unresolved relation or condition, nearby evidence, search/access boundary, why resolving it matters to the user's goal, and what evidence or study could discriminate alternatives. Do not manufacture a gap when none is supported. A lack of independent validation or transfer evidence may be useful without being a claim that nobody has studied the topic.

## Stopping

For field onboarding, a sufficient result explains the relevant research directions and their relationships, demonstrated progress and conditions, important disputes or limitations, and supported candidate gaps with useful next steps. Deliver this incrementally. Do not chase every adjacent field or require a fixed number of papers or loops. For precise tasks, answer the specific question without forcing a field survey.

Complete the task when its requested answer and evidence/coverage requirements are met. If they are not met, the following may justify a bounded result or a pause after feasible independent work is complete:

- targeted searches no longer reduce the material uncertainty, and no feasible in-scope evidence action is likely to change the judgment;
- remaining gaps require user-provided full text or unavailable access;
- unresolved disagreement cannot be reduced with accessible evidence;
- the remaining requested claim requires a formal review method beyond the agreed task;
- the user chooses to narrow, pause, or conclude;
- a time, cost, or retrieval-count limit the user declared has been reached.

A budget or access limit counts as a stopping boundary only when the user declares it or it demonstrably exists; do not set one on your own behalf or invent one to end the work early, and do not promise unlimited retrieval. When a declared limit stops retrieval, keep the coverage limitation explicit and do not describe the result as saturation.

For `COVERAGE`, decision sufficiency alone is not a stopping condition. First complete the declared query families and sources, address material expansion gaps, confirm that successive batches add little or no new independent high-relevance evidence, and document the remaining coverage limits.

Repeated top-ranked results are not evidence of saturation while a result cap or unvisited pages may hide further matches. Where supported, paginate, increase depth, or use complementary queries to address the truncation within the agreed scope. If that is unavailable or exceeds the agreed budget, stop with the truncation recorded as a coverage limit, without claiming saturation.

Phrase absence cautiously:

> No relevant study was identified in the sources, queries, dates, and access conditions used in this session.

Do not write that no literature exists unless the claim is supported by a suitable, documented search design and even then retain the scope boundary.

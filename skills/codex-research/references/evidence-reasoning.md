# Evidence and Reasoning

## Core contract

Let Codex reason adaptively. Constrain the observable justification of consequential claims.

Do not ask Codex to reveal a private chain of thought or to follow a fixed sequence of mental steps. Require a concise, auditable account of:

- what evidence was observed;
- why it bears on the claim;
- what assumptions connect evidence to claim;
- how far the inference extends;
- where the claim may fail;
- what contradictory or alternative evidence remains.

## Consequential claims

Use the claim contract to identify the information needed when a statement:

- changes the research direction or another consequential decision;
- reports a decision-relevant quantitative value;
- synthesizes multiple studies;
- asserts a mechanism or causal relation;
- compares effectiveness, performance, safety, risk, or superiority;
- asserts universality, absence, consensus, or sufficient evidence;
- extrapolates beyond studied conditions;
- claims a research gap;
- proposes a research hypothesis or recommendation;
- resolves or suppresses a material conflict.

Ordinary definitions, bibliographic facts, and low-stakes background can remain lighter, with appropriate sources.

Scale the record to the judgment it supports. The contract is a set of questions to answer where material, not a required schema for every sentence. A brief source report may need only attribution, the relevant result, and its conditions; a disputed comparison or proposed research direction needs the assumptions and alternatives that could change it. Shared source details can be recorded once.

## Checkpoints for consequential claims

Check at the transition from reading to using evidence, from evidence to a judgment, and from that judgment to the final answer. A checkpoint closes with a supported disposition, not with a label such as "checked". Use the existing claim notes or a brief in-context record; no new file, fixed schema, or user approval is required for routine checks.

| Transition | Evidence needed to close the checkpoint | If the check fails |
|---|---|---|
| Original material read → usable evidence | Matched work/version; sections and objects actually read; claim-relevant locator and observation; any unread, truncated, or damaged material that could change the finding. Whole-paper tasks also account for substantive reading coverage. | Read the missing relevant material or repair extraction within scope. If unavailable, retain the specific access gap and leave the dependent finding unverified. |
| Evidence → consequential judgment | Compare the proposed wording with the original finding, its methods/definitions, conditions and uncertainty; compare relevant abstract/conclusion statements with the body and figures/tables/supplements. Decide whether the evidence supports, limits, contradicts, or leaves the claim unresolved. | Revise or narrow the claim, investigate a material discrepancy, or withhold the unsupported component. Do not erase the discrepancy merely by using cautious wording. |
| Judgment → final answer | Check the actual final wording and adjacent citation against the verified scope. For each material conflict or gap found above, identify where its effect is stated in the answer, or record the source-based reason it no longer affects the judgment. | Repair the affected sentence or missing qualification before delivery. Reopen reading only if that repair needs evidence; do not restart unaffected research. |

### Brief source comparison

For a paper appraisal, a consequential claim drawn from its abstract/conclusion, or a material within-source discrepancy, keep a brief paired comparison as part of the evidence record. Identify the decision-relevant assertions before summarizing them away. Separate assertions that need different checks or dispositions, such as a numerical finding, its uncertainty, its applicability, and an explanation of cause. Checking one does not close the others; compatible assertions with the same disposition may be grouped.

For each, retain `source statement + locator → original finding + locator → disposition and effect on the claim`. Use ordinary prose or a small table, in context or existing authorized notes; no separate file or fixed schema is needed. A disposition distinguishes agreement, an assertion beyond what the evidence establishes, an actual conflicting result, and a comparison still unverified. Resolve extraction, version, or definition differences before calling them scientific disagreement. This records observable evidence, not private reasoning.

Before delivery, match these assertions against the actual answer. Include a concise source-comparison result in an appraisal: each material mismatch needs its paired evidence and consequence; when the compared material is compatible, one sentence identifying the compared sections and relevant locators can close that comparison. Agreement within a paper does not establish the user's broader claim. Do not invent a conflict because an abstract is shorter, an interval is uncertain, or the user's proposed extrapolation is unsupported. An unchecked assertion stays unverified, not implicitly consistent.

If an assertion has no disposition or its material consequence has disappeared during drafting, repair that specific omission before delivery. Use the already-read evidence when sufficient; reopen the original only for a real evidence gap. Do not replace the requested scientific answer with a checklist, and do not add this comparison to metadata lookup or an explicitly abstract-only summary.

For an unresolved reporting conflict, put both statements and their respective locators next to the conflict explanation in the final answer; a citation to only one side is incomplete. Explain the consequence for the claim and distinguish the source's wording from the assessed finding. In that explanation, preserve the difference between "does not establish" and "disproves": uncertainty, absent validation, or an untested mechanism can limit an assertion without refuting it. A generic limitation, a silently weakened conclusion, or a note left only in working state does not close this checkpoint. A reconciled extraction error need not be presented as a scientific conflict; retain the repair basis when it matters to traceability.

Use independent review selectively when a consequential numerical discrepancy, causal/mechanistic interpretation, or extrapolation warrants a second reading and the host/user permits delegation. A prior missed material conflict or an unsupported assertion that sources agree is a reason to review the affected comparisons, rather than trusting another fluent rewrite. Review is not mandatory for every paper and does not transfer evidence decisions to the user.

When this review needs to discover omissions beyond an existing critique, use a fresh reviewer context: first provide the research question and access to the relevant original material, withholding the draft, known-error list and prior appraisal history. Neutral locators may help navigation but must not reveal expected discrepancies or restrict access to suspect fragments. Have the reviewer retain brief source-grounded observations and uncertainties before providing the draft for comparison; then ask which consequential assertions are missing, unchecked or wrong. If the reviewer already saw the draft or earlier findings, describe the review as draft-informed, not source-first. A fresh context limits answer cues but does not guarantee independent judgments or correct findings.

The main agent resolves findings against the source, repairs accepted issues in the answer, and checks the changed statements and any dependent conclusions before closing them. A review report alone does not complete that repair. If independent review is unavailable, perform a targeted second source check and identify the remaining uncertainty.

When structured claim notes or citation files already exist, available programmatic checks can flag missing sources/locators, abstract-only records mislabeled as verified scientific support, or unresolved citation keys. Use these only as structural checks: populated fields and passing scripts cannot establish semantic support, actual reading, or conflict resolution. This Skill does not supply an automatic semantic gate.

## Preserve evidence boundaries

Use one access-state vocabulary to describe the material actually obtained, not a scientific-quality ranking or mandatory acquisition sequence:

- `SEARCH_HIT`: discovery lead or snippet only;
- `METADATA_ONLY`: paper identity and bibliographic facts were obtained;
- `ABSTRACT_READ`: an explicit abstract was opened and read, for discovery/screening or an attributed abstract summary, not as support for a scientific conclusion;
- `FULLTEXT_FILE_AVAILABLE`: an accessible asset passed the paper-identity gate;
- `FULLTEXT_TEXT_READ`: verified article body text was actually read; parsing alone is insufficient;
- `FULLTEXT_LOCATED`: a claim-relevant body passage, table, figure, equation, or section was located and read in its relevant context in the verified original work.

A discovered or downloaded PDF, HTML page, XML file, repository copy, or supplement remains a candidate asset until the available title, authors, stable identifier, document type, and publication-version relationship establish a reliable match to the target paper. A missing DOI is not a failed identity check when title, authors, year, and provenance establish the match; record missing or conflicting metadata. Until identity is established, retain the paper's existing access state and record the asset separately; do not promote it to any `FULLTEXT_*` state.

Attach access scope and locators to the evidence used for each claim. A paper-level state is only a summary: `FULLTEXT_TEXT_READ` may cover selected sections, and `FULLTEXT_LOCATED` means a particular claim has a located passage, not that the whole paper or its supplements were read or verified. A verified excerpt can support what it contains but cannot establish that a parameter is absent from unread sections. Track the main article, supplements, and different versions separately when their access differs. In supplied synthetic or excerpt-only tasks, describe the supplied record and locator without inventing real full-text access.

Apply the same idea to data and code: when a conclusion depends on a dataset or implementation, record its source relationship, version, and what was actually read or run. Obtaining or reading code is not evidence that the reported result reproduces.

Record `HUMAN_VERIFIED` as an orthogonal verification flag when the user or researcher explicitly checks a relevant source detail. It does not replace or automatically upgrade the access state.

Full-text access does not imply validation of methods, figures, statistics, retraction status, or scientific truth.

For substantive scientific support, follow [full-text reading](fulltext-reading.md): obtain and read the relevant original material before using a candidate as evidence. Access state describes what was obtained, not permission to support a claim at any lower level. In explicitly abstract-only or supplied-excerpt tasks, give a faithful bounded account of that material without presenting its claims as independently checked findings; do not violate the user's retrieval boundary.

## Publication status

Before relying on a paper for a consequential conclusion, inspect its current publisher or responsible repository record and any linked correction, retraction, withdrawal, or expression-of-concern notice. Record the status, source link, check date, and effect on the specific claim separately from the access state. For a preprint or other non-journal work, use the responsible repository's version and withdrawal information rather than requiring a nonexistent publisher page. Before formal delivery, resolve missing checks when feasible and refresh them when a new notice or elapsed research interval could change the judgment; do not repeat an unchanged check within a short session. For tasks explicitly restricted to supplied materials or no external retrieval, use available status records and mark current external status unchecked; do not violate the task boundary to obtain it.

Use corrected findings where a correction affects the claim. Do not use a retracted or withdrawn finding as affirmative support; retain it only when needed to explain the research history or the notice itself. An expression of concern requires stating the affected uncertainty and seeking independent support. If status cannot be checked, record it as unknown rather than assuming the paper is unaffected.

## Fact checking

For a consequential factual claim, separate the checkable components before verifying it: object, relation, value, unit, condition, date, and version or status. Verify each material component against the source responsible for that fact when available, such as the article body for a reported result, the publisher for publication status, a standards body for a standard, or an official registry for a current identifier or regulatory state.

Trace numerical or methodological claims found in reviews or secondary summaries back to the original study when they affect the conclusion. Compare the original wording, table, figure, or record with the proposed claim; do not treat agreement between derivative sources as independent confirmation.

For time-dependent facts, record the effective or checked date. When sources disagree, determine whether the cause is a correction, version change, different definition, population, condition, denominator, unit, or transcription error. Preserve the conflict when it cannot be resolved. If a material component cannot be checked, narrow the claim or mark it unresolved rather than filling the gap from plausibility.

Treat disagreement within one paper—such as between the abstract, main text, table, figure, caption, or supplement—as an unresolved source conflict until the object, condition, time point, statistic, normalization, unit, rounding, and publication version are reconciled. Record each conflicting value with its locator. Values estimated from a plot must be labeled as estimates. Check the rendered source when extraction or OCR could explain the discrepancy; distinguish an extraction error from an error in the paper. Do not silently choose, average, or correct conflicting values, or hide the conflict in a rounded value or range; if the discrepancy remains material, report it and avoid relying on the precise value for a consequential claim.

### Experimental transfer

When turning a paper into an experimental plan, distinguish what the source reports, consequential missing parameters, calculations or scale conversions derived from reported values, proposed adjustments, and diagnostic hypotheses. For important source-reported conditions and results, provide the paper title, a stable clickable identifier, and the experimental section, page, figure, or table locator when available. If an identifier, locator, or parameter is unavailable, say so directly; never invent one. Say a parameter is not reported only after checking the relevant methods and available supplements; otherwise say it was not found in the material accessed.

Do not present a scale transfer, acceptance threshold, troubleshooting step, or suspected failure cause as a source finding. State the observation that motivates it, the assumption it depends on, and what result would support or reject it. Where several failure causes remain plausible, preserve them as alternatives rather than selecting one without discriminating evidence.

## Claim types

### Source report

Faithfully describes what one source reports without strengthening its scope or causal language.

Preferred language:

> The study reports...
> Under the tested conditions...

Use "The abstract states..." only for an attributed summary or unverified discovery lead. It does not turn the abstract's scientific claim into supporting evidence for the synthesis.

### Synthesis

Combines comparable evidence to identify a pattern. State the scope, evidence types, comparability, independence, and important heterogeneity.

### Interpretation

Explains why a pattern may occur. Separate compatibility with a mechanism from evidence that discriminates that mechanism from alternatives.

### Extrapolation

Transfers evidence to a different population, material, scale, setting, operating condition, metric, or time. Name the changed dimension and the assumption required.

### Hypothesis

A plausible, testable proposition not adequately established by current evidence. State what would support, distinguish, or falsify it.

Never present an interpretation, extrapolation, or hypothesis as a direct finding.

## Claim-specific relevance and support

Topical relevance and scientific support are separate decisions. For each consequential source–claim pairing, retain a concise connection: proposed claim → original observation and locator → how it bears on the claim → supported scope and material limits. Reuse the existing claim record or an in-context sentence; no additional ledger or fixed fields are required. A correct paper identity, shared vocabulary, or relevance to the broad topic does not establish support for the particular assertion.

### Assess fit to the actual question

Start from the use intended for this paper, not its title or an overall relevance score. Compare the question with the original study along the dimensions that can change the answer:

| Dimension | What to compare |
|---|---|
| Target | Actual population, material, phase, system or setting; distinguish the studied object from an application mentioned only as motivation. |
| Relation | Intervention, exposure, comparison or mechanism actually tested; distinguish a measurement/method validation from validation of the application outcome. |
| Outcome | Measured outcome and its definition; a proxy, prediction metric or preparation yield is not automatically the requested performance or mechanism. |
| Conditions | Comparator, duration, scale, environment, dose or other decisive boundaries; identify which differences require transfer assumptions. |
| Evidence type | Original data/derivation, review of earlier findings, simulation, proposed mechanism or speculation; match the type to the assertion. |

Use title/abstract screening to assign provisional roles and reading priority. If a decisive dimension is unstated, keep fit pending and inspect the relevant body; do not infer it from shared keywords or exclude the work solely for an incomplete abstract. A negative or null result can be directly relevant. For each material decision, state the match or mismatch that matters and what the paper can still be used for. Avoid a compulsory checklist for dimensions irrelevant to the question.

Distinguish direct support, indirect or transferred evidence with explicit assumptions, context or methodological reference, contradictory or limiting evidence, and unverified support. Judge the role for the specific claim: a method paper can directly support a method description without establishing an application outcome; a study in another system can inform an interpretation without proving the target-system result. Rejecting one pairing does not exclude the paper from other valid uses. Do not promote weakly related records to key evidence, suppress directly relevant contrary findings, or treat missing original material as verified support.

If only part of a sentence is supported, split or narrow it, obtain the missing evidence within scope, or withhold the unsupported component. Adding “may”, “potentially”, or another hedge does not repair an unsupported factual attribution. A synthesis or hypothesis remains possible when its observations, warrant, changed conditions and uncertainty are explicit; do not write it as a source finding or use several adjacent citations to imply direct or independent confirmation.

### Appraise reliability separately from relevance

A closely matched paper may have weak evidence; a rigorous study in a different system may be useful only as indirect evidence. Appraise the particular result, not an immutable grade for the whole article. Use the original material needed to answer the following questions; an abstract cannot establish these checks.

| Appraisal question | Decision consequence |
|---|---|
| Is the work/version authentic and is its current status known? | Apply the identity and publication-status rules. A correction can alter one result without invalidating every use; unverified identity or an affected withdrawn/retracted finding cannot supply affirmative support. |
| Can the design establish this relation? | Inspect the relevant controls, comparator, definitions, assumptions and alternative explanations under [question-matched appraisal](#match-evidence-to-the-research-question). A confounded before/after observation cannot isolate an intervention's effect. |
| Are the measurements/analysis adequate for this result? | Check independent units versus repeated observations, reference validity, denominators, uncertainty and material analysis choices. Distinguish imprecision from bias; neither a large sample nor statistical significance repairs a design defect. |
| Does the reporting support the stated interpretation? | Match methods, result, table/figure/caption and relevant supplement; use the paired comparison for abstract/conclusion claims. Missing information stays unknown; a reporting problem is not automatically misconduct or a demonstrated scientific error. |
| Is corroboration independent and applicable? | Examine the underlying study/data relationships and changed conditions. Multiple versions or reviews of one result do not add replication, and a reliable local result does not establish broad transfer. |

For each material concern, preserve `observed detail + locator → affected result/claim → consequence`. Distinguish an observed defect, a plausible risk, and information not available for assessment. Choose the consequence: usable within verified scope; usable with an explicit limitation; unresolved pending a specific check; or unusable as affirmative support for the affected claim. Keep unrelated valid findings and contextual uses. Funding or conflicts of interest may guide scrutiny but do not independently prove validity or invalidity. Data/code availability helps determine what can be checked; availability alone is not reproduction.

#### Publication and citation context after relevance

First establish the paper's relevance to its intended use. Then journal/venue standing, editorial and peer-review context, and citation influence may contribute auxiliary evidence to a provisional credibility judgment as well as reading priority. Explain which signal was used and why it is informative; keep it subordinate to the original result's design, consistency, applicability and verified support. A strong venue cannot rescue an irrelevant paper or a demonstrated defect, and low visibility cannot invalidate directly relevant sound evidence.

Use checked information rather than impressions: identify the source/date of a citation count and the year/category/source of a journal metric when that metric matters. Consider article age, discipline, document type and database coverage before comparing counts; a new paper or a narrow field may have few citations. No fixed citation/impact-factor cutoff or automatic numerical weight is required. If treating influence as credibility affects the answer, inspect a proportionate sample of consequential citing work: citation may indicate background use, criticism, extension or independent validation. Unknown citation context is influence, not confirmed endorsement; verified independent corroboration is a stronger signal than the raw count. Keep corrections, retractions and article-level defects controlling the affected evidence judgment.

### Decide whether a result can become key evidence

Before using a result to determine the answer, check that its identity/status is adequate, the claim-relevant original evidence and conditions have been located, the design can support the proposed wording, and material conflicts have been resolved or visibly limit the judgment. Record its actual role and what could change the decision. A directly relevant but methodologically weak finding cannot establish a strong claim; several weak lexical neighbors cannot compensate for missing direct evidence. Indirect evidence may inform an explicitly reasoned inference, with transfer assumptions, rather than being cited as a target-system finding.

Use [reading-depth decisions](fulltext-reading.md#choose-reading-depth) to obtain the missing information. If a required check is unavailable, keep the affected support pending or bounded instead of calling the paper reliable because it was downloaded or assigning a numerical confidence score. Reassess only decisions changed by new scope, material, status or wording. Confidence in a synthesis depends on the appraised, comparable and independent evidence across studies, not the best-looking paper alone.

## Minimal evidence and claim contracts

An important evidence record should preserve, when available:

```text
Evidence ID
Paper ID or stable identifier
Publication version and underlying study relationship
Evidence access level
Verbatim excerpt or faithful data observation
Locator: section, page, paragraph, table, figure, equation, or data row
Target object, population, system, or material
Conditions, comparator, measurement, and time boundary
Result, direction, and units when relevant
Relation: SUPPORTS | CONTRADICTS | LIMITS | CONTEXT
Source URL and access date
Paper identity status
```

For each consequential claim, maintain enough information to answer:

```text
Claim ID and claim text
Type: report | synthesis | interpretation | extrapolation | hypothesis
Scope and boundary conditions
Supporting Evidence IDs and precise locator when available
Contradicting, limiting, or contextual Evidence IDs
Warrant: why the evidence supports the claim
Assumptions and inference distance
Evidence independence
Uncertainty and confidence rationale
What evidence would change the judgment
```

This may remain in working state and be summarized naturally for the user. Expand it when the user requests an audit or when the claim carries high research consequence.

Identifiers, separate evidence/claim records, and inference-distance labels are optional organization aids. Reuse them for a long or complex synthesis; do not create empty fields, duplicate shared source details, or print an entire ledger for a short answer.

## Warrant and inference distance

The warrant is the bridge between evidence and claim. If it cannot be stated clearly, weaken or withhold the claim.

Use qualitative inference distance:

- `NONE`: faithful source report;
- `SHORT`: direct comparison or tightly scoped synthesis;
- `MODERATE`: interpretation requiring explicit assumptions;
- `LONG`: extrapolation, causal attribution from indirect evidence, or a new hypothesis.

A long inference is not automatically invalid. It carries a higher burden to expose assumptions and alternatives.

Use that room for synthesis: compare explanatory power, derive implications from stated premises, propose testable hypotheses, and make conditional recommendations where the task calls for them. An original hypothesis need not already have direct empirical confirmation; its premises must be grounded and its predicted observations distinguished from findings. Missing evidence for one inference need not prevent a supported conclusion elsewhere, and uncertainty does not make all alternatives equally plausible.

## Match evidence to the research question

Adapt appraisal to the claim rather than applying one cross-disciplinary hierarchy.

Examples:

- theoretical claims require valid assumptions, definitions, and derivation or proof;
- experimental performance claims require suitable controls, measurements, uncertainty, and operating conditions;
- simulation claims require model assumptions, calibration, verification, validation, and sensitivity where relevant;
- algorithmic comparisons require comparable datasets, baselines, metrics, leakage controls, and reproducibility details;
- observational claims require attention to selection, confounding, measurement, and temporal order;
- prototype and systems claims require realistic workloads, interfaces, failure modes, and transfer to deployment conditions;
- qualitative or case-based claims require transparent sampling, interpretation, context, and rival accounts.

Use domain-specific appraisal standards when available, but do not pretend to have completed a formal checklist unless it was actually applied.

## Independence and corroboration

Use the dependency chain `Publication → Study → Dataset/Sample/Implementation → Evidence`. Count support by underlying studies, datasets, samples, implementations, experiments, or independent causal pathways, not by publication count.

Check when feasible:

- shared datasets, samples, cohorts, specimens, or code;
- preprint, conference, journal, correction, and repository versions;
- overlapping authors, laboratories, institutions, or funders;
- repeated use of the same model, benchmark, measurement method, or source data;
- reviews that echo the same primary evidence;
- citations that ultimately trace to one original result.

Shared authors, institutions, methods, or benchmarks are clues to examine, not proof that observations are duplicated. Distinguish shared underlying data from independent measurements exposed to similar methodological biases. If independence remains unknown, record it as unknown (`INDEPENDENCE_UNKNOWN` in structured notes). Explain it to the user when it affects the conclusion. Multiple publications with unknown dependence do not establish independent replication.

## Evidence quality dimensions

Assess only the dimensions relevant to the claim. Common dimensions include:

- identity and provenance;
- directness to the research question;
- methodological adequacy;
- completeness of accessible reporting;
- independence;
- consistency and heterogeneity;
- precision and measurement uncertainty;
- applicability and transferability;
- risk of bias or selective reporting;
- model and mechanism dependence;
- vulnerability to plausible alternatives.

Do not collapse these into an unsupported universal score. Confidence labels require reasons.

## Conflict handling

Before aggregating disagreement, classify it:

- direction;
- magnitude;
- scope or boundary conditions;
- population, system, material, or setting;
- measurement or outcome definition;
- design or comparator;
- model, analysis, or adjustment;
- publication version or reporting layer.

Then decide whether the evidence should be combined, stratified, explained as heterogeneity, retained as competing conclusions, or left unresolved.

Do not use majority vote. Do not remove a material contradiction to make prose smoother. After ruling out a vote count, a qualitative weighting across directly comparable evidence is allowed when justified by directness, method quality, independence, and comparability; state the reason, and do not invent numerical weights or statistical pooling.

## Causal and mechanism claims

Association, prediction, temporal change, simulation fit, author speculation, and mechanistic plausibility do not by themselves establish causation.

For a causal claim, identify as applicable:

- intervention or exposure;
- comparator or counterfactual;
- target system or population;
- temporal order and horizon;
- outcome;
- design and identifying assumptions;
- confounding, selection, measurement, and alternative paths.

Match wording to the actual design. Prefer “associated with,” “consistent with,” or “the authors propose” when causal support is incomplete.

For mechanism claims, ask what observations discriminate the proposed mechanism from alternatives. A mechanism compatible with results remains an interpretation until discriminating evidence exists.

## Uncertainty

Treat uncertainty as part of the conclusion. Identify material sources such as:

- bias or design limitations;
- imprecision or measurement uncertainty;
- inconsistency;
- indirectness;
- selective reporting and publication bias;
- missing full text or inaccessible details;
- model dependence;
- unknown evidence independence;
- extrapolation.

Avoid invented numerical probabilities. Use calibrated qualitative language tied to explicit reasons.

## Research gaps

A search gap, reporting gap, methodological weakness, inconsistent result, and genuinely unstudied question are different.

A defensible research-gap claim should state:

- what was searched and accessed;
- what evidence exists nearby;
- what exact relation, condition, comparison, or validation remains unresolved;
- whether the gap reflects absence, insufficient quality, conflicting evidence, or inaccessible information;
- what study could reduce it.

## Publication audit

For a formal review draft or reference-bearing research artifact, distinguish three checks: local citation consistency, bibliographic identity at the responsible source, and support for the adjacent claim. Resolved keys, plausible metadata, and a DOI that opens do not establish scientific support. Where citation files exist, check for missing or mismatched entries and placeholders; keep valid source types without DOIs. Report what was actually checked instead of one undifferentiated "verified" label. Standalone formatting or reference-manager work remains outside this Skill's scope.

Reconcile named methods, baselines, datasets, and prior findings introduced during drafting with the sources actually assessed. A genuinely missing source becomes a targeted retrieval or reading task within scope; a user's own proposed method does not need an invented external citation. Resolve the gap, narrow or remove the unsupported attribution, or leave the specific limitation visible. Do not cite a merely adjacent paper to make the reference list look complete.

For claims that determine the research direction, recommendation, key quantitative comparison, mechanism, or research-gap judgment, verify the wording against the actual source evidence at its recorded access level. Use the relevant passage, table, figure, equation, or available excerpt rather than relying only on a prior summary or state entry. For synthesis or inference, check the supporting observations and warrant without treating the inferred claim as a source finding. Reuse checks already completed for unchanged claims and sources; revisit those not yet verified or affected by new wording, evidence, or publication status. If verification remains unavailable, narrow or withhold the affected claim and retain the specific limitation.

Check consequential paraphrases for changed entities, outcomes or proxy measures, quantities and units, comparator or denominator, tested conditions and duration, uncertainty, and causal strength. Preserve distinctions between a source result, an author's explanation, and your inference. Each adjacent citation must support the attributed clause; a broad-topic match or a faithful number attached to the wrong outcome does not suffice. Recheck the affected source–claim pairing when drafting changes any of these elements.

Before presenting a formal synthesis, verify:

- every consequential scientific claim has a traceable source or is labeled as inference;
- citations support the adjacent claim, including numbers, direction, objects, and conditions;
- metadata, abstract, and full-text evidence are not mixed;
- causal language matches the design;
- versions and shared evidence are not double-counted;
- each material contradiction and unresolved gap has passed the [final-answer checkpoint](#checkpoints-for-consequential-claims), with its effect visible beside the affected judgment;
- venue prestige and citation count did not substitute for appraisal;
- source or access failures were not written as evidence of absence;
- the organization and wording did not strengthen the epistemic status established during analysis.

A clear unresolved answer is preferable to a fluent overclaim.

## Method inspirations

These principles draw on, without mechanically reproducing:

- OpenAI reasoning prompting guidance: https://developers.openai.com/api/docs/guides/reasoning-best-practices
- Toulmin argument structure: https://owl.purdue.edu/owl/general_writing/academic_writing/historical_perspectives_on_argumentation/toulmin_argument.html
- Cochrane/GRADE evidence certainty: https://www.cochrane.org/authors/handbooks-and-manuals/handbook/current/chapter-14
- Cochrane result-specific risk of bias and domain-based appraisal: https://www.cochrane.org/authors/handbooks-and-manuals/handbook/current/chapter-07
- DORA's limitations of journal metrics as proxies for individual-article quality: https://sfdora.org/read/
- Cochrane interpretation and conclusions: https://www.cochrane.org/authors/handbooks-and-manuals/handbook/current/chapter-15
- National Academies causal inference overview: https://www.ncbi.nlm.nih.gov/books/NBK588337/
- National Academies reproducibility and replicability: https://www.nationalacademies.org/read/25303/chapter/3

# Interactive Research Workflow

## Purpose

Treat literature research as a collaboration that can change the question, not as a one-shot report generator. Move only as deep as the current decision requires. Pause when progress requires a material user choice about scope, priorities, resources, or authorization; investigate evidence problems within the agreed scope.

The stages below are states that Codex may revisit. They are not a mandatory sequence for every task.

## Entry modes

Infer the entry mode from the conversation. Ask only when the distinction changes the next action.

### Direction finding

Use when the user has a broad interest, an observation, a material or system, but no stable research question.

Goal:

- discover the field's language and major branches;
- expose assumptions and adjacent directions;
- help the user formulate candidate questions.

### Question deepening

Use when the user has a question but its scope, variables, comparison, or intended conclusion remains unclear.

Goal:

- make the question answerable;
- identify the evidence dimensions required;
- distinguish background, mechanism, performance, method, risk, and gap questions.

### Focused evidence review

Use when the user already has a precise question, known papers, a protocol, or explicit inclusion boundaries.

Goal:

- verify identities and coverage;
- retrieve the most decision-relevant evidence;
- synthesize within the confirmed scope.

### Coverage-oriented review

Use when the user explicitly wants broad coverage or wants to reduce missed literature within declared boundaries.

Goal:

- declare the source, date, language, and publication-type boundaries;
- use complementary query families, known-paper recall, and citation expansion when relevant;
- stop on bounded saturation and preserve a coverage statement.

Do not call this a formal systematic or scoping review unless the required protocol, duplicate screening, appraisal, and reporting workflow was actually completed.

### Known-paper or claim verification

Use when the user provides a DOI, title, PDF, or scientific claim.

Goal:

- verify the paper or claim first;
- avoid restarting broad discovery unless verification exposes a larger gap.

## Lightweight orientation

For broad or uncertain topics, begin with Codex Web Search and Web Fetch when available. Use authoritative pages to learn terminology, subfields, institutions, standards, and candidate primary sources.

Do not treat search snippets or general web pages as empirical paper evidence. Trace consequential scientific claims to primary literature before relying on them in synthesis.

Skip or abbreviate orientation when the user supplies a precise research question, a mature query strategy, or asks to continue an existing review.

## Direction checkpoint before heavy paper search

For an unclear direction, share a provisional field map after enough orientation to make it useful and before committing to a materially different or expensive retrieval route. The map may include:

- current interpretation of the user's goal;
- important terms or ambiguities discovered;
- plausible research branches;
- what paper search would resolve;
- a recommended direction;
- one decision question, only if needed to proceed.

Ask and wait only when a material scope or direction choice remains unresolved and has not been delegated. This is not a required standalone deliverable before every paper search. For a scoped request to understand the field, compare the relevant branches first; do not require a newcomer to choose a specialty before providing the requested overview. Recommend candidate personal research directions when requested; committing to a new objective or application requires the user's decision unless delegated.

Treat a clear instruction already present in the conversation as a resolved checkpoint. Do not ask the user to confirm the same scope, route, or execution preference again.

## Risk-triggered checkpoints

Use these state names in working notes when useful. They flag conditions to assess, not automatic pauses or a mandatory sequence:

- `SCOPE_UNCLEAR`
- `MCP_CAPABILITY_INSUFFICIENT`
- `DIRECTION_CONFIRMATION_REQUIRED`
- `COVERAGE_GAP`
- `FULLTEXT_REQUIRED`
- `CONFLICT_REQUIRES_DECISION`
- `READY_TO_DELIVER`

Apply the [consequential-claim checkpoints](evidence-reasoning.md#checkpoints-for-consequential-claims) within the authorized work. They test reading, judgments, and final wording; they do not create additional user-confirmation gates.

At a checkpoint that needs the user, state the trigger, Codex's recommendation, meaningful options, and any available degraded path. Do not take a degraded path without the required user choice. Treat prior explicit choices as resolved; precise or delegated tasks can continue within those choices, while scope changes and additional authorization still require a decision.

A checkpoint closes when its blocking condition is resolved, not merely when it has been reported:

- scope and direction: the user has decided, the choice was already delegated, or existing context is enough to determine the next action;
- tool capability: the available tools can meet the agreed evidence level, the user chose an alternative route, or an approved setup completed a real capability check;
- coverage and full text: the evidence was obtained, or the workflow can conclude with an explicitly bounded or unresolved answer that meets the requested task; the missing evidence itself remains recorded as unresolved. Changing required coverage or lowering an agreed evidence standard still needs the user's decision. Continue independent work while a material item remains blocked;
- conflict: the disagreement was explained, or a bounded conclusion that retains the conflict was reached; the decision for the user concerns scope and resources, not scientific truth;
- delivery: the purpose and evidence standard are established and remaining limits are preserved.

Do not ask again about a choice already made or delegated, and do not write an unresolved full text or conflict as resolved.

### Scope checkpoint

Trigger when unresolved alternatives for populations, systems, settings, outcomes, comparisons, time ranges, or evidence standards would materially change the requested result and cannot be inferred from context or an existing delegation. Multiple branches that the user asked to compare are not themselves an unresolved choice.

Present the consequence of each option and recommend one. Ask one question.

### Coverage checkpoint

Record when a major source, date range, language, discipline, or publication type remains uncovered, or when source failures materially limit the map. Continue feasible retrieval within the agreed scope and budget. Ask when further progress requires the user to change scope, resources, access, or the agreed evidence standard.

State what was covered, what was not, and whether the gap could change the conclusion.

### Direction checkpoint

Trigger when initial results reveal distinct research branches, a mistaken term, an overlooked variable, or a more promising question.

Distinguish a refinement needed to answer the current question from a change to the user's objective. Investigate the former within scope, then report what changed. For a new objective, application, or research commitment, explain the evidence and ask whether to revise direction unless that choice was delegated. An unexpected paper or new term alone does not require a pause.

### Full-text checkpoint

Before relying on a paper for a substantive scientific conclusion, check the original methods or derivation, results, and relevant qualifications using [full-text reading](fulltext-reading.md). This applies even when the abstract appears to answer the question or already contains the desired number. Keep the affected claim unresolved while checking feasible access options.

Before pausing, try one low-cost lawful alternative access path when it is reversible and likely to resolve the missing evidence. Do not repeat equivalent retrieval attempts across sources without a new reason.

List only the papers worth the user's effort. For each, explain:

- stable identifier and link;
- why it is decision-relevant;
- what cannot be verified without the full text;
- whether an accessible substitute exists.

When access still requires the user, offer the relevant options: provide the full text, pursue an available original-source alternative, or limit the deliverable to a discovery/abstract summary with the scientific claim left unverified. Such a summary does not satisfy a request for original-source confirmation. Wait only for a necessary access or scope decision, while continuing independent work within the confirmed scope. If the user does not decide, preserve the affected claim as unresolved.

### Conflict checkpoint

When comparable studies materially disagree or competing explanations remain plausible, first investigate the conflict within the confirmed scope and resources.

Before weighing the disagreement, check whether the publications represent independent studies or reuse the same dataset, sample, project, implementation, or policy intervention. Then classify the conflict by scope, conditions, measurement, design, analysis, or reporting. Use targeted retrieval when it can discriminate the competing explanations; otherwise retain unresolved alternatives. Ask the user only when proceeding requires a choice about research direction, application, scope, or additional resources, not to choose which finding is scientifically true.

### Delivery checkpoint

Trigger before turning exploratory work into a formal report, proposal input, or decision document.

Confirm the intended evidence standard and the unresolved items that must remain visible only when the conversation has not already established them or when new evidence would materially change the user's choice.

## Progress updates

A checkpoint update should be concise and decision-oriented. Follow host disclosure requirements without repeatedly narrating Skill loading or internal prompts. Adapt the headings rather than forcing a template. It normally contains:

- what is currently understood;
- what changed in this round;
- evidence level and important limitations;
- Codex's recommended next step;
- one question the user can actually decide, only when progress depends on that decision.

Do not repeat the entire history at every checkpoint. Show the delta.

## Asking well

- Ask one decision at a time.
- Propose a recommended answer and briefly explain why.
- Avoid asking for information that tools or existing files can provide.
- Avoid turning the session into an intake questionnaire.
- Preserve the user's terminology while introducing more precise field terms.
- When the user is uncertain, offer concrete alternatives grounded in preliminary findings.
- When the user delegates, make the decision and report it at the next meaningful checkpoint.

## Evidence-led follow-up questions

Decision questions control the workflow. Evidence-led questions help the user see useful next questions without requiring an immediate answer.

When new evidence exposes an important boundary, conflict, mechanism alternative, transfer problem, or missing comparison, offer a short prioritized set of questions the user is likely to find useful. Derive each question from an observed result or unresolved claim, and state briefly what answering it could change. Do not generate generic topic-expansion questions, repeat questions already answered, or turn every update into a questionnaire.

Give each material follow-up a disposition: investigate now within scope and authorization, seek a necessary user decision, or defer with a brief reason. Ranking depends on relevance to the user's goal, the judgment it could change, and feasible evidence, not novelty alone. Investigate the highest-value feasible questions before closing the current task; an optional question list does not substitute for requested work. If the user asked only for questions or prohibited retrieval, provide the questions and rationale without executing them.

When reporting to the user, select only questions that help them understand a finding or choose a meaningful next direction. Questions the agent can answer through routine research should drive its work without becoming a questionnaire. Do not invent new questions to keep the loop running after the requested result is sufficient.

## When not to interrupt

Do not pause for routine searches, metadata normalization, deduplication, citation formatting, or minor query refinements. Continue when the next action is reversible and unlikely to change the research direction.

## Ending or pausing

Before a long pause, context reset, or handoff, recommend creating or updating `research_state.md` as described in [research-state-and-delivery.md](research-state-and-delivery.md).

A task can end with an unresolved result. State the most informative next evidence or experiment instead of manufacturing closure.

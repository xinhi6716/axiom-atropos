# Academic MCP retrieval in practice

Use this guide when selecting or calling an academic MCP for discovery, coverage, citation expansion, or original-text acquisition. Inspect the tools exposed in the current session first: repository documentation, a compatibility lock, and an installed package do not establish the running server's capabilities. A restarted server may expose different tools from an existing session.

## Verify the route before depending on it

Establish tool exposure and one harmless task-relevant call. Separate server/transport success, provider execution, eligible candidates, and original-text access. For an important source, check a verified in-scope seed or inspect representative records when this can resolve uncertainty about query interpretation or execution. This is a diagnostic, not a recall estimate.

For `search_papers`, explicitly set `sources`. Compare requested sources with `sources_used`, `source_results`, and `errors`. An unknown source, omitted source, timeout, blocked response, missing credential, or returned error is an unsearched coverage branch. A zero count with no error remains unverified if the connector can swallow failures; consult available diagnostics or a proportionate source-native check. Never infer absence of studies from that response.

Provider limits are external to the Skill. Respect retry advice; do not repeatedly resend a throttled request or attempt to defeat a browser challenge. Continue relevant independent work through another suitable source. Describe the failed source and the scope lost: recovering one paper elsewhere does not recover that source's coverage. Add credentials or institutional access only with authorization and user-supplied values through local secret configuration.

## Match each source to its actual function

These are possible roles, not a mandatory bundle. Availability and supported inputs vary by connector revision and access rights.

| Source family | Useful role | Calling and evidence limits |
|---|---|---|
| OpenAlex, Crossref, Semantic Scholar | Broad discovery, identifiers, versions; citation relations where exposed | Query syntax/ranking differ. Crossref/OpenAlex `read_*` may be information-only. Semantic Scholar can be throttled without a key. |
| PubMed, Europe PMC, PMC | Biomedical discovery; repository copies and open full text | PubMed search is not a full-text reader. Match PMID/PMCID/DOI before changing access route. Repository availability is narrower than biomedical coverage. |
| arXiv, IACR, dblp, OpenReview | Relevant disciplinary papers and conference/preprint records | Choose by discipline. Preprint and journal records may be the same study. dblp metadata is not original text; OpenReview public search does not guarantee PDF access. |
| bioRxiv, medRxiv | Preprints by supported category, interval, or exact DOI | Inspect current descriptions: category/date listings are not free-text search. Some revisions accept DOI/date ranges; do not send ordinary topic queries to a category endpoint or count a recent interval as historical coverage. |
| CORE, OpenAIRE, DOAJ, HAL, Zenodo | Repository/OA discovery and alternative copies | Returned links may be landing pages, datasets, or unrelated assets. Credentials, rate limits, repository coverage, and item type affect access. |
| SSRN, ACM metadata tools | Relevant disciplinary records or publisher subsets | Inspect the underlying index. An OpenAlex/Crossref-backed tool is not an independent search of the provider's whole native database. |
| Google Scholar, CiteSeerX, BASE | Supplementary discovery when actually accessible | Automated access may be challenged, unavailable, or registration-dependent. BASE harvesting/local filtering is not necessarily ranked keyword search. Do not make these the sole basis of coverage. |
| Unpaywall | DOI-specific open-access resolution | Requires a configured contact email; not a keyword-search database or independent corroborating study. |
| Scopus, Web of Science, IEEE tools, if exposed | Additional discipline/citation-index coverage with suitable access | Metadata API credentials and entitlements are separate from full-text rights. Explicitly selected sources may require keys and may be excluded from `all`. Record unsearched subscription coverage. |

## Build a search that can find more than one branch

For a coverage request, make a concise plan in existing working notes: question/scope; concepts and their synonyms; eligible study types; source roles; query families; date/language/type limits; and unresolved branches. Use as many families and sources as the question needs, without a fixed count. Include older names, abbreviations, alternate spellings, and relevant local-language terms when they can change what is found.

Start from a discriminating core concept combination, then use complementary queries for relevant mechanisms, materials/systems, methods, conditions, contrary or null findings, and older terminology. Run these as separate families when forcing every concept into one query would hide eligible studies. Reviews can identify vocabulary and original studies; do not substitute them for the original evidence or treat their reference lists as complete.

Translate the search for each source's supported fields and syntax. Do not paste a PubMed Boolean/field-tag query into a free-text relevance endpoint and assume equivalent execution. Confirm the interpretation using representative hits and known in-scope papers when consequential. Use source-specific tools when filters matter: for example, inspect `search_pubmed` sorting, `search_crossref` filter/sort/order, and `search_openalex` filters instead of relying on unified defaults. The unified `year` parameter may apply only to Semantic Scholar; it does not establish a date filter for every selected source. The numerical depths below illustrate tool usage, not completion criteria.

```json
{"query":"\"far-UVC\" \"airborne human coronaviruses\"","sources":"pubmed,openalex","max_results_per_source":5}
```

```json
{"query":"far UVC airborne coronavirus","max_results":20,"filter":"from_publication_date:2020-01-01,to_publication_date:2025-12-31"}
```

The second example is for an OpenAlex-specific tool with that filter schema. Set dates to the actual agreed scope; inspect results rather than assuming every index interprets the phrase identically. Screen actual study subjects/outcomes before key-evidence promotion, including records returned by a precise-looking query.

## Expand and account for truncation

Search tools returning only `max_results` or `max_results_per_source` often expose no cursor. Their `total`/`raw_total` may count only retrieved records, not all matching literature. Repeating the same top results is not saturation. Increase relevant depth where supported, use complementary queries or date/topic partitions, and follow citation relations. If those routes cannot remove material truncation, report the unvisited portion; do not claim completeness or silently drop it.

Where exposed, use `get_referenced_papers` for backward expansion and `get_citing_papers` for forward expansion from verified DOI/OpenAlex seeds. Choose seeds across material branches, including contrary findings, rather than only a convenient highly cited positive study. Inspect the exact schema and returned budgets, pages, errors, and truncation. These tools may support one hop with bounded `max_results`, `max_pages`, `max_requests`, and `timeout_seconds`; they are not an unlimited citation graph.

```json
{"identifier":"10.1038/s41598-020-67211-2","max_results":10,"max_pages":2,"max_requests":4,"timeout_seconds":30}
```

This argument shape can be used with either relationship tool when exposed. Citation edges identify leads, not endorsement or independent evidence. Missing index references do not prove that the original article has no bibliography. Use the original reference list or a suitable alternative citation source for a consequential missing branch.

Deduplicate stable identifiers, preserve source provenance and version relationships, and count underlying independent studies rather than database records. Apply the same eligibility criteria to positive, negative, and inconvenient results. Retain contextual/method references and unresolved original-text candidates in their actual roles; do not manufacture direct support by broadening the claim.

Check the identity of a citation-graph seed against the verified original identifier, version and publication context, not just title/author similarity. Index records and edges can contain conflicting dates, DOIs or mixed identities. Keep such nodes/edges pending, verify consequential cited/citing papers at their original sources, and use the original bibliography or an alternative verified graph when needed. Do not silently correct index fields from plausibility or count suspicious edges as discovered independent studies.

## Obtain and verify original material

After discovery, apply [question-specific relevance](evidence-reasoning.md#assess-fit-to-the-actual-question) and [result reliability](evidence-reasoning.md#appraise-reliability-separately-from-relevance) separately. Use [reading-depth decisions](fulltext-reading.md#choose-reading-depth) to move from screening to an original-structure map and located evidence. Retrieval rank, a clean tool status and full-text availability do not establish quality or claim support.

Use a source-native reader only when it actually returns the needed original content. For metadata-only tools, follow the DOI/publisher page or a verified repository copy. If `download_with_fallback` is appropriate, pass identifiers, title, and DOI from the checked record, use a task-specific absolute save directory, and explicitly set `use_scihub=false`. Inspect the returned content/path and paper identity. A successful tool status, `.pdf` suffix, or OA URL can still be an error page or wrong asset.

Some hosts require approval for readers that also download/cache files. Do not relabel a writing tool as read-only or disable broad approval controls to make it run. When exposed, `fetch_paper_text` provides a genuinely read-only, in-memory route for a verified arXiv ID or PMCID, with `source`, `paper_id`, `max_pages` and `max_chars`. Inspect `truncated`, actual page coverage and source URL; `pmc`/`europepmc` modes may use the same Europe PMC XML endpoint, so they are not independent sources. This local extension is not assumed to exist in every upstream installation. Otherwise choose an available lawful read-only publisher/repository route or preserve the access gap.

Read relevant body methods/results and captions, and inspect figures/supplements when the claim depends on them under [full-text reading](fulltext-reading.md). If an `extract_sections` tool is exposed, its page/character/section limits are extraction limits, not proof of whole-paper reading. Check truncation, unread pages, and coverage; text extraction does not verify a visual claim. Publisher HTML, lawful repository XML, and user-supplied files can resolve access gaps when the connector's PDF path fails.

## Before moving from discovery to synthesis

For the declared coverage scope, check that material source roles and query branches were searched or have explicit failures; known in-scope papers were not missed without explanation; relevant backward/forward expansion was addressed; caps and missing languages/types/date intervals remain visible; and newly screened batches add little new independent eligible evidence. Preserve only the records needed for these judgments in the current notes, with query/date/source/filter/depth and exclusions/pending cases.

Continue discovery if a material branch is still unsearched and a feasible authorized route exists. Research can proceed on verified evidence while other branches remain pending, but an unresolved branch cannot support a literature-wide gap or absence claim. Deliver the requested synthesis only at the justified evidence and coverage level, with consequential source/claim pairings checked under [evidence and reasoning](evidence-reasoning.md#claim-specific-relevance-and-support).

<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work items for provenance-docs. Most important at top. Max 20.
Self-reorganizes based on recency and relevance.

> Last triaged: 2026-04-03. Cross-references: kova, exopack, nanosign, IRONHIVE.
> P23 Triple Lens run 2026-04-03 — top 3 items are synthesis output.

---

1. `[feature]` **`generate-toi` subcommand** — `cargo run -- generate-toi` reads `git log -1`, extracts date/hash/message, and appends a structured TOI entry stub to `TIMELINE_OF_INVENTION.md` plus emits the corresponding POA Commit Log row. Breaks the self-documentation loop: every commit currently puts f30 Stage 8 into a failing state until docs are manually updated. Phase I item #2. Highest-impact usability fix. Depends: none.

2. `[feature]` **f30: validate TOI dates against git commit timestamps** — for each TOI entry, extract the `git log --format=%ai` timestamp of the listed commit hash and verify the TOI `### YYYY-MM-DD` date is within a configurable window (default ±7 days). Closes the largest integrity hole in the provenance chain: TOI dates are currently manually entered and never cross-referenced against actual commit timestamps. Turns f30 from "self-consistent" to "tamper-evident." Paranoia lens finding. Depends: none.

3. `[build]` **GitHub Actions CI** — single workflow: `cargo clippy --deny warnings`, `cargo run` (f30 exit code), `cargo run --bin provenance-docs-test --features tests` (TRIPLE SIMS). 20 lines of YAML. The whitepaper leads with "programmatic enforcement" but the public repo has no CI badge — federal reviewers and SBIR evaluators check this first. Depends: none.

4. `[build]` **Release binary size tracking** — POA claims 346 KB main binary. Add a build step or f30 check that verifies the release binary size is within expected range. Flag if it grows past a threshold. Depends: none.

5. `[docs]` **Update supply chain audit** — `govdocs/SUPPLY_CHAIN_AUDIT.md` was written 2026-03-30. If deps change (nanosign, blake3), re-run `cargo audit`, `cargo deny`, `cargo geiger`, `cargo outdated` and update the doc. Depends: item #2 (nanosign dep added).

6. `[feature]` **TOI entry generator** — `cargo run -- generate-toi` reads `git log -1`, prompts for What/Why/AI Role, and appends a formatted entry to TIMELINE_OF_INVENTION.md. Phase I item #2. Depends: none.

7. `[research]` **Survey federal contractors for pilot interest** — Phase I item #3. Identify 3-5 SDVOSB or small business defense contractors using AI-assisted development. Document outreach in govdocs/. Depends: none (human-driven).

8. `[feature]` **f30: WHITEPAPER.md section structure validation** — verify whitepaper contains expected sections (Executive Summary, Problem, Solution, Why This Works, Integration, SBIR Work, Conclusion). Catches accidental deletions during edits. Depends: none.

9. `[build]` **CI via GitHub Actions** — single job: `cargo clippy`, `cargo run`, `cargo run --bin provenance-docs-test --features tests`. No YAML complexity — mirrors the two-binary model. Depends: none.

10. `[docs]` **Phase II IDE plugin spec** — write a design doc for VS Code extension that prompts for AI Role at commit time. Even as a spec, it advances the SBIR Phase II narrative. Depends: none.

<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work items for provenance-docs. Most important at top. Max 20.
Self-reorganizes based on recency and relevance.

> Last triaged: 2026-04-03. Cross-references: kova, exopack, nanosign, IRONHIVE.

---

1. `[feature]` **f30: verify TOI commit hashes exist in git history** — run `git log` at build time, confirm every hash in TOI `**Commit:**` fields maps to a real commit. Catches typos and fabricated hashes. Depends: none.

2. `[feature]` **f30: verify POA Commit Log covers all repo commits** — compare POA table rows against `git log --oneline` output. Flag commits missing from POA. Closes the bootstrap gap where hash-pin commits go undocumented. Depends: none.

3. `[feature]` **f30: bidirectional cross-doc check** — Stage 6 currently only checks POA→TOI. Add TOI→POA direction: every hash in TOI `**Commit:**` should appear in POA Commit Log. Depends: none.

4. `[docs]` **Whitepaper Section 2.2: add machine-readable schema proposal** — Phase I item #1. Define JSON-LD or SPDX extension for TOI/POA. Even a draft schema moves the SBIR narrative from "we should formalize" to "here's the format." Depends: none.

5. `[feature]` **NanoSign: publish standalone `nanosign` crate** — extract from kova into `github.com/cochranblock/nanosign`. Phase I item #5. 3 public functions: `sign`, `verify`, `strip`. Single dep: `blake3`. Depends: kova (`docs/NANOSIGN.md` is the spec).

6. `[build]` **Add `nanosign` as optional dependency** — behind a `nanosign` feature flag. f30 can then verify model hashes listed in a `## Model Provenance` POA section. Depends: item #5 (nanosign crate published).

7. `[docs]` **CDRL template stubs** — create `govdocs/CDRL_MAPPING.md` with actual DI-IPSC-81435, DI-IPSC-81438, DI-MGMT-81466, DI-IPSC-81441 templates showing how TOI/POA content maps to each deliverable. Phase I item #4. Depends: none.

8. `[research]` **P23 Triple Lens: evaluate TOI/POA as SPDX extension** — run optimist/pessimist/paranoia analysis on whether the framework fits inside SPDX vs standalone spec. Depends: kova IRONHIVE cluster online, kova `c2 research` command.

9. `[test]` **Expand TRIPLE SIMS to cover BACKLOG.md and govdocs/** — f30 should verify BACKLOG.md exists and govdocs/SUPPLY_CHAIN_AUDIT.md is present. All repo docs under the quality gate. Depends: none.

10. `[feature]` **f30: validate AI Role field content** — check that every `**AI Role:**` field mentions both "AI" and "Human" (or equivalent). Catches entries where attribution is one-sided. Depends: none.

11. `[docs]` **Whitepaper: add NanoSign origin authentication roadmap** — P23 paranoia lens flagged that BLAKE3 alone doesn't authenticate origin. Document the planned extension: sled registry binding or ed25519 pubkey wrapper. Section 4.4 addendum. Depends: none.

12. `[fix]` **POA screenshots go stale every commit** — the screenshots section hardcodes f30 output including hash lists that change on every commit. Consider: (a) truncate hash list in screenshot, (b) add `cargo run` to How to Verify and remove hash-level detail from screenshots, or (c) accept the lag. Depends: none.

13. `[build]` **Release binary size tracking** — POA claims 346 KB main binary. Add a build step or f30 check that verifies the release binary size is within expected range. Flag if it grows past a threshold. Depends: none.

14. `[docs]` **Update supply chain audit** — `govdocs/SUPPLY_CHAIN_AUDIT.md` was written 2026-03-30. If deps change (nanosign, blake3), re-run `cargo audit`, `cargo deny`, `cargo geiger`, `cargo outdated` and update the doc. Depends: item #6 (nanosign dep added).

15. `[feature]` **TOI entry generator** — `cargo run -- generate-toi` reads `git log -1`, prompts for What/Why/AI Role, and appends a formatted entry to TIMELINE_OF_INVENTION.md. Phase I item #2. Depends: none.

16. `[research]` **Survey federal contractors for pilot interest** — Phase I item #3. Identify 3-5 SDVOSB or small business defense contractors using AI-assisted development. Document outreach in govdocs/. Depends: none (human-driven).

17. `[docs]` **Whitepaper: update "12 repos" claims to "12 of 16"** — Phase I tooling description (line 208) says "enforced across 12 repos through exopack test binaries" — accurate but could specify "12 of 16" for consistency with Section 3.4 language. Depends: none.

18. `[feature]` **f30: WHITEPAPER.md section structure validation** — verify whitepaper contains expected sections (Executive Summary, Problem, Solution, Why This Works, Integration, SBIR Work, Conclusion). Catches accidental deletions during edits. Depends: none.

19. `[build]` **CI via GitHub Actions** — single job: `cargo clippy`, `cargo run`, `cargo run --bin provenance-docs-test --features tests`. No YAML complexity — mirrors the two-binary model. Depends: none.

20. `[docs]` **Phase II IDE plugin spec** — write a design doc for VS Code extension that prompts for AI Role at commit time. Even as a spec, it advances the SBIR Phase II narrative. Depends: none.

<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry below maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-03-27 — Whitepaper Expansion: 16 Repos, TRIPLE SIMS Coverage, Programmatic Enforcement

**What:** Updated whitepaper from 12 to 16 repositories. Added full project table with per-repo exopack feature coverage. Added Section 3.5 documenting the two-binary model and TRIPLE SIMS as programmatic enforcement of documentation compliance. Updated Phase I tooling status to reflect existing f30 validator. Updated POA validation metrics.
**Why:** The whitepaper was written when 12 repos existed. Four more shipped since (ronin-sites, railgun, ironhive, provenance-docs itself). The exopack TRIPLE SIMS enforcement layer — the mechanism that makes this framework more than a process document — was not mentioned anywhere in the whitepaper.
**Commit:** See `git log --oneline -1`
**AI Role:** AI audited all 16 repos for exopack features, TOI/POA presence, and test binary existence. AI drafted Section 3.5 and the project table. Human directed the audit scope, validated repo descriptions, and confirmed exopack feature mappings against actual Cargo.toml files.
**Proof:** [WHITEPAPER.md](WHITEPAPER.md) Section 3.4 table + Section 3.5

### 2026-03-27 — Exopack Integration: Two-Binary Model with TRIPLE SIMS

**What:** Added exopack as optional dependency behind `tests` feature. Created `provenance-docs-test` binary using `exopack::triple_sims::f60` to run the doc validation gate 3x. Added `lib.rs` with `f30` that validates TOI fields, POA sections, and required doc existence. Updated POA with Build Output, Screenshots, and self-verification commands.
**Why:** Every CochranBlock repo uses the two-binary model. provenance-docs had a stub `main.rs` with no test gate. Now it has the same quality gate as the other 11 repos — the test binary IS the CI pipeline.
**Commit:** 783564d
**AI Role:** AI implemented the exopack wiring (Cargo.toml features, test binary, f30 validation logic) and expanded POA to match whitepaper spec. Human directed the two-binary architecture, defined which TOI fields and POA sections f30 should validate, and verified TRIPLE SIMS 3/3 pass.
**Proof:** `cargo run --bin provenance-docs-test --features tests` — TRIPLE SIMS 3/3

### 2026-03-26 — Provenance Docs Whitepaper + Framework Specification

**What:** Published whitepaper defining the Provenance Docs framework — a commit-integrated documentation system for AI-piloted software development. Includes Timeline of Invention (TOI) and Proof of Artifacts (POA) specifications, CDRL mapping, SBOM integration proposal, and SBIR Phase I/II work plan.
**Why:** Federal acquisition has no standard for documenting human/AI contribution boundaries. This framework already exists across 12 production repos — formalizing it as an SBIR proposal turns operational practice into fundable research.
**Commit:** 55b2eac
**AI Role:** AI drafted whitepaper sections including legal references (Thaler v. Vidal, DFARS 252.227-7014, EO 14028). Human directed the thesis, designed the framework specification, validated all legal citations, defined the CDRL mapping, and structured the SBIR phase plan based on 13 years of defense acquisition experience.
**Proof:** [WHITEPAPER.md](WHITEPAPER.md)

### 2026-03-11 through 2026-03-26 — Framework Validation Across 12 Repositories

**What:** Deployed TOI and POA documents across all 12 CochranBlock production repositories: cochranblock, ghost-fabric, kova, pixel-forge, approuter, oakilydokily, illbethejudgeofthat, exopack, rogue-repo, wowasticker, whyyoulying, pocket-server.
**Why:** The framework is only credible if it's been used in real development. 500+ commits documented with human/AI attribution proves the system works at scale for a solo developer using AI-piloted development.
**Commit:** Various across all 12 repos
**AI Role:** AI generated initial TOI/POA templates. Human reviewed all entries for accuracy, corrected technical claims, and ensured every "AI Role" field truthfully described the human/AI boundary.
**Proof:** Visit any repo at github.com/cochranblock — every one contains both documents.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*

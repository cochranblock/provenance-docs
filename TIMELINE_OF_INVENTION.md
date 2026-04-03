<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry below maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-04-03 — P23 triple lens: readjust fire

**What:** P23 Triple Lens analysis readjusted BACKLOG top 3: generate-toi subcommand (breaks self-doc loop), f30 date-vs-git-timestamp validation (closes integrity gap), GitHub Actions CI (closes credibility gap). Updated TOI/POA to document e691e4f and 2d6f83f.
**Why:** P23 synthesis identified the self-documentation loop as the highest-impact usability gap. Without generate-toi, every commit breaks f30 Stage 8 until docs are manually updated.
**Commit:** 2d6f83f
**AI Role:** AI ran P23 Triple Lens, synthesized findings into 3 ranked actions, updated BACKLOG. Human directed the analysis scope and validated the synthesis.
**Proof:** `cargo run` — all checks passed; BACKLOG.md top 3 updated

### 2026-04-03 — P23 Triple Lens + Bug Fixes + BACKLOG Readjustment

**What:** P23 Triple Lens analysis of provenance-docs: full source read, optimist/pessimist/paranoia evaluation, synthesis into 3 concrete next actions. Fixed `validate_ai_roles` prefix-stripping bug (prefix `**ai role:**` contained `"ai "` making has_ai always true). Fixed git coverage to use `git log --oneline` instead of `--all` (stash commits were false-failing Stage 8). Replaced BACKLOG top 3 with P23 synthesis: (1) `generate-toi` subcommand, (2) f30 date-vs-git-timestamp validation, (3) GitHub Actions CI.
**Why:** The self-documentation loop breaks on every commit — f30 Stage 8 fails immediately after any commit until POA/TOI are manually updated. This is the primary usability gap. The paranoia lens also identified that TOI dates are never verified against actual git commit timestamps, which is the largest integrity hole in the provenance chain.
**Commit:** aacefa0, e691e4f
**AI Role:** AI ran the full P23 Triple Lens analysis, identified all three synthesis findings, fixed both code bugs, updated all docs. Human directed the P23 execution and validated the synthesis output.
**Proof:** `cargo test` — 93 tests pass; `cargo run` — all checks passed

### 2026-04-03 — NanoSign as Provenance Mechanism + P23 Triple Lens Validation

**What:** Integrated NanoSign into provenance-docs as a first-class AI supply chain security mechanism. Whitepaper Section 4.4 documents the 36-byte model signing standard (4-byte NSIG magic + 32-byte BLAKE3 hash) with four provenance capabilities: model integrity, chain of custody, supply chain security, SBOM integration. Added P23 Triple Lens validation to Section 4.4 — NanoSign was evaluated using three independent analyses (optimist/pessimist/paranoia) across the IRONHIVE cluster per the P23 protocol (kova/docs/KOVA_BLUEPRINT.md Section 10). Synthesis: high-confidence for integrity, origin authentication deferred to future extension. Updated POA architecture diagram with NanoSign subgraph. Created BACKLOG.md with 20 prioritized work items. Expanded f30 with Stage 7 (git history hash verification), Stage 8 (POA Commit Log completeness vs git log), Stage 9 (bidirectional TOI↔POA cross-check). Added Section 2.3: draft JSON-LD schemas for TOI and POA with `@context` at cochranblock.org/provenance/v1. Created govdocs/CDRL_MAPPING.md with templates for DI-IPSC-81435 (SDD), DI-IPSC-81438 (SPS), DI-MGMT-81466 (ECP), DI-IPSC-81441 (STR) showing how TOI/POA content maps to each federal deliverable.
**Why:** AI model files are supply chain inputs with zero provenance tracking. EO 14028 mandates supply chain transparency but no framework addresses AI models. NanoSign closes this gap in 36 bytes. P23 validation ensures the design was stress-tested from three perspectives before adoption — not just accepted on first impression.
**Commit:** 5754bf5, 3116cf0, 46f2b0f, 7ff5698, 1ccb79b, 5425bc8, 7b915ed
**AI Role:** AI drafted whitepaper Section 4.4, P23 validation paragraph, POA architecture updates, and cross-references from kova/docs/NANOSIGN.md and kova/docs/KOVA_BLUEPRINT.md. Human designed NanoSign, created the P23 protocol, directed which provenance concepts to link (SBOM, EO 14028, chain of custody), and validated all technical claims against the reference implementation.
**Proof:** [WHITEPAPER.md](WHITEPAPER.md) Section 4.4 (NanoSign + P23), [PROOF_OF_ARTIFACTS.md](PROOF_OF_ARTIFACTS.md) Architecture diagram

### 2026-04-02 — f30 Validator Expansion: Hash Validation, Date Ordering, Cross-Doc Consistency

**What:** Expanded f30 from 13 checks to 28. Added Stage 4 (TOI commit hash format validation — skips prose, enforces 7-40 char hex), Stage 5 (TOI date ordering — entries must be reverse-chronological), Stage 6 (cross-document consistency — every POA Commit Log hash must appear in TOI). Fixed three audit gaps: added missing TOI entry for 7fd287a, fixed README repo count from 14 to 16, added Validation row to whitepaper Section 2.2 POA spec table.
**Why:** The audit found that f30 only checked for string presence — it could not catch structural errors like placeholder hashes, mis-ordered entries, or POA/TOI desynchronization. These are the exact errors that a federal auditor would flag. Deeper validation moves f30 from "documents exist" to "documents are internally consistent."
**Commit:** 2c03770, be91115, 6ce4142
**AI Role:** AI implemented the three new f30 validation stages, drafted the TOI entry and POA updates. Expanded README with quick start and links. Added cross-links to cochranblock.org across all docs. Added repository/homepage to Cargo.toml. Human directed which validations to add, defined the acceptance criteria (hex-only hashes, reverse-chronological dates, cross-doc hash verification), and verified TRIPLE SIMS 3/3 pass.
**Proof:** `cargo run` shows all checks passed; `cargo run --bin provenance-docs-test --features tests` — TRIPLE SIMS 3/3

### 2026-03-31 — Pin TOI Hashes, Add POA Commit Log, Complete Self-Documentation

**What:** Pinned all TOI commit hashes to actual short hashes. Added missing TOI entries for whitepaper expansion (4f9459a) and supply chain audit (dc2bcfe). Added Commit Log table to POA with all 7 historical commits. Updated POA Validation metrics and Screenshots to match actual binary output.
**Why:** The TOI and POA were internally inconsistent — TOI had gaps for two commits, POA had no commit log, and POA screenshots did not match actual binary output. A documentation framework that fails its own standards is not credible for federal acquisition.
**Commit:** 7fd287a
**AI Role:** AI identified the missing entries and drafted TOI/POA content. Human verified all commit hashes against `git log --oneline`, confirmed POA screenshots matched actual `cargo run` output, and validated the Commit Log table was complete.
**Proof:** [TIMELINE_OF_INVENTION.md](TIMELINE_OF_INVENTION.md), [PROOF_OF_ARTIFACTS.md](PROOF_OF_ARTIFACTS.md)

### 2026-03-30 — Supply Chain Audit + Hot Reload + File Sprawl Cleanup

**What:** Full EO 14028 supply chain security audit. cargo audit (0 CVEs), cargo deny (license fix), cargo geiger (0 unsafe in application code), cargo outdated (all current), deep code review of all deps. Added hot reload via PID lockfile + SIGTERM/SIGKILL handoff using nix crate. Expanded .gitignore. Created govdocs/ directory with SUPPLY_CHAIN_AUDIT.md. Main binary now runs f30 doc validation with hot reload instead of Hello World.
**Why:** Federal-grade supply chain verification per EO 14028 and NIST SP 800-218. Hot reload enables zero-downtime binary upgrades — same pattern across all CochranBlock binaries. The main binary was a stub; now it validates docs on every run.
**Commit:** dc2bcfe
**AI Role:** AI ran all audit tools (cargo audit, deny, geiger, outdated, tree), reviewed dep source code for unsafe blocks/network calls/env var reads/command injection, and drafted SUPPLY_CHAIN_AUDIT.md. AI implemented hot reload using nix crate for POSIX signals. Human directed the audit scope, validated findings, confirmed nix upgrade from 0.29 to 0.31, and defined the hot reload PID lifecycle.
**Proof:** [govdocs/SUPPLY_CHAIN_AUDIT.md](govdocs/SUPPLY_CHAIN_AUDIT.md)

### 2026-03-29 — Whitepaper Expansion: 16 Repos, TRIPLE SIMS Coverage, Programmatic Enforcement

**What:** Updated whitepaper from 12 to 16 repositories. Added full project table with per-repo exopack feature coverage. Added Section 3.5 documenting the two-binary model and TRIPLE SIMS as programmatic enforcement of documentation compliance. Updated Phase I tooling status to reflect existing f30 validator. Updated POA validation metrics.
**Why:** The whitepaper was written when 12 repos existed. Four more shipped since (ronin-sites, railgun, ironhive, provenance-docs itself). The exopack TRIPLE SIMS enforcement layer — the mechanism that makes this framework more than a process document — was not mentioned anywhere in the whitepaper.
**Commit:** 4f9459a
**AI Role:** AI audited all 16 repos for exopack features, TOI/POA presence, and test binary existence. AI drafted Section 3.5 and the project table. Human directed the audit scope, validated repo descriptions, and confirmed exopack feature mappings against actual Cargo.toml files.
**Proof:** [WHITEPAPER.md](WHITEPAPER.md) Section 3.4 table + Section 3.5

### 2026-03-27 — Exopack Integration + POA Expansion

**What:** Added exopack as optional dependency behind `tests` feature. Created `provenance-docs-test` binary using `exopack::triple_sims::f60` to run the doc validation gate 3x. Added `lib.rs` with `f30` that validates TOI fields, POA sections, and required doc existence. Expanded POA with Build Output, Screenshots, and self-verification commands matching whitepaper Section 2.2 spec. Updated f30 to check `## Build Output` and `## Screenshots`.
**Why:** Every CochranBlock repo uses the two-binary model. provenance-docs had a stub `main.rs` with no test gate. The POA was missing required sections per its own whitepaper spec.
**Commit:** 783564d, b143ff4
**AI Role:** AI implemented the exopack wiring (Cargo.toml features, test binary, f30 validation logic) and expanded POA to match whitepaper spec. Human directed the two-binary architecture, defined which TOI fields and POA sections f30 should validate, and verified TRIPLE SIMS 3/3 pass.
**Proof:** `cargo run --bin provenance-docs-test --features tests` — TRIPLE SIMS 3/3

### 2026-03-27 — README Backlink + TOI Hash Fix

**What:** Added README.md with cochranblock.org backlink. Fixed TOI commit hash from placeholder (`See git log --oneline`) to actual hash `55b2eac`.
**Why:** Public repo needs a README for discoverability. TOI entries must reference real commit hashes — placeholders defeat the purpose of the framework.
**Commit:** b7d18be, 8e21788
**AI Role:** AI generated README content and identified the placeholder hash. Human approved the backlink target and verified the pinned hash.
**Proof:** README.md, TIMELINE_OF_INVENTION.md

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

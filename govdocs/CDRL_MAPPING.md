<!-- Unlicense — cochranblock.org -->

# CDRL Mapping — Provenance Docs to Federal Deliverables

How Timeline of Invention (TOI) and Proof of Artifacts (POA) content maps to Contract Data Requirements List (CDRL) items. Each section shows the DI number, what the government expects, and exactly where Provenance Docs provides it.

---

## DI-IPSC-81435 — Software Design Description (SDD)

**What the government expects:** System architecture, component relationships, data flows, interface definitions.

**Provenance Docs source:** POA `## Architecture` section.

**Mapping:**

| SDD Requirement | POA Content |
|----------------|-------------|
| System context diagram | Mermaid flowchart in POA Architecture |
| Component identification | Subgraph labels (e.g., "provenance-docs binary", "NanoSign") |
| Data flow | Directed edges between nodes (Dev → Code → Commit → TOI/POA → Audit) |
| Interface definitions | Build commands in How to Verify section |

**Example extraction:**
```
POA Architecture → SDD Section 4 (System Architecture)
POA Build Output → SDD Section 5 (Component Specifications)
```

---

## DI-IPSC-81438 — Software Product Specification (SPS)

**What the government expects:** Build environment, dependencies, binary descriptions, configuration data.

**Provenance Docs source:** POA `## Build Output` section.

**Mapping:**

| SPS Requirement | POA Content |
|----------------|-------------|
| Build environment | Rust edition, toolchain version |
| External dependencies | Default deps (nix) and optional deps (exopack, tokio) |
| Binary catalog | Binary names, sizes (provenance-docs: 346 KB, provenance-docs-test: 432 KB) |
| Cloud dependencies | Explicitly documented as "Zero" |
| Infrastructure cost | Documented ($0) |
| Supply chain audit | Cross-reference to govdocs/SUPPLY_CHAIN_AUDIT.md |

**Example extraction:**
```
POA Build Output table → SPS Section 3 (Build Description)
govdocs/SUPPLY_CHAIN_AUDIT.md → SPS Appendix A (Dependency Audit)
```

---

## DI-MGMT-81466 — Engineering Change Proposal (ECP)

**What the government expects:** Dated record of changes, justification, impact assessment, approval authority.

**Provenance Docs source:** TOI entries.

**Mapping:**

| ECP Requirement | TOI Field |
|----------------|-----------|
| Change date | `### YYYY-MM-DD` header |
| Change description | `**What:**` field |
| Justification | `**Why:**` field |
| Configuration identification | `**Commit:**` field (git short hash) |
| Impact assessment | Implicit in What/Why — scope of change is visible |
| **AI contribution boundary** | `**AI Role:**` field — not in standard ECP, but fills the attribution gap |

**Key advantage:** Traditional ECPs don't track AI contributions. The `**AI Role:**` field extends the ECP format with human/AI attribution, which is the core innovation of Provenance Docs. Each TOI entry is an ECP that also answers "did a human or AI make this decision?"

**Example extraction:**
```
TOI entry → ECP (one entry per change)
TOI **Commit:** → ECP Configuration Item Identifier
TOI **AI Role:** → ECP Appendix: AI Attribution (new field)
```

---

## DI-IPSC-81441 — Software Test Report (STR)

**What the government expects:** Test procedures, test results, pass/fail criteria, test environment description.

**Provenance Docs source:** POA `## How to Verify` and `## Screenshots` sections.

**Mapping:**

| STR Requirement | POA Content |
|----------------|-------------|
| Test procedures | How to Verify: exact `cargo run` commands |
| Test environment | Build Output: Rust edition, dependency list |
| Test results | Screenshots: terminal output showing pass/fail |
| Pass/fail criteria | f30 validator: 0 = pass, nonzero = fail |
| Regression detection | TRIPLE SIMS: 3 consecutive passes required |

**Test gate detail:**
```
Stage 1: Required docs exist (TOI, POA, WHITEPAPER)
Stage 2: TOI field completeness (What, Why, Commit, AI Role, Proof)
Stage 3: POA section completeness (Architecture, Build Output, Validation, Screenshots, How to Verify)
Stage 4: Commit hash format validation (7-40 char hex)
Stage 5: Date ordering (reverse-chronological)
Stage 6: Cross-doc consistency (POA hashes in TOI)
Stage 7: Git history verification (TOI hashes in git log)
Stage 8: POA completeness (all git commits in POA Commit Log)
Stage 9: Bidirectional cross-check (TOI hashes in POA Commit Log)
```

**Example extraction:**
```
POA How to Verify → STR Section 3 (Test Procedures)
POA Screenshots → STR Section 4 (Test Results)
POA Validation → STR Section 5 (Test Summary)
```

---

## NanoSign CDRL Extension

NanoSign model integrity data maps to SBOM deliverables:

| SBOM Requirement | NanoSign Content |
|-----------------|-----------------|
| Component identification | Model filename + format (safetensors, GGUF, etc.) |
| Version/hash | BLAKE3 32-byte hash |
| Integrity verification | `nanosign verify` command |
| Supply chain provenance | sled registry: who baked the model, when, what hash |

This extends DI-IPSC-81438 (SPS) with AI model supply chain data that no current CDRL captures.

---

## Usage

A contractor delivering under a Provenance Docs framework can satisfy CDRL requirements by:

1. Delivering the repository (which contains TOI and POA)
2. Running `cargo run --bin provenance-docs-test --features tests` to prove compliance
3. Mapping this document's sections to the specific CDRLs in their contract

The validator binary IS the test report. The TOI IS the change log. The POA IS the design description. No separate documents needed — the code repository is the deliverable.

---

*Cross-reference: [WHITEPAPER.md](../WHITEPAPER.md) Section 4.1*

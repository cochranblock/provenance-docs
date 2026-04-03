# PROVENANCE DOCS

**A Commit-Integrated Documentation Framework for AI-Piloted Software Development**

*Author: The Cochran Block*

---

## Executive Summary

The federal government has no standard for documenting who did what when AI assists software development. Existing acquisition documentation (SOWs, CDRLs, technical reports) assumes human authorship. AI-assisted development creates an attribution gap that threatens intellectual property rights, security audits, and procurement integrity.

Provenance Docs is a working framework — already deployed across 16 production repositories — that solves this with two documents embedded in the development workflow:

1. **Timeline of Invention (TOI)** — a dated, commit-linked record with a mandatory "AI Role" field on every entry, documenting what the human directed versus what the AI generated.
2. **Proof of Artifacts (POA)** — architecture diagrams, build metrics, screenshots, and verification commands proving the software is real, runs, and does what it claims.

Compliance is enforced programmatically: 12 of the 16 repositories include a dedicated test binary that validates TOI and POA completeness via TRIPLE SIMS — a flaky-test-proof quality gate that runs every check three times and fails on the first deviation.

This is not a proposal for a tool that might work. This is documentation of a tool that already works, validated across 16 Unlicense Rust repositories with 500+ commits of production software.

## 1. The Problem: AI Broke the Attribution Chain

### 1.1 The Patent Office Can't Tell Who Invented What

The USPTO requires that patent applications name human inventors. When an AI generates a function, refactors an algorithm, or proposes an architecture — and a human accepts, modifies, and ships it — who invented it? Current documentation practices provide no answer.

The Federal Circuit ruled in *Thaler v. Vidal* (2022) that AI cannot be named as an inventor. But it left open the question: how do you document the human's inventive contribution when AI assisted the process?

### 1.2 Defense Acquisition Has No AI Audit Trail

DFARS 252.227-7014 (Rights in Noncommercial Computer Software) requires contractors to identify computer software developed at private expense versus government expense. When AI generates code during a government contract, the IP boundary becomes ambiguous:

- Did the contractor's human engineer create the architecture? (Private expense — contractor retains rights)
- Did the AI generate the implementation from a prompt? (Unclear provenance — potential IP dispute)
- Was the AI trained on government data? (Government purpose rights may apply)

No current CDRL or SOW template captures this distinction.

### 1.3 Supply Chain Security Can't Audit Generated Code

Executive Order 14028 (Improving the Nation's Cybersecurity) mandates software supply chain transparency. NIST SP 800-218 (Secure Software Development Framework) requires provenance tracking for software components. Neither framework addresses AI-generated code as a supply chain input.

A developer who copy-pastes AI output into a codebase is introducing an unaudited component with no bill of materials, no version history, and no vulnerability disclosure process. This is a supply chain attack vector hiding in plain sight.

The same gap exists for AI model files themselves. A `.safetensors` or `.gguf` downloaded from the internet ships unsigned — no integrity verification, no tamper detection. Provenance Docs addresses this through NanoSign (Section 4.4): 36 bytes appended to any model file, providing self-verifying integrity via BLAKE3 hashing.

## 2. The Solution: Two Documents, One Workflow

Provenance Docs introduces two machine-readable, human-auditable documents that live in the repository alongside the code they describe.

### 2.1 Timeline of Invention (TOI)

**Purpose:** Establish a dated record of human-directed invention for every significant deliverable.

**Format:** Reverse-chronological entries, each containing:

| Field | Description |
|-------|-------------|
| **Date** | When the work shipped (not when it started) |
| **What** | Concrete deliverable — binary, feature, fix, architecture change |
| **Why** | Business or technical reason driving the decision |
| **Commit** | Git short hash(es) for traceability |
| **AI Role** | What the AI generated versus what the human directed and verified |
| **Proof** | Link to artifact, screenshot, or test output |

**The "AI Role" field is the critical innovation.** It forces the developer to articulate, at commit time, the boundary between human direction and AI execution. This creates an auditable chain of custody for every design decision.

**Example entry:**

```
### 2026-03-26 — Ghost Fabric Whitepaper + Rust Scaffold

What: Published whitepaper on sovereign edge intelligence over sub-GHz
      cognitive mesh networks. Created Rust binary scaffold.
Why:  Defines technical thesis for edge AI on LoRa — needed for SBIR
      proposals and as public proof of the approach.
Commit: f502788, 24e2817
AI Role: AI drafted whitepaper sections and added technical specifics
         (LoRa throughput, L3 cache sizing, dependency counts). Human
         directed the thesis, validated all claims against hardware
         specs, and corrected cache-resident execution claims for accuracy.
Proof: WHITEPAPER.md
```

### 2.2 Proof of Artifacts (POA)

**Purpose:** Provide verifiable evidence that the software exists, builds, runs, and does what the TOI claims.

**Standard sections:**

| Section | Content |
|---------|---------|
| **Architecture** | Mermaid diagram or ASCII wire showing system topology |
| **Build Output** | Binary size, infrastructure cost, external dependencies, cloud dependencies |
| **Validation** | Framework-wide metrics: repo coverage, test gate adoption, total commits documented |
| **Screenshots** | Visual proof of running software |
| **How to Verify** | Exact commands to reproduce the build and run the software |

**The verification commands are mandatory.** Any reviewer can clone the repo, run the commands, and confirm the software works. This is not documentation — it is a reproducibility contract.

### 2.3 Machine-Readable Schema (Draft)

The markdown format is human-first. For tooling, CI integration, and SBOM pipelines, the same data can be expressed as structured JSON-LD. This is a draft proposal — Phase I work item #1 is to formalize it.

**TOI Entry Schema:**

```json
{
  "@context": "https://cochranblock.org/provenance/v1",
  "@type": "TimelineEntry",
  "date": "2026-03-26",
  "title": "Ghost Fabric Whitepaper + Rust Scaffold",
  "what": "Published whitepaper on sovereign edge intelligence...",
  "why": "Defines technical thesis for edge AI on LoRa...",
  "commits": ["f502788", "24e2817"],
  "aiRole": {
    "aiGenerated": "Drafted whitepaper sections, added technical specifics",
    "humanDirected": "Directed thesis, validated claims against hardware specs"
  },
  "proof": ["WHITEPAPER.md"]
}
```

**POA Schema:**

```json
{
  "@context": "https://cochranblock.org/provenance/v1",
  "@type": "ProofOfArtifacts",
  "repository": "https://github.com/cochranblock/ghost-fabric",
  "architecture": { "format": "mermaid", "content": "..." },
  "buildOutput": {
    "binarySize": "346 KB",
    "edition": "2024",
    "cloudDeps": 0,
    "supplyChainAudit": "govdocs/SUPPLY_CHAIN_AUDIT.md"
  },
  "validation": {
    "repoCount": 16,
    "testGateCoverage": "12 of 16",
    "totalCommits": 500
  },
  "verification": [
    "cargo build --release",
    "cargo run --bin provenance-docs-test --features tests"
  ],
  "nanosign": {
    "modelHash": "blake3:a1b2c3d4...",
    "registry": "model_hashes.sled"
  }
}
```

**Design decisions:**
- JSON-LD over SPDX extension: SPDX is designed for software components, not human/AI attribution. A standalone `@context` keeps the schema self-contained and avoids forcing TOI semantics into package-dependency models.
- The `aiRole` object separates `aiGenerated` from `humanDirected` — this is the machine-readable form of the mandatory "AI Role" field.
- `nanosign` is optional — present only when AI model files are part of the supply chain.
- `commits` is an array of short hashes that can be expanded to full SHA via `git rev-parse`.

## 3. Why This Works

### 3.1 Embedded, Not Retroactive

Unlike a final technical report written after delivery, TOI and POA are maintained during development. Entries are written when the work ships, not months later from memory. This makes them more accurate and harder to fabricate.

### 3.2 Git-Verifiable

Every TOI entry references a git commit hash. The commit is timestamped, immutable (absent force-push), and contains the actual code diff. A reviewer can trace any claim in the TOI to the exact lines of code that changed. This is stronger evidence than any Word document.

### 3.3 AI Role as First-Class Field

No other documentation framework includes a mandatory human/AI attribution field. By making "AI Role" required on every entry, the framework normalizes the practice of attributing AI contributions — making it routine rather than exceptional.

### 3.4 Already Validated at Scale

This framework is not theoretical. It is deployed across 16 production Rust repositories at cochranblock.org:

| Repository | Purpose | Exopack Test Gate |
|------------|---------|-------------------|
| cochranblock | Production web server (cochranblock.org) | triple_sims, screenshot, devtools |
| kova | AI augment engine with agent loop and tool use | triple_sims, interface, baked_demo, video |
| approuter | Reverse proxy with Cloudflare tunnel automation | interface |
| oakilydokily | ESIGN-compliant waiver system with multi-auth | triple_sims, screenshot, devtools |
| pixel-forge | On-device diffusion model pipeline | — |
| ghost-fabric | Sovereign edge intelligence over sub-GHz mesh | — |
| whyyoulying | DoD labor-category fraud detection | triple_sims |
| pocket-server | Phone-as-web-server | — |
| rogue-repo | HTTP API backend + endless runner game | triple_sims, interface |
| wowasticker | Sticker product platform | triple_sims |
| illbethejudgeofthat | Google Takeout to custody court documents | triple_sims |
| exopack | Testing framework and quality gate library | (is the library) |
| provenance-docs | This framework — whitepaper, TOI/POA spec, validator | triple_sims |
| ronin-sites | Multi-auth web application platform | triple_sims, screenshot, devtools |
| railgun | Coordination daemon for multi-AI collaboration | — |
| ironhive | Lean file sync daemon over SSH | — |

12 of 16 repositories enforce TOI/POA compliance through exopack test binaries. 500+ commits across these repositories are documented with TOI entries and backed by POA build evidence.

### 3.5 Programmatic Enforcement via TRIPLE SIMS

Documentation frameworks fail when they rely on process discipline alone. Provenance Docs enforces compliance through code.

Each repository follows a **two-binary model**:

1. **Production binary** — the application itself, with zero test dependencies
2. **Test binary** (`*-test`) — a quality gate that validates documentation completeness, runs integration tests, and reports pass/fail

The test binary uses **exopack**, a Rust testing library, to execute the TRIPLE SIMS gate: every validation runs three times sequentially via `exopack::triple_sims::f60`. All three passes must succeed. This catches flaky validations, race conditions, and non-deterministic failures that a single-pass check would miss.

The test binary IS the CI pipeline. No external test runner. No YAML. No cloud service. A single `cargo run --bin *-test --features tests` command validates the entire repository — code, documentation, and compliance — in one shot.

This means a reviewer can clone any CochranBlock repository and run one command to verify that the TOI and POA are present, structurally complete, and that the software builds and passes its own quality gate.

## 4. Integration with Federal Acquisition

### 4.1 CDRL Integration

TOI and POA map directly to existing Contract Data Requirements List items:

| CDRL | TOI/POA Mapping |
|------|----------------|
| DI-IPSC-81435 (Software Design Description) | POA Architecture section |
| DI-IPSC-81438 (Software Product Specification) | POA Build Output section |
| DI-MGMT-81466 (Engineering Change Proposal) | TOI entries (each is a dated change record) |
| DI-IPSC-81441 (Software Test Report) | POA How to Verify section |

### 4.2 SBOM Enhancement

NTIA's Software Bill of Materials (SBOM) framework tracks third-party components. Provenance Docs extends this to track AI-generated components — treating AI output as a supply chain input that requires attribution, versioning, and audit.

NanoSign model hashes (Section 4.4) integrate directly into SBOM: each AI model used during development gets a 32-byte BLAKE3 hash that appears in the software bill of materials alongside traditional dependency entries. This closes the gap where AI models are treated as tools rather than auditable supply chain inputs.

### 4.3 FAR/DFARS Compliance

The "AI Role" field directly addresses the provenance question in DFARS 252.227-7014. When a contract requires identification of privately-developed versus government-funded software, the TOI provides line-item attribution of human versus AI contribution at the commit level.

### 4.4 NanoSign — AI Model Integrity in 36 Bytes

AI model files ship unsigned. A developer downloads a `.safetensors` or `.gguf` from the internet and trusts it blindly — no integrity check, no tamper detection, no chain of custody. Existing signing solutions (GPG, X.509, sigstore) require infrastructure and ceremony. Nobody uses them for model files.

[NanoSign](https://github.com/cochranblock/nanosign) solves this by appending 36 bytes to any model file:

| Component | Size | Content |
|-----------|------|---------|
| Magic | 4 bytes | `NSIG` (ASCII) |
| Hash | 32 bytes | BLAKE3 hash of everything before these 36 bytes |

The file becomes self-verifying. No external registry. No key server. No setup. BLAKE3 runs at memory bandwidth (~6 GB/s) — a 4GB model verifies in under 1 second.

**How NanoSign extends Provenance Docs:**

1. **Model integrity** — tamper detection on AI weights. Any modification to the model file (poisoning, truncation, corruption) changes the hash. Verification is one function call.
2. **Chain of custody** — a sled-backed registry tracks who baked what model, when, and what hash it produced. This is the AI model equivalent of the TOI: a dated, attributable record.
3. **Supply chain security** — before any inference call, verify the model came from a trusted source. This addresses the EO 14028 gap where AI models are supply chain inputs with no provenance tracking.
4. **SBOM integration** — model BLAKE3 hashes appear in the software bill of materials alongside traditional dependency entries. A model is a dependency. It should be inventoried like one.

NanoSign is format-agnostic (safetensors, GGUF, ONNX, PyTorch, nanobyte), backward-compatible (existing tools ignore trailing bytes), and implemented in 3 lines of Rust with a single dependency (`blake3`). The reference implementation lives in the kova augment engine and is deployed across the IRONHIVE distributed inference cluster.

**P23 validation:** The NanoSign design was evaluated using the P23 Triple Lens Research Protocol — three independent analyses (optimist, pessimist, paranoia) run in parallel across the IRONHIVE cluster, then synthesized into a single assessment. The optimist lens confirmed the 36-byte format solves the unsigned-model problem with zero infrastructure. The pessimist lens identified that BLAKE3 alone does not authenticate origin (who signed, only what was signed). The paranoia lens flagged model-poisoning attacks where an adversary re-signs a tampered file. Synthesis: NanoSign is high-confidence for integrity verification; origin authentication requires a future extension (registry or public key binding). This is documented and accepted — integrity first, authentication second.

**Origin authentication roadmap:** NanoSign v1 answers "has this file been modified?" but not "who created it?" Two planned extensions address this:

1. **Registry binding (v1.1)** — the sled-backed hash registry already stores `(model_name, blake3_hash)`. Extending to `(model_name, blake3_hash, source_id, timestamp)` adds origin tracking without changing the file format. The 36-byte trailer stays the same; the registry adds the "who" and "when." This is local-first — no external infrastructure.

2. **Public key signing (v2)** — append an additional 64 bytes (ed25519 signature) after the BLAKE3 hash, with a new magic `NSG2`. The file becomes: `[payload][NSIG][blake3_32b][NSG2][ed25519_sig_64b]`. Verification checks both integrity (BLAKE3) and origin (signature). Public keys are distributed via the registry or out-of-band. This adds 100 bytes total overhead (still under 0.000003% of a 4GB model).

The v1.1 registry extension is planned for Phase I. The v2 public key extension is Phase II work — it requires key management decisions that should be informed by government pilot deployments.

## 5. Proposed SBIR Work

### Phase I ($50K-$275K, 6-12 months)

1. **Formalize the specification** — publish TOI and POA as machine-readable schemas (JSON-LD, SPDX extension, or standalone spec)
2. **Build the tooling** — CLI tool that generates TOI entries from git commits, integrates with CI/CD pipelines, and validates POA completeness (POA structural validation is already implemented in `provenance-docs` via `f30` and enforced across 12 of 16 repos through exopack test binaries)
3. **Pilot with 3-5 federal contractors** — validate the framework against real CDRL deliveries and security audits
4. **Publish compliance mapping** — formal mapping to DFARS, NIST SSDF, EO 14028, and SBOM requirements
5. **NanoSign standardization** — publish NanoSign as a standalone crate and propose the 36-byte model signing format as an AI supply chain security standard for SBOM integration

### Phase II ($500K-$1.5M, 24 months)

1. **IDE integration** — VS Code and JetBrains plugins that prompt developers for AI Role attribution at commit time
2. **Automated POA generation** — CI pipeline that auto-generates architecture diagrams, build metrics, and screenshot captures on every release
3. **Government pilot deployment** — deploy to a DoD program office for real-world acquisition cycle testing
4. **Patent landscape analysis** — map the framework against AI inventorship case law and propose policy recommendations

## 6. Why The Cochran Block

- **13 years defense and enterprise** — Army 17C (Cyber Operations), USCYBERCOM J38 dev lead for a Congressional NDAA-directed offensive cyber operations study
- **The framework is already built** — 16 repos, 500+ commits, 12 with programmatic enforcement, all publicly auditable at [github.com/cochranblock](https://github.com/cochranblock)
- **Zero-cloud architecture** — the tooling runs as a compiled Rust binary with no cloud dependencies, suitable for classified environments
- **SDVOSB** — Service-Disabled Veteran-Owned Small Business (pending certification)
- **Unlicense** — all source code is public domain, eliminating IP concerns for government adoption

## 7. Conclusion

AI-assisted development is already happening inside every defense contractor and federal agency. The question is not whether to allow it — it's how to document it. Provenance Docs provides the answer: two documents, embedded in the workflow, with a mandatory human/AI attribution field on every entry.

The framework exists. The code is public. The proof is at [github.com/cochranblock](https://github.com/cochranblock).

---

*The Cochran Block, LLC — Dundalk, MD*
*SDVOSB (Pending) · SAM.gov Registered · [cochranblock.org](https://cochranblock.org)*

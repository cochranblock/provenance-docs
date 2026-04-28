# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface (project-specific)

`provenance-docs` is the framework itself — it emits the canonical TOI/POA chain-of-custody records for 16 downstream repositories and validates their structural integrity via `f30`. If this project is compromised, every repo that inherits its pattern inherits forged provenance. The blast radius is the portfolio, not just this repo.

**Records of consequence this project emits:**

- `TIMELINE_OF_INVENTION.md` — dated human/AI attribution entries. Legal record supporting DFARS 252.227-7014 attribution, patent inventorship claims (Thaler v. Vidal), and CDRL DI-MGMT-81466 change records.
- `PROOF_OF_ARTIFACTS.md` — build metrics, commit log, named techniques registry. Feeds CDRL DI-IPSC-81435 (Software Design Description), DI-IPSC-81438 (Software Product Specification), DI-IPSC-81441 (Software Test Report).
- `WHITEPAPER.md` — framework specification referenced by SBIR Phase I/II proposals.
- `govdocs/SUPPLY_CHAIN_AUDIT.md`, `govdocs/CDRL_MAPPING.md` — federal-acquisition-facing compliance artifacts.
- `f30` validator pass/fail signal — the gate that downstream repos' exopack test binaries depend on to certify their own TOI/POA compliance.

**Project-specific threats:**

- **Validator compromise → silent forged entries.** `f30` is the only enforcement layer. If its source is tampered or its binary is replaced, fabricated TOI entries pass all 28 structural checks and propagate as "audited" to every downstream consumer. Self-validating code is self-lying code if the validator itself is the attack surface.
- **TOI date forgery.** Dates are currently manually entered and never cross-referenced against `git log --format=%ai` for the cited commit. An attacker (or a careless author) can backdate or forward-date an invention claim without tripping `f30`. This is the largest integrity hole — BACKLOG item #2 is the planned fix that promotes the chain from "self-consistent" to "tamper-evident."
- **Commit hash citation without resolution.** `f30` verifies hash *format* (7–40 hex chars) but does not verify the hash resolves to an actual object in the repository. A plausible-looking but fictional or alien-repo hash passes validation.
- **AI Role field falsification.** The "AI Role" field is free-form prose. Nothing prevents an author from claiming "Human-written" on an AI-generated commit. Detection must be external (code review, linguistic analysis, behavioral signals).
- **Force-push rewriting git history.** TOI entries cite git commits as evidence. If `main` is force-pushed on this repo or any downstream repo, cited hashes can be removed or replaced. GitHub retains a reflog but it is not publicly queryable by default — the external witness thins out once a branch is rewritten.
- **Supply chain (`exopack` + `nix` + `tokio`).** The test-feature build pulls `exopack` as a git dep (not a registry-pinned version), which transitively pulls `blake3`, `tokio`, and others. A compromised upstream crate could alter TRIPLE SIMS pass/fail semantics without any visible source change in this repo. `Cargo.lock` pins revisions but not content hashes for git deps.
- **`generate-toi` subcommand injection.** `generate-toi` reads `git log -1` output and writes directly into `TIMELINE_OF_INVENTION.md`. A crafted commit subject line (shell metacharacters, markdown-breaking tokens, control chars, HTML-comment injection) could corrupt the doc structure or sneak text past reviewers reading the rendered markdown.
- **Downstream drift.** Each of the 16 repos ships its own copy of the TOI/POA pattern. If this canonical repo updates the required fields or validation rules, downstream repos silently diverge until their next sync. Drift = attribution gaps at acquisition time.

**N/A for this project:**

- **Storage compromise at DB level.** No sled / no database layer. All state lives in markdown files and git. Append-only discipline is enforced by git history + public remote, not by a storage engine. (The sled registry referenced in the whitepaper lives in `nanosign`, not here.)
- **Signing key theft.** No hardware-key signatures on TOI/POA are currently integrated or claimed. Authenticity rests on GitHub commit signatures (where configured) and human review. There is no key to steal from this repo.
- **Network MITM on runtime data.** Zero runtime network calls. `git push` to GitHub is the only network edge, protected by SSH/HTTPS plus GitHub's own controls. The binary does not fetch, call home, or publish over the network.
- **Backup tampering across targets.** Source is Unlicense / public domain; every public clone is the de facto backup. There is no private backup corpus for an attacker to tamper with.
- **Insider privilege escalation.** No admin roles, no delete paths, no user accounts, no runtime state. The project is a single-repo static framework with a read-only validator.
- **Physical device seizure (framework-as-IP).** Source is public. Seizing the dev laptop does not expose anything that is not already mirrored to `github.com/cochranblock/provenance-docs`.
- **Clock manipulation of runtime records.** The binary writes no timestamped artifacts of its own — all timestamps come from git (`%ai`) or from the filesystem at document-edit time. Clock attacks collapse into "TOI date forgery" above.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [ ] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*

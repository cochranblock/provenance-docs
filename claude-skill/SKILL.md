---
name: provenance-docs
description: Document AI vs. human attribution on software commits. Use when the user is committing AI-assisted code in a repository that contains TIMELINE_OF_INVENTION.md and PROOF_OF_ARTIFACTS.md, when the user mentions "TOI", "POA", "AI Role", "provenance", "f30", or asks to validate / generate provenance documentation, and when the user is preparing federal deliverables (CDRL, SBIR, DFARS 252.227-7014, EO 14028) for AI-piloted code. Drives the local `provenance-docs` Rust binary — fully offline, no network calls.
---

# Provenance Docs

Maintain commit-integrated AI/human attribution records (Timeline of Invention + Proof of Artifacts) using the local `provenance-docs` Rust binary.

## When this skill applies

A repository is "provenance-tracked" if it contains both `TIMELINE_OF_INVENTION.md` and `PROOF_OF_ARTIFACTS.md` at the workspace root. Check with:

```bash
test -f TIMELINE_OF_INVENTION.md && test -f PROOF_OF_ARTIFACTS.md && echo tracked
```

If the repo is not tracked, do nothing unless the user explicitly asks to initialize it.

## Tools

The skill drives a single binary. It must already be installed on the user's `PATH`:

```bash
command -v provenance-docs || cargo install --git https://github.com/cochranblock/provenance-docs
```

All commands below are pure local — no network, no cloud, no telemetry.

## Workflows

### 1. After a commit (auto-stub TOI + POA)

When the user has just made a commit (or asks to "document the last commit"), run:

```bash
provenance-docs generate-toi
```

This appends a stub entry to `TIMELINE_OF_INVENTION.md` and a row to the `PROOF_OF_ARTIFACTS.md` Commit Log. The stub has `TODO` placeholders for **What**, **Why**, **AI Role**, and **Proof**.

After generating the stub:

1. Read the diff for the commit: `git show --stat HEAD` and `git show HEAD -- <key files>`.
2. Edit the TOI entry in place. Replace the four `TODO` fields:
   - **What:** the concrete deliverable (binary, feature, fix). One sentence.
   - **Why:** the business or technical reason. One sentence.
   - **AI Role:** *required dual attribution.* Must mention both the AI's contribution and the human's direction/verification. The validator rejects entries that don't contain both "ai" and "human" in this field.
   - **Proof:** path to a file, test output, or screenshot proving the work.
3. Do **not** edit the **Commit** field — the binary fills it from `git log -1`.

### 2. Drafting the AI Role field

This is the field that defines the framework. Write it the way the user would: lead with what the AI generated, then what the human directed and verified. Examples in the repo's existing TOI are the style reference — match them.

**Pattern:** `AI generated <X>. Human directed <Y> and verified <Z>.`

Avoid vague phrasing ("AI helped", "human reviewed"). Be specific about the boundary.

### 3. Validating the docs (f30)

Before the user commits docs, ships a release, or prepares a federal deliverable, run the validator:

```bash
provenance-docs
```

Exit 0 = all checks passed. Exit 1 = at least one failure. The output lists each check (`OK` or `FAIL`). Common failures and fixes:

| Failure | Fix |
| --- | --- |
| `TOI missing field **AI Role:**` | Add the field to the offending entry. |
| `AI Role entry at line N missing human or AI attribution` | Rewrite that entry's AI Role to mention both. |
| `TOI date order: A before B` | Move the out-of-order entry. TOI is reverse-chronological. |
| `POA hash X not found in TOI` | Either add a TOI entry for that commit or remove the POA Commit Log row. |
| `Git commit X missing from POA Commit Log` | Run `provenance-docs generate-toi` after checking out that commit, or add the row by hand. |
| `TOI hash X not found in git history` | The hash was deleted (force-push or branch reset). Replace with the current hash. |

Re-run after each fix until exit 0.

### 4. TRIPLE SIMS quality gate

For releases and federal deliverables, run the test binary (when the repo provides one):

```bash
cargo run --bin provenance-docs-test --features tests
```

This runs the f30 gate three times. All three passes must succeed. A single deviation fails the gate — this catches flaky validations and non-determinism. Use this as the final ship-readiness check.

### 5. Initializing a new repo

If the user wants to add Provenance Docs to a repository that does not yet have TOI/POA, scaffold them by hand (the binary doesn't yet do this — keep it simple):

1. Copy the structure from this repo's `TIMELINE_OF_INVENTION.md` and `PROOF_OF_ARTIFACTS.md`. Keep the section headers (`## Entries`, `## Commit Log`, `## Architecture`, `## Build Output`, `## Validation`, `## Screenshots`, `## How to Verify`) — the validator looks for them by string match.
2. Empty out the existing entries and rows. Leave the headers.
3. Run `provenance-docs generate-toi` once to populate the first entry from the latest commit.
4. Fill in the four TODO fields.
5. Run `provenance-docs` to confirm the validator passes.

## Constraints

- **Offline only.** No tool in this skill makes a network call. If the user proposes a workflow that requires uploading TOI/POA somewhere, push back: the framework is designed to live in the repo.
- **Don't fabricate AI Role content.** If you don't know what the AI did vs. what the human directed for a given commit, ask. Made-up attribution defeats the entire purpose of the framework.
- **Don't edit the Commit field.** It's git-derived. If it looks wrong, the fix is to re-run `generate-toi`, not to type a hash by hand.
- **Don't reorder TOI entries to silence date-order failures.** The failure usually means a commit was backdated or a TOI entry was edited later than its date. Investigate first.
- **Federal deliverables are sensitive.** When working on CDRL/SBIR/DFARS material, default to the user reviewing every change before commit. Don't auto-commit doc changes in this mode.

## Reference

- `WHITEPAPER.md` — full framework spec, CDRL mapping, SBIR phase plan.
- `govdocs/CDRL_MAPPING.md` — DI-IPSC-81435 / 81438 / 81441 / DI-MGMT-81466 mapping.
- `BACKLOG.md` — open work items on the framework itself.
- Source: <https://github.com/cochranblock/provenance-docs>.

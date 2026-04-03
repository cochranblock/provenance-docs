# Provenance Docs

A commit-integrated documentation framework for AI-piloted software development. Designed for federal acquisition, deployed across 16 production repositories.

**License:** [Unlicense](UNLICENSE)

## What's Here

- [**WHITEPAPER.md**](WHITEPAPER.md) — full framework specification, CDRL mapping, SBIR phase plan
- [**TIMELINE_OF_INVENTION.md**](TIMELINE_OF_INVENTION.md) — dated, commit-linked record with human/AI attribution on every entry
- [**PROOF_OF_ARTIFACTS.md**](PROOF_OF_ARTIFACTS.md) — architecture, build output, validation metrics, screenshots, verification commands
- [**govdocs/**](govdocs/) — supply chain audit (EO 14028, NIST SP 800-218)
- **src/** — Rust validator binary (`f30`) and TRIPLE SIMS test gate

## Quick Start

```bash
# Validate all docs (13 structural checks + hash/date/cross-doc validation)
cargo run

# TRIPLE SIMS quality gate (runs validation 3x, fails on first deviation)
cargo run --bin provenance-docs-test --features tests
```

## Links

- [cochranblock.org](https://cochranblock.org) — zero-cloud architecture, 16 Unlicense repos
- [All products](https://cochranblock.org/products)
- [GitHub](https://github.com/cochranblock)

---

Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. [See all products →](https://cochranblock.org/products)

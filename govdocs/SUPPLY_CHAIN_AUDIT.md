<!-- Unlicense — cochranblock.org -->

# Supply Chain Security Audit

*EO 14028 compliance — provenance-docs dependency audit*

**Audit date:** 2026-03-30
**Auditor:** Kova + Claude Opus 4.6
**Rust toolchain:** 1.94.0 (2026-03-02)
**Total dependencies:** 14 crates (with `tests` feature)

---

## Summary

| Check | Result | Notes |
|-------|--------|-------|
| `cargo audit` (CVEs) | **PASS** | 0 advisories in 1017 database |
| `cargo deny` (licenses) | **PASS** | Fixed: added `license = "Unlicense"` to Cargo.toml |
| `cargo deny` (bans) | **PASS** | No banned crates |
| `cargo deny` (sources) | **PASS** | All from crates.io or local path |
| `cargo tree --duplicates` | **PASS** | 0 duplicate crate versions |
| `cargo outdated` | **PASS** | All deps at latest version |
| `cargo geiger` (unsafe) | **PASS** | 0 unsafe in application code |
| Cargo.lock committed | **PASS** | Binary project, lock file tracked |
| Yanked crates | **PASS** | 0 yanked versions in lock file |
| Large files in git | **PASS** | 0 files > 1MB outside target/ |
| Junk files | **PASS** | 0 .DS_Store, .swp, .bak, etc. |
| Typosquatting | **PASS** | All dep names are canonical |

---

## Dependency Tree

```
provenance-docs v0.1.0 (0 unsafe)
├── nix v0.31.2 (signal + process features only)
│   ├── bitflags v2.11.0
│   ├── cfg-if v1.0.4
│   ├── cfg_aliases v0.2.1
│   └── libc v0.2.183
├── exopack v0.1.0 [optional, tests] (0 unsafe)
└── tokio v1.50.0 [optional, tests]
    ├── pin-project-lite v0.2.17
    └── tokio-macros v2.6.1
        ├── proc-macro2 v1.0.106
        │   └── unicode-ident v1.0.24
        ├── quote v1.0.45
        └── syn v2.0.117
```

## Cargo Geiger — Unsafe Usage

| Crate | Unsafe Functions | Unsafe Expressions | Risk |
|-------|-----------------|-------------------|------|
| provenance-docs 0.1.0 | 0/0 | 0/0 | None |
| exopack 0.1.0 | 0/0 | 0/0 | None |
| tokio 1.50.0 | 25/30 | 1051/2909 | Expected — async runtime |
| syn 2.0.117 | 0/0 | 87/87 | Expected — parser speed |
| proc-macro2 1.0.106 | 0/0 | 14/14 | Expected — compiler FFI |
| pin-project-lite 0.2.17 | 0/0 | 11/191 | Expected — Pin projections |
| unicode-ident 1.0.24 | 0/0 | 4/4 | Expected — lookup tables |
| quote 1.0.45 | 0/0 | 0/0 | None |
| tokio-macros 2.6.1 | 0/0 | 0/0 | None |

**Application + custom code:** 0 unsafe blocks.
**Total across all deps:** 25 used unsafe functions, 1167 used unsafe expressions.
**All unsafe is in well-audited infrastructure crates (tokio, syn, proc-macro2).**

---

## Deep Code Review

### exopack (custom crate — highest scrutiny)

| Check | Finding |
|-------|---------|
| Unsafe blocks | 0 |
| `std::process::Command` | Yes — runs `cargo test` and `cargo build` (intentional, expected) |
| Network calls | None when `triple_sims` feature only |
| Environment variables | Reads `CARGO_MANIFEST_DIR` (compile-time), sets `TEST_DEMO=1` for live demo |
| File writes | Temp dirs in `/tmp` with pid-based names for test isolation |
| File reads | `Cargo.toml` for test binary discovery (`f63`) |
| Secret exfiltration | None |
| Telemetry / phoning home | None |

**Verdict:** Clean. Command execution is limited to `cargo` invocations for the test runner, which is the crate's entire purpose.

### tokio (async runtime)

| Check | Finding |
|-------|---------|
| Features enabled | `rt`, `macros` only |
| Network capability | **Disabled** — `net` feature not enabled |
| File I/O capability | **Disabled** — `fs` feature not enabled |
| Process spawning | **Disabled** — `process` feature not enabled |
| Environment variables | `TOKIO_WORKER_THREADS` — only with `rt-multi-thread` (not enabled) |
| Unsafe justification | Thread-local storage, atomics, platform abstractions |

**Verdict:** Minimal feature surface. No network, no filesystem, no process spawning in the enabled feature set.

### nix (POSIX signal handling)

| Check | Finding |
|-------|---------|
| Features enabled | `signal`, `process` only |
| Unsafe blocks | Yes — FFI to libc (expected for POSIX bindings) |
| Network capability | None in enabled features |
| File I/O | None in enabled features |
| Purpose | `kill()` for PID-based hot reload handoff |

**Verdict:** Minimal feature surface. Only used for SIGTERM/SIGKILL delivery.

### proc-macro2, quote, syn, unicode-ident, tokio-macros (compile-time only)

These are proc-macro crates. They execute at compile time, not runtime. They generate code but have no runtime behavior, no network access, no file I/O, and no environment variable reads.

All are maintained by dtolnay (proc-macro2, quote, syn, unicode-ident) or the tokio project (tokio-macros). These are among the most downloaded and audited crates in the Rust ecosystem.

**Verdict:** No runtime risk. Compile-time only.

---

## Typosquatting Analysis

| Dep Name | Canonical? | Similar Names on crates.io | Risk |
|----------|-----------|---------------------------|------|
| tokio | Yes | tokio-rs (different project scope) | None |
| syn | Yes | — | None |
| quote | Yes | — | None |
| proc-macro2 | Yes | — | None |
| unicode-ident | Yes | — | None |
| pin-project-lite | Yes | pin-project (full version) | None — both by same author |
| tokio-macros | Yes | — | None |
| nix | Yes | — | None |
| exopack | Local path | — | None — not from registry |

---

## License Compliance

| Crate | License | Unlicense Compatible |
|-------|---------|---------------------|
| provenance-docs | Unlicense | (self) |
| exopack | Unlicense | Yes |
| tokio | MIT | Yes |
| tokio-macros | MIT | Yes |
| syn | MIT OR Apache-2.0 | Yes |
| quote | MIT OR Apache-2.0 | Yes |
| proc-macro2 | MIT OR Apache-2.0 | Yes |
| unicode-ident | (MIT OR Apache-2.0) AND Unicode-3.0 | Yes |
| pin-project-lite | MIT OR Apache-2.0 | Yes |
| nix | MIT | Yes |
| libc | MIT OR Apache-2.0 | Yes |
| bitflags | MIT OR Apache-2.0 | Yes |
| cfg-if | MIT OR Apache-2.0 | Yes |
| cfg_aliases | MIT | Yes |

**No copyleft (GPL/LGPL/AGPL) licenses. All deps are permissive. Safe for Unlicense project.**

---

## Recommended Actions

1. ~~Add `license = "Unlicense"` to Cargo.toml~~ **DONE**
2. ~~Expand .gitignore with .DS_Store, .env, *.log, etc.~~ **DONE**
3. No dependency upgrades needed — all at latest
4. No CVE remediations needed — 0 advisories
5. Re-run this audit quarterly or on any dependency change

---

*Audit conducted per EO 14028 (Improving the Nation's Cybersecurity) and NIST SP 800-218 (Secure Software Development Framework).*
*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*

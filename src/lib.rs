// Unlicense — cochranblock.org
//! provenance_docs library. f30=run_tests: validate TOI+POA exist and contain required fields.

use std::path::Path;

/// f30=run_tests. Returns 0 on success, 1 on failure.
pub fn f30() -> i32 {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut failures = 0;

    // Stage 1: required docs exist
    for name in ["TIMELINE_OF_INVENTION.md", "PROOF_OF_ARTIFACTS.md", "WHITEPAPER.md"] {
        let p = root.join(name);
        if p.exists() {
            println!("  OK  {name}");
        } else {
            eprintln!("  FAIL  {name} missing");
            failures += 1;
        }
    }

    // Stage 2: TOI contains required fields
    let toi = std::fs::read_to_string(root.join("TIMELINE_OF_INVENTION.md")).unwrap_or_default();
    for field in ["**What:**", "**Why:**", "**Commit:**", "**AI Role:**", "**Proof:**"] {
        if toi.contains(field) {
            println!("  OK  TOI field {field}");
        } else {
            eprintln!("  FAIL  TOI missing field {field}");
            failures += 1;
        }
    }

    // Stage 3: POA contains required sections
    let poa = std::fs::read_to_string(root.join("PROOF_OF_ARTIFACTS.md")).unwrap_or_default();
    for section in ["## Architecture", "## Validation", "## How to Verify"] {
        if poa.contains(section) {
            println!("  OK  POA section {section}");
        } else {
            eprintln!("  FAIL  POA missing section {section}");
            failures += 1;
        }
    }

    if failures == 0 {
        println!("All checks passed");
    } else {
        eprintln!("{failures} check(s) failed");
    }
    failures
}

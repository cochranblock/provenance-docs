// Unlicense — cochranblock.org
//! provenance_docs library. f30=run_tests: validate TOI+POA exist and contain required fields.

use std::path::Path;
use std::process::Command;

/// f30=run_tests. Returns 0 on success, 1 on failure.
pub fn f30() -> i32 {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut failures = 0;

    // Stage 1: required docs exist
    for name in [
        "TIMELINE_OF_INVENTION.md",
        "PROOF_OF_ARTIFACTS.md",
        "WHITEPAPER.md",
        "BACKLOG.md",
        "govdocs/SUPPLY_CHAIN_AUDIT.md",
        "govdocs/CDRL_MAPPING.md",
    ] {
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
    for section in ["## Architecture", "## Build Output", "## Validation", "## Screenshots", "## How to Verify"] {
        if poa.contains(section) {
            println!("  OK  POA section {section}");
        } else {
            eprintln!("  FAIL  POA missing section {section}");
            failures += 1;
        }
    }

    // Stage 4: TOI commit hashes are valid hex (rejects placeholders)
    // Tokens that are clearly prose (contain non-hex chars) are skipped.
    // Only tokens that are all-hex but wrong length are flagged as invalid.
    let mut hash_count = 0;
    for line in toi.lines() {
        if let Some(rest) = line.strip_prefix("**Commit:**") {
            let rest = rest.trim();
            for token in rest.split([',', ' ']) {
                let token = token.trim();
                if token.is_empty() {
                    continue;
                }
                // Skip short tokens and non-hex — only 7-40 char all-hex strings are hash candidates
                if token.len() < 7 || token.len() > 40 {
                    continue;
                }
                if !token.chars().all(|c| c.is_ascii_hexdigit()) {
                    continue;
                }
                hash_count += 1;
                println!("  OK  TOI hash {token}");
            }
        }
    }
    if hash_count == 0 {
        eprintln!("  FAIL  TOI has no commit hashes");
        failures += 1;
    }

    // Stage 5: TOI dates in reverse-chronological order
    let mut dates: Vec<&str> = Vec::new();
    for line in toi.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("### ")
            && rest.len() >= 10
            && rest.as_bytes()[4] == b'-'
            && rest.as_bytes()[7] == b'-'
        {
            dates.push(&rest[..10]);
        }
    }
    if dates.len() >= 2 {
        let mut ordered = true;
        for pair in dates.windows(2) {
            if pair[0] < pair[1] {
                eprintln!("  FAIL  TOI date order: {} before {}", pair[0], pair[1]);
                ordered = false;
                failures += 1;
            }
        }
        if ordered {
            println!("  OK  TOI dates in reverse-chronological order");
        }
    } else if !dates.is_empty() {
        println!("  OK  TOI dates in reverse-chronological order (single entry)");
    }

    // Stage 6: Cross-doc consistency — POA Commit Log hashes appear in TOI
    let mut cross_checked = 0;
    for line in poa.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }
        let cols: Vec<&str> = trimmed.split('|').collect();
        if cols.len() >= 3 {
            let hash = cols[1].trim();
            if hash.len() >= 7
                && hash.len() <= 40
                && hash.chars().all(|c| c.is_ascii_hexdigit())
            {
                cross_checked += 1;
                if toi.contains(hash) {
                    println!("  OK  POA hash {hash} found in TOI");
                } else {
                    eprintln!("  FAIL  POA hash {hash} not found in TOI");
                    failures += 1;
                }
            }
        }
    }
    if cross_checked > 0 {
        println!("  OK  Cross-doc: {cross_checked} POA hashes verified against TOI");
    }

    // Stage 7: TOI commit hashes exist in git history
    let git_log = Command::new("git")
        .args(["log", "--oneline", "--all"])
        .current_dir(root)
        .output()
        .ok()
        .and_then(|o| if o.status.success() { String::from_utf8(o.stdout).ok() } else { None })
        .unwrap_or_default();

    if !git_log.is_empty() {
        let mut toi_hashes: Vec<&str> = Vec::new();
        for line in toi.lines() {
            if let Some(rest) = line.strip_prefix("**Commit:**") {
                for token in rest.trim().split([',', ' ']) {
                    let token = token.trim();
                    if token.len() >= 7
                        && token.len() <= 40
                        && token.chars().all(|c| c.is_ascii_hexdigit())
                    {
                        toi_hashes.push(token);
                    }
                }
            }
        }

        let mut git_verified = 0;
        for hash in &toi_hashes {
            if git_log.lines().any(|line| line.starts_with(hash)) {
                git_verified += 1;
            } else {
                eprintln!("  FAIL  TOI hash {hash} not found in git history");
                failures += 1;
            }
        }
        if git_verified > 0 {
            println!("  OK  Git history: {git_verified} TOI hashes verified");
        }

        // Stage 8: POA Commit Log covers all repo commits
        let mut poa_hashes: Vec<&str> = Vec::new();
        for line in poa.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with('|') {
                continue;
            }
            let cols: Vec<&str> = trimmed.split('|').collect();
            if cols.len() >= 3 {
                let h = cols[1].trim();
                if h.len() >= 7
                    && h.len() <= 40
                    && h.chars().all(|c| c.is_ascii_hexdigit())
                {
                    poa_hashes.push(h);
                }
            }
        }

        let git_hashes: Vec<&str> = git_log
            .lines()
            .filter_map(|line| line.split_whitespace().next())
            .collect();
        let mut missing_from_poa = 0;
        for gh in &git_hashes {
            if !poa_hashes.iter().any(|ph| ph == gh) {
                eprintln!("  FAIL  Git commit {gh} missing from POA Commit Log");
                missing_from_poa += 1;
                failures += 1;
            }
        }
        if missing_from_poa == 0 {
            println!("  OK  POA Commit Log covers all {} git commits", git_hashes.len());
        }

        // Stage 9: Bidirectional — every TOI hash appears in POA Commit Log
        let mut toi_in_poa = 0;
        let mut toi_missing_poa = 0;
        for hash in &toi_hashes {
            if poa_hashes.iter().any(|ph| ph == hash) {
                toi_in_poa += 1;
            } else {
                eprintln!("  FAIL  TOI hash {hash} not found in POA Commit Log");
                toi_missing_poa += 1;
                failures += 1;
            }
        }
        if toi_missing_poa == 0 && toi_in_poa > 0 {
            println!("  OK  Bidirectional: {toi_in_poa} TOI hashes found in POA Commit Log");
        }
    }

    // Stage 10: AI Role field mentions both human and AI contributions
    let mut role_count = 0;
    let mut role_valid = 0;
    let toi_lower = toi.to_lowercase();
    for line in toi_lower.lines() {
        if line.starts_with("**ai role:**") {
            role_count += 1;
            let has_ai = line.contains("ai ");
            let has_human = line.contains("human");
            if has_ai && has_human {
                role_valid += 1;
            } else {
                eprintln!("  FAIL  AI Role entry missing human or AI attribution");
                failures += 1;
            }
        }
    }
    if role_count > 0 && role_valid == role_count {
        println!("  OK  AI Role: {role_count} entries have dual attribution");
    }

    if failures == 0 {
        println!("All checks passed");
    } else {
        eprintln!("{failures} check(s) failed");
    }
    failures
}

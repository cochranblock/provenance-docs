// Unlicense — cochranblock.org
//! provenance_docs library. f30=run_tests: validate TOI+POA exist and contain required fields.

use std::path::Path;
use std::process::Command;

/// Extract commit hashes from TOI `**Commit:**` lines.
/// Returns only tokens that are 7-40 char all-hex (valid git short/full hashes).
/// Skips prose words and short numeric tokens.
pub fn extract_toi_hashes(toi: &str) -> Vec<&str> {
    let mut hashes = Vec::new();
    for line in toi.lines() {
        if let Some(rest) = line.strip_prefix("**Commit:**") {
            for token in rest.trim().split([',', ' ']) {
                let token = token.trim();
                if token.len() >= 7
                    && token.len() <= 40
                    && token.chars().all(|c| c.is_ascii_hexdigit())
                {
                    hashes.push(token);
                }
            }
        }
    }
    hashes
}

/// Extract commit hashes from POA Commit Log table rows.
/// Looks for `| <hash> | ...` patterns where hash is 7-40 char hex.
pub fn extract_poa_hashes(poa: &str) -> Vec<&str> {
    let mut hashes = Vec::new();
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
                hashes.push(h);
            }
        }
    }
    hashes
}

/// Extract dates from TOI `### YYYY-MM-DD` headers.
/// Returns dates as string slices in document order.
pub fn extract_toi_dates(toi: &str) -> Vec<&str> {
    let mut dates = Vec::new();
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
    dates
}

/// Check if dates are in reverse-chronological (non-increasing) order.
/// Returns list of (earlier_date, later_date) pairs that violate ordering.
pub fn check_date_order<'a>(dates: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    let mut violations = Vec::new();
    for pair in dates.windows(2) {
        if pair[0] < pair[1] {
            violations.push((pair[0], pair[1]));
        }
    }
    violations
}

/// Check that a document contains all required fields/sections.
/// Returns list of missing items.
pub fn check_required_fields<'a>(content: &str, required: &[&'a str]) -> Vec<&'a str> {
    required
        .iter()
        .filter(|field| !content.contains(**field))
        .copied()
        .collect()
}

/// Validate AI Role fields mention both human and AI.
/// Returns (total_entries, valid_entries, invalid_line_numbers).
pub fn validate_ai_roles(toi: &str) -> (usize, usize, Vec<usize>) {
    let mut total = 0;
    let mut valid = 0;
    let mut invalid_lines = Vec::new();
    let lower = toi.to_lowercase();
    for (i, line) in lower.lines().enumerate() {
        if line.starts_with("**ai role:**") {
            total += 1;
            let rest = &line["**ai role:**".len()..];
            let has_ai = rest.contains("ai ");
            let has_human = rest.contains("human");
            if has_ai && has_human {
                valid += 1;
            } else {
                invalid_lines.push(i + 1);
            }
        }
    }
    (total, valid, invalid_lines)
}

/// Check POA→TOI cross-reference: every POA hash should appear in TOI.
/// Returns list of POA hashes not found in TOI.
pub fn check_poa_in_toi<'a>(poa_hashes: &[&'a str], toi: &str) -> Vec<&'a str> {
    poa_hashes
        .iter()
        .filter(|h| !toi.contains(**h))
        .copied()
        .collect()
}

/// Check TOI→POA cross-reference: every TOI hash should appear in POA Commit Log.
/// Returns list of TOI hashes not found in POA.
pub fn check_toi_in_poa<'a>(toi_hashes: &[&'a str], poa_hashes: &[&str]) -> Vec<&'a str> {
    toi_hashes
        .iter()
        .filter(|h| !poa_hashes.contains(h))
        .copied()
        .collect()
}

/// Check that all git commits appear in POA Commit Log.
/// Returns list of git hashes not found in POA.
pub fn check_git_coverage<'a>(git_hashes: &[&'a str], poa_hashes: &[&str]) -> Vec<&'a str> {
    git_hashes
        .iter()
        .filter(|h| !poa_hashes.contains(h))
        .copied()
        .collect()
}

/// Check that all TOI hashes exist in git history.
/// Returns list of TOI hashes not found in git log.
pub fn check_hashes_in_git<'a>(toi_hashes: &[&'a str], git_log: &str) -> Vec<&'a str> {
    toi_hashes
        .iter()
        .filter(|h| !git_log.lines().any(|line| line.starts_with(**h)))
        .copied()
        .collect()
}

/// Parse git log --oneline output into a list of short hashes.
pub fn parse_git_hashes(git_log: &str) -> Vec<&str> {
    git_log
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect()
}

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
    let toi_fields = ["**What:**", "**Why:**", "**Commit:**", "**AI Role:**", "**Proof:**"];
    let missing = check_required_fields(&toi, &toi_fields);
    for field in &toi_fields {
        if missing.contains(field) {
            eprintln!("  FAIL  TOI missing field {field}");
            failures += 1;
        } else {
            println!("  OK  TOI field {field}");
        }
    }

    // Stage 3: POA contains required sections
    let poa = std::fs::read_to_string(root.join("PROOF_OF_ARTIFACTS.md")).unwrap_or_default();
    let poa_sections = ["## Architecture", "## Build Output", "## Validation", "## Screenshots", "## How to Verify"];
    let missing = check_required_fields(&poa, &poa_sections);
    for section in &poa_sections {
        if missing.contains(section) {
            eprintln!("  FAIL  POA missing section {section}");
            failures += 1;
        } else {
            println!("  OK  POA section {section}");
        }
    }

    // Stage 4: TOI commit hashes are valid hex
    let toi_hashes = extract_toi_hashes(&toi);
    if toi_hashes.is_empty() {
        eprintln!("  FAIL  TOI has no commit hashes");
        failures += 1;
    } else {
        for hash in &toi_hashes {
            println!("  OK  TOI hash {hash}");
        }
    }

    // Stage 5: TOI dates in reverse-chronological order
    let dates = extract_toi_dates(&toi);
    let violations = check_date_order(&dates);
    if !violations.is_empty() {
        for (a, b) in &violations {
            eprintln!("  FAIL  TOI date order: {a} before {b}");
            failures += 1;
        }
    } else if !dates.is_empty() {
        println!("  OK  TOI dates in reverse-chronological order");
    }

    // Stage 6: Cross-doc consistency — POA hashes appear in TOI
    let poa_hashes = extract_poa_hashes(&poa);
    let poa_missing = check_poa_in_toi(&poa_hashes, &toi);
    for hash in &poa_hashes {
        if poa_missing.contains(hash) {
            eprintln!("  FAIL  POA hash {hash} not found in TOI");
            failures += 1;
        } else {
            println!("  OK  POA hash {hash} found in TOI");
        }
    }
    if !poa_hashes.is_empty() && poa_missing.is_empty() {
        println!("  OK  Cross-doc: {} POA hashes verified against TOI", poa_hashes.len());
    }

    // Stage 7-9: Git-dependent checks
    let git_log = Command::new("git")
        .args(["log", "--oneline"])
        .current_dir(root)
        .output()
        .ok()
        .and_then(|o| if o.status.success() { String::from_utf8(o.stdout).ok() } else { None })
        .unwrap_or_default();

    if !git_log.is_empty() {
        // Stage 7: TOI hashes exist in git
        let not_in_git = check_hashes_in_git(&toi_hashes, &git_log);
        for h in &not_in_git {
            eprintln!("  FAIL  TOI hash {h} not found in git history");
            failures += 1;
        }
        let verified = toi_hashes.len() - not_in_git.len();
        if verified > 0 {
            println!("  OK  Git history: {verified} TOI hashes verified");
        }

        // Stage 8: POA covers all git commits
        let git_hashes = parse_git_hashes(&git_log);
        let missing_poa = check_git_coverage(&git_hashes, &poa_hashes);
        for h in &missing_poa {
            eprintln!("  FAIL  Git commit {h} missing from POA Commit Log");
            failures += 1;
        }
        if missing_poa.is_empty() {
            println!("  OK  POA Commit Log covers all {} git commits", git_hashes.len());
        }

        // Stage 9: Bidirectional — TOI hashes in POA
        let toi_not_in_poa = check_toi_in_poa(&toi_hashes, &poa_hashes);
        for h in &toi_not_in_poa {
            eprintln!("  FAIL  TOI hash {h} not found in POA Commit Log");
            failures += 1;
        }
        if toi_not_in_poa.is_empty() && !toi_hashes.is_empty() {
            println!("  OK  Bidirectional: {} TOI hashes found in POA Commit Log", toi_hashes.len());
        }
    }

    // Stage 10: AI Role dual attribution
    let (total, valid, invalid) = validate_ai_roles(&toi);
    if !invalid.is_empty() {
        for ln in &invalid {
            eprintln!("  FAIL  AI Role entry at line {ln} missing human or AI attribution");
        }
        failures += invalid.len() as i32;
    }
    if total > 0 && valid == total {
        println!("  OK  AI Role: {total} entries have dual attribution");
    }

    if failures == 0 {
        println!("All checks passed");
    } else {
        eprintln!("{failures} check(s) failed");
    }
    failures
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // extract_toi_hashes
    // =========================================================================

    #[test]
    fn extract_hashes_single_line() {
        let toi = "**Commit:** abc1234";
        assert_eq!(extract_toi_hashes(toi), vec!["abc1234"]);
    }

    #[test]
    fn extract_hashes_multiple_comma_separated() {
        let toi = "**Commit:** abc1234, def5678";
        assert_eq!(extract_toi_hashes(toi), vec!["abc1234", "def5678"]);
    }

    #[test]
    fn extract_hashes_multiple_entries() {
        let toi = "**Commit:** abc1234\nsome text\n**Commit:** def5678";
        assert_eq!(extract_toi_hashes(toi), vec!["abc1234", "def5678"]);
    }

    #[test]
    fn extract_hashes_skips_prose() {
        let toi = "**Commit:** Various across all 12 repos";
        assert_eq!(extract_toi_hashes(toi), Vec::<&str>::new());
    }

    #[test]
    fn extract_hashes_skips_short_hex() {
        // "12" is all hex but only 2 chars — not a hash
        let toi = "**Commit:** 12";
        assert_eq!(extract_toi_hashes(toi), Vec::<&str>::new());
    }

    #[test]
    fn extract_hashes_skips_6_char_hex() {
        let toi = "**Commit:** abcdef";
        assert_eq!(extract_toi_hashes(toi), Vec::<&str>::new());
    }

    #[test]
    fn extract_hashes_accepts_7_char() {
        let toi = "**Commit:** abcdef0";
        assert_eq!(extract_toi_hashes(toi), vec!["abcdef0"]);
    }

    #[test]
    fn extract_hashes_accepts_40_char_full_sha() {
        let hash = "a".repeat(40);
        let toi = format!("**Commit:** {hash}");
        assert_eq!(extract_toi_hashes(&toi), vec![hash.as_str()]);
    }

    #[test]
    fn extract_hashes_rejects_41_char() {
        let hash = "a".repeat(41);
        let toi = format!("**Commit:** {hash}");
        assert_eq!(extract_toi_hashes(&toi), Vec::<&str>::new());
    }

    #[test]
    fn extract_hashes_mixed_valid_and_prose() {
        let toi = "**Commit:** abc1234, See git log";
        assert_eq!(extract_toi_hashes(toi), vec!["abc1234"]);
    }

    #[test]
    fn extract_hashes_empty_commit_line() {
        let toi = "**Commit:**";
        assert_eq!(extract_toi_hashes(toi), Vec::<&str>::new());
    }

    #[test]
    fn extract_hashes_no_commit_lines() {
        let toi = "Some random text\nNo commit here";
        assert_eq!(extract_toi_hashes(toi), Vec::<&str>::new());
    }

    #[test]
    fn extract_hashes_whitespace_only_after_prefix() {
        let toi = "**Commit:**   ";
        assert_eq!(extract_toi_hashes(toi), Vec::<&str>::new());
    }

    #[test]
    fn extract_hashes_uppercase_hex() {
        let toi = "**Commit:** ABCDEF0";
        assert_eq!(extract_toi_hashes(toi), vec!["ABCDEF0"]);
    }

    #[test]
    fn extract_hashes_mixed_case_hex() {
        let toi = "**Commit:** aBcDeF0";
        assert_eq!(extract_toi_hashes(toi), vec!["aBcDeF0"]);
    }

    #[test]
    fn extract_hashes_non_hex_char_in_middle() {
        let toi = "**Commit:** abc1g34";
        assert_eq!(extract_toi_hashes(toi), Vec::<&str>::new());
    }

    // =========================================================================
    // extract_poa_hashes
    // =========================================================================

    #[test]
    fn poa_hashes_standard_table() {
        let poa = "| abc1234 | 2026-01-01 | Description |";
        assert_eq!(extract_poa_hashes(poa), vec!["abc1234"]);
    }

    #[test]
    fn poa_hashes_multiple_rows() {
        let poa = "| Hash | Date | Desc |\n|------|------|------|\n| abc1234 | 2026-01-01 | A |\n| def5678 | 2026-01-02 | B |";
        assert_eq!(extract_poa_hashes(poa), vec!["abc1234", "def5678"]);
    }

    #[test]
    fn poa_hashes_skips_header_row() {
        let poa = "| Hash | Date | Description |\n|------|------|------|";
        assert_eq!(extract_poa_hashes(poa), Vec::<&str>::new());
    }

    #[test]
    fn poa_hashes_skips_non_table_lines() {
        let poa = "Some text\n| abc1234 | date | desc |\nMore text";
        assert_eq!(extract_poa_hashes(poa), vec!["abc1234"]);
    }

    #[test]
    fn poa_hashes_empty_input() {
        assert_eq!(extract_poa_hashes(""), Vec::<&str>::new());
    }

    #[test]
    fn poa_hashes_pipe_but_not_hash() {
        let poa = "| not a hash | date | desc |";
        assert_eq!(extract_poa_hashes(poa), Vec::<&str>::new());
    }

    #[test]
    fn poa_hashes_short_hex_in_table() {
        let poa = "| abcdef | date | desc |";
        assert_eq!(extract_poa_hashes(poa), Vec::<&str>::new());
    }

    #[test]
    fn poa_hashes_with_leading_spaces() {
        let poa = "  | abc1234 | date | desc |";
        assert_eq!(extract_poa_hashes(poa), vec!["abc1234"]);
    }

    // =========================================================================
    // extract_toi_dates
    // =========================================================================

    #[test]
    fn dates_standard_format() {
        let toi = "### 2026-04-03 — Some title";
        assert_eq!(extract_toi_dates(toi), vec!["2026-04-03"]);
    }

    #[test]
    fn dates_multiple_entries() {
        let toi = "### 2026-04-03 — A\ntext\n### 2026-04-02 — B\ntext\n### 2026-03-30 — C";
        assert_eq!(extract_toi_dates(toi), vec!["2026-04-03", "2026-04-02", "2026-03-30"]);
    }

    #[test]
    fn dates_skip_non_date_headers() {
        let toi = "### Not a date\n### 2026-04-03 — Title";
        assert_eq!(extract_toi_dates(toi), vec!["2026-04-03"]);
    }

    #[test]
    fn dates_empty_input() {
        assert_eq!(extract_toi_dates(""), Vec::<&str>::new());
    }

    #[test]
    fn dates_h2_not_h3() {
        let toi = "## 2026-04-03 — Not h3";
        assert_eq!(extract_toi_dates(toi), Vec::<&str>::new());
    }

    #[test]
    fn dates_date_range_header() {
        // "### 2026-03-11 through 2026-03-26" — should extract first date
        let toi = "### 2026-03-11 through 2026-03-26 — Title";
        assert_eq!(extract_toi_dates(toi), vec!["2026-03-11"]);
    }

    #[test]
    fn dates_with_leading_whitespace() {
        let toi = "  ### 2026-04-03 — Title";
        assert_eq!(extract_toi_dates(toi), vec!["2026-04-03"]);
    }

    #[test]
    fn dates_short_line_after_prefix() {
        let toi = "### ab";
        assert_eq!(extract_toi_dates(toi), Vec::<&str>::new());
    }

    // =========================================================================
    // check_date_order
    // =========================================================================

    #[test]
    fn date_order_correct_descending() {
        let dates = vec!["2026-04-03", "2026-04-02", "2026-03-30"];
        assert_eq!(check_date_order(&dates), Vec::<(&str, &str)>::new());
    }

    #[test]
    fn date_order_equal_dates_ok() {
        let dates = vec!["2026-04-03", "2026-04-03", "2026-04-02"];
        assert_eq!(check_date_order(&dates), Vec::<(&str, &str)>::new());
    }

    #[test]
    fn date_order_ascending_violates() {
        let dates = vec!["2026-04-02", "2026-04-03"];
        assert_eq!(check_date_order(&dates), vec![("2026-04-02", "2026-04-03")]);
    }

    #[test]
    fn date_order_violation_in_middle() {
        let dates = vec!["2026-04-03", "2026-04-01", "2026-04-02"];
        assert_eq!(check_date_order(&dates), vec![("2026-04-01", "2026-04-02")]);
    }

    #[test]
    fn date_order_single_entry() {
        let dates = vec!["2026-04-03"];
        assert_eq!(check_date_order(&dates), Vec::<(&str, &str)>::new());
    }

    #[test]
    fn date_order_empty() {
        let dates: Vec<&str> = vec![];
        assert_eq!(check_date_order(&dates), Vec::<(&str, &str)>::new());
    }

    #[test]
    fn date_order_multiple_violations() {
        let dates = vec!["2026-01-01", "2026-02-01", "2026-03-01"];
        let v = check_date_order(&dates);
        assert_eq!(v.len(), 2);
    }

    // =========================================================================
    // check_required_fields
    // =========================================================================

    #[test]
    fn required_fields_all_present() {
        let content = "**What:** stuff\n**Why:** reason";
        let required = vec!["**What:**", "**Why:**"];
        assert_eq!(check_required_fields(content, &required), Vec::<&str>::new());
    }

    #[test]
    fn required_fields_one_missing() {
        let content = "**What:** stuff";
        let required = vec!["**What:**", "**Why:**"];
        assert_eq!(check_required_fields(content, &required), vec!["**Why:**"]);
    }

    #[test]
    fn required_fields_all_missing() {
        let content = "nothing here";
        let required = vec!["**What:**", "**Why:**"];
        assert_eq!(check_required_fields(content, &required), vec!["**What:**", "**Why:**"]);
    }

    #[test]
    fn required_fields_empty_content() {
        let required = vec!["**What:**"];
        assert_eq!(check_required_fields("", &required), vec!["**What:**"]);
    }

    #[test]
    fn required_fields_empty_requirements() {
        let required: Vec<&str> = vec![];
        assert_eq!(check_required_fields("anything", &required), Vec::<&str>::new());
    }

    #[test]
    fn required_fields_substring_match() {
        let content = "## Architecture Overview";
        let required = vec!["## Architecture"];
        assert_eq!(check_required_fields(content, &required), Vec::<&str>::new());
    }

    // =========================================================================
    // validate_ai_roles
    // =========================================================================

    #[test]
    fn ai_role_valid_entry() {
        let toi = "**AI Role:** AI drafted code. Human reviewed.";
        let (total, valid, invalid) = validate_ai_roles(toi);
        assert_eq!(total, 1);
        assert_eq!(valid, 1);
        assert!(invalid.is_empty());
    }

    #[test]
    fn ai_role_missing_human() {
        let toi = "**AI Role:** AI did everything.";
        let (total, valid, invalid) = validate_ai_roles(toi);
        assert_eq!(total, 1);
        assert_eq!(valid, 0);
        assert_eq!(invalid.len(), 1);
    }

    #[test]
    fn ai_role_missing_ai() {
        let toi = "**AI Role:** Human did it all by hand.";
        let (total, valid, invalid) = validate_ai_roles(toi);
        assert_eq!(total, 1);
        assert_eq!(valid, 0);
        assert_eq!(invalid.len(), 1);
    }

    #[test]
    fn ai_role_multiple_entries_all_valid() {
        let toi = "**AI Role:** AI wrote. Human reviewed.\n**AI Role:** AI generated. Human directed.";
        let (total, valid, invalid) = validate_ai_roles(toi);
        assert_eq!(total, 2);
        assert_eq!(valid, 2);
        assert!(invalid.is_empty());
    }

    #[test]
    fn ai_role_mixed_valid_invalid() {
        let toi = "**AI Role:** AI wrote. Human reviewed.\n**AI Role:** Only human work.";
        let (total, valid, invalid) = validate_ai_roles(toi);
        assert_eq!(total, 2);
        assert_eq!(valid, 1);
        assert_eq!(invalid.len(), 1);
    }

    #[test]
    fn ai_role_no_entries() {
        let toi = "No AI Role here.";
        let (total, valid, invalid) = validate_ai_roles(toi);
        assert_eq!(total, 0);
        assert_eq!(valid, 0);
        assert!(invalid.is_empty());
    }

    #[test]
    fn ai_role_case_insensitive() {
        let toi = "**AI Role:** ai assisted. Human approved.";
        let (total, valid, _) = validate_ai_roles(toi);
        assert_eq!(total, 1);
        assert_eq!(valid, 1);
    }

    #[test]
    fn ai_role_ai_word_boundary() {
        // "AI" must be followed by space (checked as "ai ")
        let toi = "**AI Role:** AI-generated code. Human verified.";
        let (total, valid, _) = validate_ai_roles(toi);
        assert_eq!(total, 1);
        // "ai-generated" doesn't contain "ai " — this should fail
        assert_eq!(valid, 0);
    }

    #[test]
    fn ai_role_reports_correct_line_numbers() {
        let toi = "line1\n**AI Role:** only human work\nline3\n**AI Role:** AI and Human";
        let (_, _, invalid) = validate_ai_roles(toi);
        assert_eq!(invalid, vec![2]); // line 2 is the invalid one
    }

    // =========================================================================
    // check_poa_in_toi
    // =========================================================================

    #[test]
    fn poa_in_toi_all_found() {
        let poa_hashes = vec!["abc1234", "def5678"];
        let toi = "**Commit:** abc1234\n**Commit:** def5678";
        assert_eq!(check_poa_in_toi(&poa_hashes, toi), Vec::<&str>::new());
    }

    #[test]
    fn poa_in_toi_one_missing() {
        let poa_hashes = vec!["abc1234", "def5678"];
        let toi = "**Commit:** abc1234";
        assert_eq!(check_poa_in_toi(&poa_hashes, toi), vec!["def5678"]);
    }

    #[test]
    fn poa_in_toi_empty_poa() {
        let poa_hashes: Vec<&str> = vec![];
        let toi = "**Commit:** abc1234";
        assert_eq!(check_poa_in_toi(&poa_hashes, toi), Vec::<&str>::new());
    }

    #[test]
    fn poa_in_toi_hash_in_prose() {
        // Hash appears in TOI text, not just Commit line
        let poa_hashes = vec!["abc1234"];
        let toi = "Some text mentioning abc1234 hash";
        assert_eq!(check_poa_in_toi(&poa_hashes, toi), Vec::<&str>::new());
    }

    // =========================================================================
    // check_toi_in_poa (bidirectional)
    // =========================================================================

    #[test]
    fn toi_in_poa_all_found() {
        let toi_hashes = vec!["abc1234", "def5678"];
        let poa_hashes = vec!["abc1234", "def5678", "ghi9012"];
        assert_eq!(check_toi_in_poa(&toi_hashes, &poa_hashes), Vec::<&str>::new());
    }

    #[test]
    fn toi_in_poa_one_missing() {
        let toi_hashes = vec!["abc1234", "def5678"];
        let poa_hashes = vec!["abc1234"];
        assert_eq!(check_toi_in_poa(&toi_hashes, &poa_hashes), vec!["def5678"]);
    }

    #[test]
    fn toi_in_poa_empty_toi() {
        let toi_hashes: Vec<&str> = vec![];
        let poa_hashes = vec!["abc1234"];
        assert_eq!(check_toi_in_poa(&toi_hashes, &poa_hashes), Vec::<&str>::new());
    }

    #[test]
    fn toi_in_poa_empty_both() {
        let toi_hashes: Vec<&str> = vec![];
        let poa_hashes: Vec<&str> = vec![];
        assert_eq!(check_toi_in_poa(&toi_hashes, &poa_hashes), Vec::<&str>::new());
    }

    // =========================================================================
    // check_git_coverage
    // =========================================================================

    #[test]
    fn git_coverage_all_covered() {
        let git = vec!["abc1234", "def5678"];
        let poa = vec!["abc1234", "def5678"];
        assert_eq!(check_git_coverage(&git, &poa), Vec::<&str>::new());
    }

    #[test]
    fn git_coverage_one_missing() {
        let git = vec!["abc1234", "def5678"];
        let poa = vec!["abc1234"];
        assert_eq!(check_git_coverage(&git, &poa), vec!["def5678"]);
    }

    #[test]
    fn git_coverage_empty_git() {
        let git: Vec<&str> = vec![];
        let poa = vec!["abc1234"];
        assert_eq!(check_git_coverage(&git, &poa), Vec::<&str>::new());
    }

    #[test]
    fn git_coverage_poa_superset() {
        let git = vec!["abc1234"];
        let poa = vec!["abc1234", "def5678", "ghi9012"];
        assert_eq!(check_git_coverage(&git, &poa), Vec::<&str>::new());
    }

    // =========================================================================
    // check_hashes_in_git
    // =========================================================================

    #[test]
    fn hashes_in_git_all_found() {
        let toi = vec!["abc1234", "def5678"];
        let git = "abc1234 some commit\ndef5678 another commit";
        assert_eq!(check_hashes_in_git(&toi, git), Vec::<&str>::new());
    }

    #[test]
    fn hashes_in_git_one_missing() {
        let toi = vec!["abc1234", "def5678"];
        let git = "abc1234 some commit\nghi9012 different";
        assert_eq!(check_hashes_in_git(&toi, git), vec!["def5678"]);
    }

    #[test]
    fn hashes_in_git_empty_log() {
        let toi = vec!["abc1234"];
        assert_eq!(check_hashes_in_git(&toi, ""), vec!["abc1234"]);
    }

    #[test]
    fn hashes_in_git_prefix_match() {
        // git log line starts with the hash
        let toi = vec!["abc1234"];
        let git = "abc1234567890 full hash commit";
        assert_eq!(check_hashes_in_git(&toi, git), Vec::<&str>::new());
    }

    #[test]
    fn hashes_in_git_no_match_in_middle() {
        // hash appears in middle of line, not at start — should not match
        let toi = vec!["abc1234"];
        let git = "def5678 commit with abc1234 in message";
        assert_eq!(check_hashes_in_git(&toi, git), vec!["abc1234"]);
    }

    // =========================================================================
    // parse_git_hashes
    // =========================================================================

    #[test]
    fn parse_git_standard() {
        let log = "abc1234 first commit\ndef5678 second commit";
        assert_eq!(parse_git_hashes(log), vec!["abc1234", "def5678"]);
    }

    #[test]
    fn parse_git_empty() {
        assert_eq!(parse_git_hashes(""), Vec::<&str>::new());
    }

    #[test]
    fn parse_git_single_line() {
        assert_eq!(parse_git_hashes("abc1234 only commit"), vec!["abc1234"]);
    }

    #[test]
    fn parse_git_trailing_newline() {
        let log = "abc1234 commit\n";
        assert_eq!(parse_git_hashes(log), vec!["abc1234"]);
    }

    // =========================================================================
    // Integration: real doc content patterns
    // =========================================================================

    #[test]
    fn integration_realistic_toi() {
        let toi = r#"# Timeline of Invention

## Entries

### 2026-04-03 — Feature A

**What:** Did something
**Why:** Needed it
**Commit:** abc1234, def5678
**AI Role:** AI drafted. Human directed and verified.
**Proof:** WHITEPAPER.md

### 2026-04-02 — Feature B

**What:** Did another thing
**Why:** Also needed
**Commit:** 1234567
**AI Role:** AI implemented. Human reviewed.
**Proof:** README.md

### 2026-03-26 — Initial

**What:** Setup
**Why:** Start
**Commit:** Various across all repos
**AI Role:** AI generated templates. Human reviewed all entries.
**Proof:** All repos
"#;
        // Hashes: should find abc1234, def5678, 1234567
        let hashes = extract_toi_hashes(toi);
        assert_eq!(hashes, vec!["abc1234", "def5678", "1234567"]);

        // Dates: reverse chronological
        let dates = extract_toi_dates(toi);
        assert_eq!(dates, vec!["2026-04-03", "2026-04-02", "2026-03-26"]);
        assert!(check_date_order(&dates).is_empty());

        // Fields
        let required = vec!["**What:**", "**Why:**", "**Commit:**", "**AI Role:**", "**Proof:**"];
        assert!(check_required_fields(toi, &required).is_empty());

        // AI Roles: all 3 should be valid
        let (total, valid, _) = validate_ai_roles(toi);
        assert_eq!(total, 3);
        assert_eq!(valid, 3);
    }

    #[test]
    fn integration_realistic_poa() {
        let poa = r#"# Proof of Artifacts

## Architecture
Diagram here.

## Build Output
| Metric | Value |
|--------|-------|
| Binary | 416 KB |

## Validation
Metrics here.

## Screenshots
Terminal output.

## Commit Log

| Hash | Date | Description |
|------|------|-------------|
| abc1234 | 2026-04-03 | Feature A |
| def5678 | 2026-04-02 | Feature B |

## How to Verify
Commands here.
"#;
        let hashes = extract_poa_hashes(poa);
        assert_eq!(hashes, vec!["abc1234", "def5678"]);

        let sections = vec!["## Architecture", "## Build Output", "## Validation", "## Screenshots", "## How to Verify"];
        assert!(check_required_fields(poa, &sections).is_empty());
    }

    #[test]
    fn integration_cross_doc_consistency() {
        let toi = "**Commit:** abc1234\n**Commit:** def5678";
        let poa = "| abc1234 | date | desc |\n| def5678 | date | desc |";
        let poa_hashes = extract_poa_hashes(poa);
        let toi_hashes = extract_toi_hashes(toi);

        // POA→TOI
        assert!(check_poa_in_toi(&poa_hashes, toi).is_empty());
        // TOI→POA
        assert!(check_toi_in_poa(&toi_hashes, &poa_hashes).is_empty());
    }

    #[test]
    fn integration_cross_doc_mismatch() {
        let toi = "**Commit:** abc1234";
        let poa = "| abc1234 | date | A |\n| orphan0 | date | B |";
        let poa_hashes = extract_poa_hashes(poa);

        // POA hash "orphan0" not in TOI — but wait, it's only 7 chars and all hex? yes.
        let missing = check_poa_in_toi(&poa_hashes, toi);
        // "orphan0" contains 'r','p','h','n' which are not hex
        // Actually: o=not hex. So extract_poa_hashes won't pick it up.
        // Let's use a real hex hash instead.
        let poa2 = "| abc1234 | date | A |\n| bbbbbbb | date | B |";
        let poa_hashes2 = extract_poa_hashes(poa2);
        assert_eq!(poa_hashes2, vec!["abc1234", "bbbbbbb"]);
        let missing2 = check_poa_in_toi(&poa_hashes2, toi);
        assert_eq!(missing2, vec!["bbbbbbb"]);
    }

    #[test]
    fn integration_date_order_violation_detected() {
        let toi = "### 2026-04-01 — A\n### 2026-04-03 — B\n### 2026-03-30 — C";
        let dates = extract_toi_dates(toi);
        let violations = check_date_order(&dates);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0], ("2026-04-01", "2026-04-03"));
    }

    #[test]
    fn integration_git_log_verification() {
        let toi_hashes = vec!["abc1234", "def5678"];
        let git_log = "abc1234 commit one\ndef5678 commit two\nghi9012 commit three";

        // All TOI hashes in git
        assert!(check_hashes_in_git(&toi_hashes, git_log).is_empty());

        // Git coverage: all 3 git commits need POA entries
        let git_hashes = parse_git_hashes(git_log);
        assert_eq!(git_hashes.len(), 3);
        let poa_hashes = vec!["abc1234", "def5678"];
        let uncovered = check_git_coverage(&git_hashes, &poa_hashes);
        assert_eq!(uncovered, vec!["ghi9012"]);
    }

    // =========================================================================
    // Edge cases and boundary conditions
    // =========================================================================

    #[test]
    fn edge_empty_strings_everywhere() {
        assert!(extract_toi_hashes("").is_empty());
        assert!(extract_poa_hashes("").is_empty());
        assert!(extract_toi_dates("").is_empty());
        assert!(check_date_order(&[]).is_empty());
        assert!(check_required_fields("", &[]).is_empty());
        let (t, v, i) = validate_ai_roles("");
        assert_eq!((t, v), (0, 0));
        assert!(i.is_empty());
    }

    #[test]
    fn edge_single_pipe_line() {
        let poa = "|";
        assert!(extract_poa_hashes(poa).is_empty());
    }

    #[test]
    fn edge_table_separator_row() {
        let poa = "|------|------|------|";
        assert!(extract_poa_hashes(poa).is_empty());
    }

    #[test]
    fn edge_commit_with_only_spaces_between_commas() {
        let toi = "**Commit:** abc1234, , , def5678";
        assert_eq!(extract_toi_hashes(toi), vec!["abc1234", "def5678"]);
    }

    #[test]
    fn edge_exactly_7_char_hash() {
        let toi = "**Commit:** 0000000";
        assert_eq!(extract_toi_hashes(toi), vec!["0000000"]);
    }

    #[test]
    fn edge_exactly_40_char_hash() {
        let h = "0123456789abcdef0123456789abcdef01234567";
        assert_eq!(h.len(), 40);
        let toi = format!("**Commit:** {h}");
        assert_eq!(extract_toi_hashes(&toi), vec![h]);
    }

    #[test]
    fn edge_date_at_year_boundary() {
        let dates = vec!["2027-01-01", "2026-12-31"];
        assert!(check_date_order(&dates).is_empty());
    }

    #[test]
    fn edge_same_date_multiple_entries() {
        let dates = vec!["2026-04-03", "2026-04-03", "2026-04-03"];
        assert!(check_date_order(&dates).is_empty());
    }

    #[test]
    fn edge_ai_role_with_ai_in_word() {
        // "maintained" contains "ai" but not "ai "
        let toi = "**AI Role:** Human maintained everything.";
        let (total, valid, _) = validate_ai_roles(toi);
        assert_eq!(total, 1);
        assert_eq!(valid, 0); // no "ai " token
    }

    #[test]
    fn edge_poa_table_with_extra_pipes() {
        let poa = "|| abc1234 | date | desc ||";
        // split by | gives: ["", "", " abc1234 ", " date ", " desc ", "", ""]
        // cols[1] is "" (empty), cols[2] is " abc1234 " — but we check cols[1]
        // So this should NOT extract abc1234 from cols[1]
        let hashes = extract_poa_hashes(poa);
        // cols[1] = "" which is not a hash
        assert!(hashes.is_empty());
    }

    #[test]
    fn edge_multiple_commit_lines_same_entry() {
        let toi = "**Commit:** abc1234\n**Commit:** def5678";
        assert_eq!(extract_toi_hashes(toi), vec!["abc1234", "def5678"]);
    }

    // =========================================================================
    // f30 integration test (runs against real repo files)
    // =========================================================================

    #[test]
    fn f30_passes_on_real_repo() {
        // This tests the actual validator against the real repo
        assert_eq!(f30(), 0, "f30 should pass on the real repository");
    }
}

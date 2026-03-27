// Unlicense — cochranblock.org
//! provenance-docs-test. TRIPLE SIMS via exopack::triple_sims::f60. f30=run_tests.

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let ok = exopack::triple_sims::f60(|| async { provenance_docs::f30() == 0 }).await;
    std::process::exit(if ok { 0 } else { 1 });
}

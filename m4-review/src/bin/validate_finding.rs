//! Module 4 demo — runtime gate for sub-agent review output.
//!
//! Usage: `validate-finding <path-to.yaml>`
//!
//! Parses the YAML via `serde_yaml` into `m4_review::Review`, then
//! runs `Review::validate()`. Prints the reviewer+verdict and exits
//! 0 on success; prints the invariant violation and exits 2 on
//! failure.
//!
//! Provable contract: the fixture at
//! `contracts/review-finding.valid.yaml` must exit 0, and the fixture
//! at `contracts/review-finding.invalid.yaml` must exit 2 with an
//! `ApproveWithError` message. `cargo test` exercises both paths
//! through the library.

use std::env;
use std::fs;
use std::process;

use m4_review::Review;

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!("usage: validate-finding <path-to.yaml>");
        process::exit(2);
    };

    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            process::exit(2);
        }
    };

    let review: Review = match serde_yaml::from_str(&raw) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: YAML parse failed for {path}: {e}");
            process::exit(2);
        }
    };

    if let Err(e) = review.validate() {
        eprintln!("error: invariant violation in {path}: {e}");
        process::exit(2);
    }

    println!(
        "contract: {} produced verdict={:?} for {} — {} findings — OK",
        review.reviewer,
        review.verdict,
        review.target,
        review.findings.len()
    );
}

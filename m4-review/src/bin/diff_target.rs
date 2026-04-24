//! Module 4 demo — target binary for parallel sub-agent review.
//!
//! Provable contract: the CLI consumes one positional integer argument,
//! squares it, and prints the result. Missing argument exits with code
//! 2 and an error message on stderr. Exit code 0 implies the integer
//! was parsed, squared without overflow, and the result was printed.
//!
//! A learner asks Claude Code to spawn two parallel sub-agents to
//! review this file against `contracts/review-finding.yaml`. Each
//! sub-agent returns YAML; the parent validates against the schema
//! and rejects any response that violates a documented invariant
//! (for example, `approve` verdict alongside a severity=error finding).

use std::env;
use std::process;

fn square(n: i32) -> Option<i32> {
    n.checked_mul(n)
}

fn parse_arg(args: &[String]) -> Result<i32, String> {
    let raw = args
        .get(1)
        .ok_or_else(|| "missing integer argument".to_string())?;
    raw.parse::<i32>()
        .map_err(|e| format!("'{raw}' is not a valid i32: {e}"))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let n = match parse_arg(&args) {
        Ok(n) => n,
        Err(msg) => {
            eprintln!("error: {msg}");
            process::exit(2);
        }
    };

    let Some(squared) = square(n) else {
        eprintln!("error: {n}² overflows i32");
        process::exit(2);
    };

    println!("{squared}");

    // Contract proof: if we reach this line, every precondition held.
    // The assertion restates the invariant so a reviewer cannot remove
    // the end-of-main proof without also removing this line.
    assert!(squared == n.wrapping_mul(n));
    eprintln!("contract: input parsed, squared without overflow, result printed — OK");
}

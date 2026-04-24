//! Module 4 demo — target binary for parallel sub-agent review.
//!
//! Provable contract: the CLI consumes one positional integer argument,
//! squares it via `m4_review::square` (contract: square-kernel-v1),
//! and prints the result. Missing argument or overflow exits with
//! code 2 and an error message on stderr. Exit code 0 implies the
//! integer parsed, squared without overflow, and was printed.
//!
//! A learner asks Claude Code to spawn two parallel sub-agents to
//! review this file against `contracts/square-kernel-v1.yaml`. Each
//! sub-agent runs `pv validate` + `pv status` on the contract,
//! inspects the implementation, and returns a verdict. The parent
//! rejects any verdict that contradicts a documented invariant.

use std::env;
use std::process;

use m4_review::square;

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

    // Contract proof (square-kernel-v1): every invariant the YAML
    // declares holds here at runtime.
    assert_eq!(squared, n.wrapping_mul(n));
    assert!(squared >= 0, "square result negative — contract violated");
    assert_eq!(
        square(n),
        square(-n),
        "square asymmetric — contract violated"
    );
    eprintln!(
        "contract: square-kernel-v1 holds for n={n} — non-negative, symmetric, overflow-safe — OK"
    );
}

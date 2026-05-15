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
//!
//! All logic lives in `m4_review::diff_target_run`. This `main` is
//! a thin translation of the outcome enum to stdout/stderr + exit
//! codes, so the lib is the testable seam.

use std::env;
use std::process;

use m4_review::{diff_target_run, DiffTargetOutcome};

fn main() {
    let args: Vec<String> = env::args().collect();
    match diff_target_run(&args) {
        DiffTargetOutcome::Ok { squared, marker } => {
            println!("{squared}");
            eprintln!("{marker}");
        }
        DiffTargetOutcome::ParseError(msg) => {
            eprintln!("error: {msg}");
            process::exit(2);
        }
        DiffTargetOutcome::Overflow(n) => {
            eprintln!("error: {n}² overflows i32");
            process::exit(2);
        }
    }
}

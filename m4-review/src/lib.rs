//! Module 4 demo — integer squaring kernel used as the sub-agent review target.
//!
//! The provable contract is authored in
//! `contracts/square-kernel-v1.yaml` (aprender kernel format) and
//! validated with `pv validate`. This module is the Rust
//! implementation the contract governs. Two parallel sub-agents
//! review candidate edits against the YAML contract before the
//! parent accepts a diff.
//!
//! Provable contract (square-kernel-v1): `square(n)` returns
//! `Some(n·n)` when the product fits in `i32`, `None` on overflow,
//! and never panics. The unit tests below restate every invariant
//! from the YAML so `cargo test` is the runtime proof.

/// Square an `i32`, returning `None` on overflow.
///
/// Contract: `contracts/square-kernel-v1.yaml` (obligation F-SQ-001).
/// Every invariant is exercised in the module's unit tests.
pub fn square(n: i32) -> Option<i32> {
    n.checked_mul(n)
}

/// Parse `argv[1]` as an `i32`. Refactored out of the
/// `diff-target` binary so the failure paths (missing arg,
/// invalid parse) are unit-testable. Returns the error message the
/// binary prints to stderr.
pub fn parse_arg(args: &[String]) -> Result<i32, String> {
    let raw = args
        .get(1)
        .ok_or_else(|| "missing integer argument".to_string())?;
    raw.parse::<i32>()
        .map_err(|e| format!("'{raw}' is not a valid i32: {e}"))
}

/// Outcome of running the `diff-target` flow on `args`. The binary
/// translates this to stdout/stderr + exit code; this enum is the
/// testable seam.
#[derive(Debug, PartialEq, Eq)]
pub enum DiffTargetOutcome {
    /// Argument parsed and squared without overflow. `squared` is the
    /// printed result; `marker` is the contract-proof line written to
    /// stderr.
    Ok { squared: i32, marker: String },
    /// Parsing failed (missing arg, bad i32).
    ParseError(String),
    /// Squaring overflowed `i32`.
    Overflow(i32),
}

/// Run the `diff-target` pipeline against the given `argv` slice.
/// `main` is then a thin shell that pattern-matches on this outcome
/// to produce stdout/stderr + exit codes. The contract proof itself
/// runs inside the `Ok` arm.
pub fn diff_target_run(args: &[String]) -> DiffTargetOutcome {
    let n = match parse_arg(args) {
        Ok(n) => n,
        Err(msg) => return DiffTargetOutcome::ParseError(msg),
    };

    let Some(squared) = square(n) else {
        return DiffTargetOutcome::Overflow(n);
    };

    // Contract proof (square-kernel-v1): every invariant the YAML
    // declares holds here at runtime. If any assertion fails the
    // process panics, which is the intended behavior — the contract
    // is broken.
    assert_eq!(squared, n.wrapping_mul(n));
    assert!(squared >= 0, "square result negative — contract violated");
    assert_eq!(
        square(n),
        square(-n),
        "square asymmetric — contract violated"
    );

    let marker = format!(
        "contract: square-kernel-v1 holds for n={n} — non-negative, symmetric, overflow-safe — OK"
    );
    DiffTargetOutcome::Ok { squared, marker }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_at_zero() {
        assert_eq!(square(0), Some(0));
    }

    #[test]
    fn square_at_one() {
        assert_eq!(square(1), Some(1));
        assert_eq!(square(-1), Some(1));
    }

    #[test]
    fn square_is_non_negative_when_some() {
        // i32::MIN exercises the None branch — its absence was the
        // 1 uncovered line in baseline (the if-let's else path).
        for n in [i32::MIN, -46340, -1000, -1, 0, 1, 1000, 46340] {
            if let Some(x) = square(n) {
                assert!(x >= 0, "square({n}) = {x} < 0");
            }
        }
    }

    #[test]
    fn square_is_sign_symmetric() {
        for n in [-46340, -1000, -1, 0, 1, 1000, 46340] {
            assert_eq!(square(n), square(-n), "asymmetric at n={n}");
        }
    }

    #[test]
    fn square_min_overflows() {
        assert_eq!(square(i32::MIN), None);
    }

    #[test]
    fn square_agrees_with_checked_mul() {
        for n in [i32::MIN, -46341, -46340, 0, 46340, 46341, i32::MAX] {
            assert_eq!(square(n), n.checked_mul(n), "divergence at n={n}");
        }
    }

    // ── parse_arg ──────────────────────────────────────────────────

    fn argv(extras: &[&str]) -> Vec<String> {
        let mut v = vec!["diff-target".to_string()];
        v.extend(extras.iter().map(|s| (*s).to_string()));
        v
    }

    #[test]
    fn parse_arg_returns_int_when_valid() {
        assert_eq!(parse_arg(&argv(&["7"])), Ok(7));
        assert_eq!(parse_arg(&argv(&["-3"])), Ok(-3));
        assert_eq!(parse_arg(&argv(&["0"])), Ok(0));
    }

    #[test]
    fn parse_arg_errors_when_missing() {
        let err = parse_arg(&argv(&[])).unwrap_err();
        assert!(err.contains("missing integer argument"), "got: {err}");
    }

    #[test]
    fn parse_arg_errors_on_non_integer() {
        let err = parse_arg(&argv(&["banana"])).unwrap_err();
        assert!(err.contains("'banana'"), "got: {err}");
        assert!(err.contains("not a valid i32"), "got: {err}");
    }

    #[test]
    fn parse_arg_errors_on_overflow_text() {
        // 2^31 = 2147483648, one more than i32::MAX.
        let err = parse_arg(&argv(&["2147483648"])).unwrap_err();
        assert!(err.contains("'2147483648'"), "got: {err}");
        assert!(err.contains("not a valid i32"), "got: {err}");
    }

    // ── diff_target_run ────────────────────────────────────────────
    //
    // Assertions use `assert_eq!` directly against the outcome enum
    // (which derives PartialEq) so each test is a single equality
    // check — no `_ => panic!()` catch-all arms that would leave
    // unreachable branches uncovered by llvm-cov.

    #[test]
    fn diff_target_run_ok_for_in_range_integer() {
        assert_eq!(
            diff_target_run(&argv(&["7"])),
            DiffTargetOutcome::Ok {
                squared: 49,
                marker: "contract: square-kernel-v1 holds for n=7 \
                         — non-negative, symmetric, overflow-safe — OK"
                    .to_string(),
            }
        );
    }

    #[test]
    fn diff_target_run_negative_input_squares_positive() {
        assert_eq!(
            diff_target_run(&argv(&["-7"])),
            DiffTargetOutcome::Ok {
                squared: 49,
                marker: "contract: square-kernel-v1 holds for n=-7 \
                         — non-negative, symmetric, overflow-safe — OK"
                    .to_string(),
            }
        );
    }

    #[test]
    fn diff_target_run_parse_error_on_missing_arg() {
        assert_eq!(
            diff_target_run(&argv(&[])),
            DiffTargetOutcome::ParseError("missing integer argument".to_string()),
        );
    }

    #[test]
    fn diff_target_run_parse_error_on_garbage_arg() {
        assert_eq!(
            diff_target_run(&argv(&["nope"])),
            DiffTargetOutcome::ParseError(
                "'nope' is not a valid i32: invalid digit found in string".to_string()
            ),
        );
    }

    #[test]
    fn diff_target_run_overflow_on_i32_min() {
        assert_eq!(
            diff_target_run(&argv(&[&i32::MIN.to_string()])),
            DiffTargetOutcome::Overflow(i32::MIN),
        );
    }
}

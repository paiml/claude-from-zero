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
        for n in [-46340, -1000, -1, 0, 1, 1000, 46340] {
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
}

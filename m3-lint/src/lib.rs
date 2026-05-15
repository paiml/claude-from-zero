//! Module 3 demo — crate with planted lint issues that `/lint-demo` catches.
//!
//! This crate is intentionally imperfect. Every function below carries
//! a clippy warning the `/lint-demo` skill is expected to surface when
//! a learner invokes `/lint-demo m3-lint`. Do not "fix" the warnings —
//! they are the lesson.

#![allow(dead_code)]

pub fn greet_everyone(names: &Vec<String>) -> String {
    // clippy: ptr_arg — prefer &[String] over &Vec<String>
    names
        .iter()
        .map(|n| format!("hello, {}", n))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn is_positive(n: i32) -> bool {
    // clippy: needless_return
    return n > 0;
}

pub fn double_if_small(n: i32) -> Option<i32> {
    // clippy: unnecessary_wraps — fn returns Option but never returns None
    Some(n.saturating_mul(2))
}

// Tests verify behavior of the planted-lint functions WITHOUT fixing the
// lints — the lints are intentional and are the M3 lesson. cargo clippy
// must still flag them (the CI workflow asserts this); cargo test must
// also pass (this module asserts behavior).
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_everyone_joins_names_with_newlines() {
        let names = vec!["world".to_string(), "claude".to_string()];
        assert_eq!(greet_everyone(&names), "hello, world\nhello, claude");
    }

    #[test]
    fn greet_everyone_empty_returns_empty_string() {
        let names: Vec<String> = Vec::new();
        assert_eq!(greet_everyone(&names), "");
    }

    #[test]
    fn greet_everyone_single_name_has_no_separator() {
        let names = vec!["alone".to_string()];
        assert_eq!(greet_everyone(&names), "hello, alone");
    }

    #[test]
    fn is_positive_true_for_positives() {
        assert!(is_positive(1));
        assert!(is_positive(i32::MAX));
    }

    #[test]
    fn is_positive_false_for_zero_and_negatives() {
        assert!(!is_positive(0));
        assert!(!is_positive(-1));
        assert!(!is_positive(i32::MIN));
    }

    #[test]
    fn double_if_small_doubles_in_range() {
        assert_eq!(double_if_small(0), Some(0));
        assert_eq!(double_if_small(7), Some(14));
        assert_eq!(double_if_small(-7), Some(-14));
    }

    #[test]
    fn double_if_small_saturates_on_overflow() {
        assert_eq!(double_if_small(i32::MAX), Some(i32::MAX));
        assert_eq!(double_if_small(i32::MIN), Some(i32::MIN));
    }
}

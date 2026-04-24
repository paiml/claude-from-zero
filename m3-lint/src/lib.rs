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

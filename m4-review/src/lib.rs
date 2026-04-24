//! Module 4 demo — runtime-enforced `ReviewFinding` contract.
//!
//! A parallel sub-agent reviewing `m4-review::diff-target` returns YAML
//! conforming to `contracts/review-finding.yaml`. The parent parses the
//! response through `serde_yaml` into [`Review`], then calls
//! [`Review::validate`] to enforce the invariants documented in the
//! YAML schema. Anything that violates an invariant is rejected before
//! the parent acts on the review.
//!
//! Provable contract: the invariants named in the YAML schema are the
//! same invariants enforced here. A unit test feeds both a valid and
//! an invariant-violating fixture through [`Review::validate`] and
//! asserts the expected outcome; `cargo test` exiting zero is the
//! proof.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Verdict {
    Approve,
    RequestChanges,
    Comment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Correctness,
    Style,
    Performance,
    Safety,
    Readability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub line: u32,
    pub message: String,
    pub category: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub reviewer: String,
    pub target: String,
    pub findings: Vec<Finding>,
    pub verdict: Verdict,
    pub rationale: String,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InvariantError {
    #[error("reviewer must not be empty")]
    EmptyReviewer,

    #[error("target must not be empty")]
    EmptyTarget,

    #[error("findings exceeds max of 10 (got {0})")]
    TooManyFindings(usize),

    #[error("message at findings[{0}] must be 10..=280 chars (got {1})")]
    MessageLengthOutOfRange(usize, usize),

    #[error("line at findings[{0}] must be >= 1")]
    LineZero(usize),

    #[error(
        "verdict=approve is incompatible with severity=error at findings[{0}]; \
         use verdict=request-changes instead"
    )]
    ApproveWithError(usize),

    #[error("severity=error at findings[{0}] requires verdict=request-changes (got {1:?})")]
    ErrorRequiresRequestChanges(usize, Verdict),

    #[error("rationale must be 20..=500 chars (got {0})")]
    RationaleLengthOutOfRange(usize),

    #[error("rationale must mention target {0:?} or its basename")]
    RationaleMissingTarget(String),
}

impl Review {
    /// Enforce every invariant documented in `contracts/review-finding.yaml`.
    ///
    /// This is the runtime half of the contract. The YAML schema is
    /// the human-readable half; keeping both in sync is the whole point
    /// of this module.
    pub fn validate(&self) -> Result<(), InvariantError> {
        if self.reviewer.trim().is_empty() {
            return Err(InvariantError::EmptyReviewer);
        }
        if self.target.trim().is_empty() {
            return Err(InvariantError::EmptyTarget);
        }
        if self.findings.len() > 10 {
            return Err(InvariantError::TooManyFindings(self.findings.len()));
        }

        for (i, f) in self.findings.iter().enumerate() {
            let msg_len = f.message.chars().count();
            if !(10..=280).contains(&msg_len) {
                return Err(InvariantError::MessageLengthOutOfRange(i, msg_len));
            }
            if f.line < 1 {
                return Err(InvariantError::LineZero(i));
            }
        }

        let first_error = self
            .findings
            .iter()
            .position(|f| f.severity == Severity::Error);

        match (self.verdict, first_error) {
            (Verdict::Approve, Some(i)) => return Err(InvariantError::ApproveWithError(i)),
            (v, Some(i)) if v != Verdict::RequestChanges => {
                return Err(InvariantError::ErrorRequiresRequestChanges(i, v));
            }
            _ => {}
        }

        let rationale_len = self.rationale.chars().count();
        if !(20..=500).contains(&rationale_len) {
            return Err(InvariantError::RationaleLengthOutOfRange(rationale_len));
        }

        let basename = std::path::Path::new(&self.target)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.target);
        if !self.rationale.contains(&self.target) && !self.rationale.contains(basename) {
            return Err(InvariantError::RationaleMissingTarget(self.target.clone()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_YAML: &str = include_str!("../../contracts/review-finding.valid.yaml");
    const INVALID_YAML: &str = include_str!("../../contracts/review-finding.invalid.yaml");

    #[test]
    fn valid_fixture_parses_and_validates() {
        let review: Review = serde_yaml::from_str(VALID_YAML).expect("parse");
        review.validate().expect("valid fixture should validate");
    }

    #[test]
    fn invalid_fixture_fails_approve_with_error_invariant() {
        let review: Review = serde_yaml::from_str(INVALID_YAML).expect("parse");
        let err = review.validate().expect_err("invalid fixture must fail");
        assert!(
            matches!(err, InvariantError::ApproveWithError(_)),
            "expected ApproveWithError, got {err:?}"
        );
    }

    #[test]
    fn empty_reviewer_rejected() {
        let mut review: Review = serde_yaml::from_str(VALID_YAML).unwrap();
        review.reviewer.clear();
        assert_eq!(
            review.validate().unwrap_err(),
            InvariantError::EmptyReviewer
        );
    }

    #[test]
    fn rationale_missing_target_rejected() {
        let mut review: Review = serde_yaml::from_str(VALID_YAML).unwrap();
        review.rationale = "a".repeat(50);
        assert!(matches!(
            review.validate().unwrap_err(),
            InvariantError::RationaleMissingTarget(_)
        ));
    }

    #[test]
    fn line_zero_rejected() {
        let mut review: Review = serde_yaml::from_str(VALID_YAML).unwrap();
        review.findings[0].line = 0;
        assert_eq!(review.validate().unwrap_err(), InvariantError::LineZero(0));
    }
}

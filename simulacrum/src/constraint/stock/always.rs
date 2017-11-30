//! `Constraint`s that always pass or fail, for testing.

use constraint::{Constraint, ConstraintError, ConstraintResult};

/// A `Constraint` that always passes.
pub struct AlwaysPass;

impl<I> Constraint<I> for AlwaysPass {
    fn verify(&self) -> ConstraintResult {
        Ok(())
    }
}

/// A `Constraint` that always fails.
pub struct AlwaysFail;

impl<I> Constraint<I> for AlwaysFail {
    fn verify(&self) -> ConstraintResult {
        Err(ConstraintError::AlwaysFail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_pass() {
        let c: AlwaysPass = AlwaysPass;

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_ok(), "Constraint should always pass");
    }

    #[test]
    fn test_always_fail() {
        let c = AlwaysFail;

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_err(), "Constraint should always fail");
        assert_eq!(r.unwrap_err(), ConstraintError::AlwaysFail, "Constraint should return the correct error");
    }
}
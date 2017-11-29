use std::fmt;

pub type ConstraintResult = Result<(), ConstraintError>;

#[derive(Debug, PartialEq)]
pub enum ConstraintError {
    AlwaysFail,
    CalledTooFewTimes(i64),
    CalledTooManyTimes(i64),
    CallNotExpected,
    MismatchedParams,
}

use self::ConstraintError::*;

impl fmt::Display for ConstraintError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AlwaysFail => {
                write!(f, "Expectation will always fail")
            },
            &CalledTooFewTimes(times) => {
                write!(f, "Called {} times fewer than expected", times)
            },
            &CalledTooManyTimes(times) => {
                write!(f, "Called {} times more than expected", times)
            },
            &CallNotExpected => {
                write!(f, "Called when not expected")
            },
            &MismatchedParams => {
                write!(f, "Called with unexpected parameters")
            },
        }
    }
}

/// The `Constraint`s attatched to an `Expectation` must all pass in order for the
/// `Excpectation` to also pass.
pub enum Constraint<I> {
    /// A method must be called with parameters that meet certain requirements.
    /// The data member is a closure that can be called with the params to verify this.
    Params(Box<FnMut(I) -> bool>),
    /// A method must be called a certain number of times
    Times(i64),
    /// For testing
    AlwaysPass,
    /// For testing
    AlwaysFail
}

impl<I> Constraint<I> where
    I: 'static 
{
    fn verify(&self) -> ConstraintResult {
        match self {
            &Constraint::AlwaysFail => Err(ConstraintError::AlwaysFail),
            &Constraint::Times(times) => {
                match times {
                    x if x < 0 => Err(ConstraintError::CalledTooManyTimes(x.abs())),
                    x if x > 0 => Err(ConstraintError::CalledTooFewTimes(x)),
                    _ => Ok(())
                }
            },
            _ => Ok(())
        }
    }
}

/*
impl Expectation {
    pub fn validatemmm(&mut self) -> ExpectationResult {
        match self {
            &mut Expectation::CallArgs(key, boxed_t) => {
                boxed_t.validate()
            },
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_pass() {
        let c: Constraint<()> = Constraint::AlwaysPass;

        assert!(c.verify().is_ok(), "Constraint should always pass");
    }

    #[test]
    fn test_always_fail() {
        let c: Constraint<()> = Constraint::AlwaysFail;

        assert!(c.verify().is_err(), "Constraint should always fail");
        assert_eq!(c.verify().unwrap_err(), ConstraintError::AlwaysFail, "Constraint should return the correct error");
    }

    #[test]
    fn test_times_pass() {
        let c: Constraint<()> = Constraint::Times(0);

        assert!(c.verify().is_ok());
    }

    #[test]
    fn test_times_fail_called_fewer() {
        let c: Constraint<()> = Constraint::Times(1);

        assert!(c.verify().is_err(), "Constraint should fail");
        assert_eq!(c.verify().unwrap_err(), ConstraintError::CalledTooFewTimes(1), "Constraint should return the correct error");
    }

    #[test]
    fn test_times_fail_called_more() {
        let c: Constraint<()> = Constraint::Times(-1);

        assert!(c.verify().is_err(), "Constraint should fail");
        assert_eq!(c.verify().unwrap_err(), ConstraintError::CalledTooManyTimes(1), "Constraint should return the correct error");
    }

    #[test]
    fn test_params_pass() {

    }

    #[test]
    fn test_params_fail() {

    }
}
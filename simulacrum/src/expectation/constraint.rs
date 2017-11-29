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
    ///
    /// Data member 0 is a closure that can be called with the params to verify this.
    ///
    /// Data member 1 is a boolean that is true if the method has been called with
    /// valid parameters every time.
    Params(Box<FnMut(I) -> bool>, bool),
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
    fn verify(&mut self) -> ConstraintResult {
        match self {
            &mut Constraint::AlwaysFail => Err(ConstraintError::AlwaysFail),
            &mut Constraint::Times(times) => {
                match times {
                    x if x < 0 => Err(ConstraintError::CalledTooManyTimes(x.abs())),
                    x if x > 0 => Err(ConstraintError::CalledTooFewTimes(x)),
                    _ => Ok(())
                }
            },
            &mut Constraint::Params(_, is_valid) => {
                if is_valid {
                    Ok(())
                } else {
                    Err(ConstraintError::MismatchedParams)
                }
            },
            _ => Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_pass() {
        let mut c: Constraint<()> = Constraint::AlwaysPass;

        let r = c.verify();

        assert!(r.is_ok(), "Constraint should always pass");
    }

    #[test]
    fn test_always_fail() {
        let mut c: Constraint<()> = Constraint::AlwaysFail;

        let r = c.verify();

        assert!(r.is_err(), "Constraint should always fail");
        assert_eq!(r.unwrap_err(), ConstraintError::AlwaysFail, "Constraint should return the correct error");
    }

    #[test]
    fn test_times_pass() {
        let mut c: Constraint<()> = Constraint::Times(0);

        let r = c.verify();

        assert!(r.is_ok());
    }

    #[test]
    fn test_times_fail_called_fewer() {
        let mut c: Constraint<()> = Constraint::Times(1);

        let r = c.verify();

        assert!(r.is_err(), "Constraint should fail");
        assert_eq!(r.unwrap_err(), ConstraintError::CalledTooFewTimes(1), "Constraint should return the correct error");
    }

    #[test]
    fn test_times_fail_called_more() {
        let mut c: Constraint<()> = Constraint::Times(-1);

        let r = c.verify();

        assert!(r.is_err(), "Constraint should fail");
        assert_eq!(r.unwrap_err(), ConstraintError::CalledTooManyTimes(1), "Constraint should return the correct error");
    }

    #[test]
    fn test_params_pass() {
        let mut c: Constraint<()> = Constraint::Params(Box::new(|_| false), true);

        let r = c.verify();

        assert!(r.is_ok(), "Constraint should pass");
    }

    #[test]
    fn test_params_fail() {
        let mut c: Constraint<()> = Constraint::Params(Box::new(|_| false), false);

        let r = c.verify();

        assert!(r.is_err(), "Constraint should fail");
        assert_eq!(r.unwrap_err(), ConstraintError::MismatchedParams, "Constraint should return the correct error");
    }
}
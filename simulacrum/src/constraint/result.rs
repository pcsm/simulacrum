use std::fmt;

/// The Error type is a message to be printed to the user.
pub type ConstraintResult = Result<(), ConstraintError>;

#[derive(Debug, PartialEq)]
pub enum ConstraintError {
    AlwaysFail,
    CalledTooFewTimes(i64),
    CalledTooManyTimes(i64),
    CallNotExpected,
    Custom(String), // For custom constraints from users
    MismatchedParams,
}

use self::ConstraintError::*;

impl fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ConstraintError::AlwaysFail => {
                write!(f, "Expectation will always fail.")
            },
            &ConstraintError::CalledTooFewTimes(times) => {
                write!(f, "Called {} times fewer than expected.", times)
            },
            &ConstraintError::CalledTooManyTimes(times) => {
                write!(f, "Called {} times more than expected.", times)
            },
            &ConstraintError::CallNotExpected => {
                write!(f, "Called when not expected.")
            },
            &ConstraintError::Custom(ref msg) => {
                write!(f, "{}", msg)
            },
            &ConstraintError::MismatchedParams => {
                write!(f, "Called with unexpected parameters.")
            },
        }
    }
}
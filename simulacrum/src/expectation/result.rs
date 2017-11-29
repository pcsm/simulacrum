use std::fmt;

use MethodName;

pub type ExpectationResult = Result<(), ExpectationError>;

pub enum ExpectationError {
    CalledTooFewTimes(MethodName, i64),
    CalledTooManyTimes(MethodName, i64),
    CallNotExpected(MethodName),
    MismatchedParams(MethodName),
}

impl fmt::Display for ExpectationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ExpectationError::CalledTooFewTimes(name, times) => {
                write!(f, "{} was called {} times fewer than expected.", name, times)
            },
            &ExpectationError::CalledTooManyTimes(name, times) => {
                write!(f, "{} was called {} times more than expected.", name, times)
            },
            &ExpectationError::CallNotExpected(name) => {
                write!(f, "{} was called when not expected.", name)
            },
            &ExpectationError::MismatchedParams(name) => {
                write!(f, "{} was called with unexpected parameters.", name)
            }
        }
    }
}
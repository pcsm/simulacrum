use std::clone::Clone;
use std::fmt;

use MethodName;
use constraint::ConstraintError;

pub type ExpectationResult = Result<(), ExpectationError>;

#[derive(Clone, Debug, PartialEq)]
pub struct ExpectationError {
    pub constraint_err: ConstraintError,
    pub method_name: MethodName
}

impl fmt::Display for ExpectationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.method_name, self.constraint_err)
    }
}
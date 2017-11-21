/*
use std::collections::vec_deque::VecDeque;

pub type TrackedMethodKey = &'static str;

pub type ExpectationResult = Result<(), ExpectationError>;

pub enum ExpectationError {
    CalledTooFewTimes(TrackedMethodKey, i64),
    CalledTooManyTimes(TrackedMethodKey, i64),
    CalledOutOfOrder,
    MismatchedArgs(TrackedMethodKey),
}

trait CallArgsT {
    fn validate(&mut self) -> ExpectationResult;
}

trait Expectation {
    fn validate(&mut self) -> ExpectationResult;
}

// enum Expectation {
//     /// A method must be called with arguments that meet certain requirements
//     CallArgs(TrackedMethodKey, Box<CallArgsT>),
//     /// A method must be called a certain number of times
//     CallTimes(TrackedMethodKey, i64),
//     /// Expectations evaluated in any order
//     Group(Vec<Expectation>),
//     /// Expectations evaluated in this specific order
//     Sequence(VecDeque<Expectation>),
// }

struct CallTimes(TrackedMethodKey, i64);
impl Expectation for CallTimes {
    
}

struct Group(Vec<Box<Expectation>>);
impl Expectation for Group {
    fn validate(&mut self) -> ExpectationResult {
        for exp in self.0.iter() {
            exp.validate()?
        }
        Ok(())
    }
}

struct Sequence(VecDeque<Box<Expectation>>);
impl Expectation for Sequence {
    fn validate(&mut self) -> ExpectationResult {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(ExpectationError::CalledOutOfOrder)
        }
    }
}

impl Expectation {
    pub fn validatemmm(&mut self) -> ExpectationResult {
        match self {
            &mut Expectation::CallArgs(key, boxed_t) => {
                boxed_t.validate()
            },
            &mut Expectation::CallTimes(key, times) => {
                match times {
                    x if x < 0 => Err(ExpectationError::CalledTooManyTimes(key, x.abs())),
                    x if x > 0 => Err(ExpectationError::CalledTooFewTimes(key, x)),
                    _ => Ok(())
                }
            },
        }
    }
}
*/
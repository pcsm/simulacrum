use std::fmt;

use super::super::Validator;

/// A `Validator` that either always passes or fails.
pub struct Trivial(bool);

impl<I> Validator<I> for Trivial {
    fn validate(&mut self, _param: &I) -> bool {
        self.0
    }
}

pub fn any() -> Trivial {
    Trivial(true)
}

pub fn none() -> Trivial {
    Trivial(false)
}

impl fmt::Debug for Trivial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 {
            write!(f, "Always Passes")
        } else {
            write!(f, "Always Fails")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any() {
        let mut c = any();
        assert!(c.validate(&()));
    }

    #[test]
    fn test_none() {
        let mut c = none();
        assert!(!c.validate(&()));
    }
}
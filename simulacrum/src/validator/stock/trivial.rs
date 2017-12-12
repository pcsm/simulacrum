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
use super::super::Validator;

/// A `Validator` that either always passes or fails.
pub struct Trivial(bool);

pub fn any() -> Trivial {
    Trivial(true)
}

pub fn none() -> Trivial {
    Trivial(false)
}

impl<I> Validator<I> for Trivial {
    fn validate(&mut self, _param: &I) -> bool {
        self.0
    }

     fn print(&self) -> String {
        if self.0 {
            "Always Passes".to_owned()
        } else {
            "Always Fails".to_owned()
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
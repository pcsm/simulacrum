use super::super::Validator;

/// Parameter(s) must equal the provided value.
pub struct EqTo<I: PartialEq>(I);

pub fn eq<I: PartialEq>(other: I) -> EqTo<I> {
    EqTo(other)
}

impl<I: PartialEq> Validator<I> for EqTo<I> {
    fn validate(&mut self, param: &I) -> bool {
        *param == self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let mut c = eq(888);
        let v = 888;
        assert!(c.validate(&v));
    }

    #[test]
    fn test_validate_fail() {
        let mut c = eq(555);
        let v = 888;
        assert!(!c.validate(&v));
    }
}
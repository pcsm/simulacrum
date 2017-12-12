use super::super::Validator;

/// Parameter(s) must equal the provided value.
pub struct GreaterThan<I: PartialOrd>(I);

pub fn gt<I: PartialOrd>(other: I) -> GreaterThan<I> {
    GreaterThan(other)
}

impl<I: PartialOrd> Validator<I> for GreaterThan<I> {
    fn validate(&mut self, param: &I) -> bool {
        *param > self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gt() {
        let mut c = gt(888);
        let v = 999;
        assert!(c.validate(&v));
    }

    #[test]
    fn test_gt_fail() {
        let mut c = gt(555);
        let v = 1;
        assert!(!c.validate(&v));
    }
}
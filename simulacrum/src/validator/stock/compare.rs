use debugit::DebugIt;

use std::fmt;

use super::super::Validator;

pub struct GreaterThan<I: PartialOrd>(I);

/// Parameter(s) must be > the provided value.
pub fn gt<I: PartialOrd>(other: I) -> GreaterThan<I> {
    GreaterThan(other)
}

impl<I: PartialOrd> Validator<I> for GreaterThan<I> {
    fn validate(&mut self, param: &I) -> bool {
        *param > self.0
    }
}

impl<I: PartialOrd> fmt::Debug for GreaterThan<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "> {:?}", DebugIt(&self.0))
    }
}

pub struct LessThan<I: PartialOrd>(I);

/// Parameter(s) must be < the provided value.
pub fn lt<I: PartialOrd>(other: I) -> LessThan<I> {
    LessThan(other)
}

impl<I: PartialOrd> Validator<I> for LessThan<I> {
    fn validate(&mut self, param: &I) -> bool {
        *param < self.0
    }
}

impl<I: PartialOrd> fmt::Debug for LessThan<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "< {:?}", DebugIt(&self.0))
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

    #[test]
    fn test_lt() {
        let mut c = lt(10);
        let v = 1;
        assert!(c.validate(&v));
    }

    #[test]
    fn test_lt_fail() {
        let mut c = lt(10);
        let v = 25;
        assert!(!c.validate(&v));
    }
}
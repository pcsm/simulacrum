use super::super::Validator;

/// A family of `Validators` that splits tuples into their own validators.
pub struct Tuple2<A, B>(Box<Validator<A>>, Box<Validator<B>>);

impl<A, B> Validator<(A, B)> for Tuple2<A, B> {
    fn validate(&mut self, param: &(A, B)) -> bool {
        self.0.validate(&param.0) &&
        self.1.validate(&param.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::trivial::*;

    #[test]
    fn test_2_both() {
        let mut c = Tuple2(Box::new(any()), Box::new(any()));
        assert!(c.validate(&((), ())));
    }

    #[test]
    fn test_2_first() {
        let mut c = Tuple2(Box::new(any()), Box::new(none()));
        assert!(!c.validate(&((), ())));
    }

    #[test]
    fn test_2_second() {
        let mut c = Tuple2(Box::new(none()), Box::new(any()));
        assert!(!c.validate(&((), ())));
    }

    #[test]
    fn test_2_neither() {
        let mut c = Tuple2(Box::new(none()), Box::new(none()));
        assert!(!c.validate(&((), ())));
    }
}
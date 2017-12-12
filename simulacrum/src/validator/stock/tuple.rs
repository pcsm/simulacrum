use super::super::Validator;

/// Use this macro to create a `Validator` that works for methods with 2, 3, or 
/// 4 parameters.
macro_rules! params {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        Tuple4(Box::new($a), Box::new($b), Box::new($c), Box::new($d));
    };
    ($a:expr, $b:expr, $c:expr) => {
        Tuple3(Box::new($a), Box::new($b), Box::new($c));
    };
    ($a:expr, $b:expr) => {
        Tuple2(Box::new($a), Box::new($b))
    };
}

/// A family of `Validators` that splits tuples into their own validators.
pub struct Tuple2<A, B>(Box<Validator<A>>, Box<Validator<B>>);

impl<A, B> Validator<(A, B)> for Tuple2<A, B> {
    fn validate(&mut self, param: &(A, B)) -> bool {
        self.0.validate(&param.0) &&
        self.1.validate(&param.1)
    }
}

pub struct Tuple3<A, B, C>(Box<Validator<A>>, Box<Validator<B>>, Box<Validator<C>>);

impl<A, B, C> Validator<(A, B, C)> for Tuple3<A, B, C> {
    fn validate(&mut self, param: &(A, B, C)) -> bool {
        self.0.validate(&param.0) &&
        self.1.validate(&param.1) &&
        self.2.validate(&param.2)
    }
}

pub struct Tuple4<A, B, C, D>(Box<Validator<A>>, Box<Validator<B>>, Box<Validator<C>>, Box<Validator<D>>);

impl<A, B, C, D> Validator<(A, B, C, D)> for Tuple4<A, B, C, D> {
    fn validate(&mut self, param: &(A, B, C, D)) -> bool {
        self.0.validate(&param.0) &&
        self.1.validate(&param.1) &&
        self.2.validate(&param.2) &&
        self.3.validate(&param.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::trivial::*;

    #[test]
    fn test_2_both() {
        let mut c = params!(any(), any());
        assert!(c.validate(&((), ())));
    }

    #[test]
    fn test_2_first() {
        let mut c = params!(any(), none());
        assert!(!c.validate(&((), ())));
    }

    #[test]
    fn test_2_second() {
        let mut c = params!(none(), any());
        assert!(!c.validate(&((), ())));
    }

    #[test]
    fn test_2_neither() {
        let mut c = params!(none(), none());
        assert!(!c.validate(&((), ())));
    }

    #[test]
    fn test_3_all() {
        let mut c = params!(any(), any(), any());
        assert!(c.validate(&((), (), ())));
    }

    #[test]
    fn test_4_all() {
        let mut c = params!(any(), any(), any(), any());
        assert!(c.validate(&((), (), (), ())));
    }
}
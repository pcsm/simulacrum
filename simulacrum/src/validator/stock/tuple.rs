//! A family of `Validators` that splits tuples into their own validators.
use super::super::Validator;

macro_rules! create_tuple_validator {
    ($name:ident: $(($index:tt, $generic:ident)),*) => {
        pub struct $name<$($generic),*>(
            $(pub Box<Validator<$generic>>),*
        );

        impl<$($generic),*> Validator<($($generic),*)> for $name<$($generic),*> {
            fn validate(&mut self, param: &($($generic),*)) -> bool {
                $(self.$index.validate(&param.$index) &&)*
                true
            }

            fn print(&self) -> String {
                let inner = [
                    $(
                        self.$index.print(),
                    )*
                ].join(", ");
                format!("({})", inner)
            }
        }
    };
}

create_tuple_validator!(Tuple2: (0, A), (1, B));
create_tuple_validator!(Tuple3: (0, A), (1, B), (2, C));
create_tuple_validator!(Tuple4: (0, A), (1, B), (2, C), (3, D));
create_tuple_validator!(Tuple5: (0, A), (1, B), (2, C), (3, D), (4, E));
create_tuple_validator!(Tuple6: (0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
create_tuple_validator!(Tuple7: (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
create_tuple_validator!(Tuple8: (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G),  (7, H));
create_tuple_validator!(Tuple9: (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G),  (7, H), (8, I));

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
    fn test_3_fail() {
        let mut c = params!(none(), any(), none());
        assert!(!c.validate(&((), (), ())));
    }

    #[test]
    fn test_4_all() {
        let mut c = params!(any(), any(), any(), any());
        assert!(c.validate(&((), (), (), ())));
    }

    #[test]
    fn test_4_fail() {
        let mut c = params!(any(), none(), any(), none());
        assert!(!c.validate(&((), (), (), ())));
    }
}
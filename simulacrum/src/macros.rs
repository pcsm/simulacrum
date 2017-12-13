/// Use this macro to create a `Validator` that works for methods with 2, 3, or 
/// 4 parameters.
#[macro_export]
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

/// Use this macro to create an `.expect_METHOD_NAME()` method.
#[macro_export]
macro_rules! create_expect_method {
    ($name:ident($key:expr) $inputs:ty => $output:ty) => {
        pub fn $name(&mut self) -> Method<$inputs, $output> {
            self.e.expect::<$inputs, $output>($key)
        }
    };
    ($name:ident($key:expr) $inputs:ty) => {
        create_expect_method!($name($key) $inputs => ());
    };
    ($name:ident($key:expr)) => {
        create_expect_method!($name($key) () => ());
    };
}

#[macro_export]
macro_rules! create_mock {
    ($name:ident: {
        $([
            $($method:tt)*
        ])*
    }) => {
        pub struct $name {
            e: Expectations
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    e: Expectations::new()
                }
            }

            pub fn then(&mut self) -> &mut Self {
                self.e.then();
                self
            }

            $(
                create_expect_method!($($method)*);
            )*
        }
    };
}
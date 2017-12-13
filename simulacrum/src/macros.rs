/// Use this macro to create a `Validator` that works for methods with 2-6 parameters.
#[macro_export]
macro_rules! params {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr, $f: expr) => {
        Tuple6(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e), Box::new($f));
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e: expr) => {
        Tuple5(Box::new($a), Box::new($b), Box::new($c), Box::new($d), Box::new($e));
    };
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
    // (@create_expect_methods) => {};
    // (@create_expect_methods $name:ident($key:expr) $inputs:ty => $output:ty; $($tail:tt)*) => {
    //     create_expect_method!($name($key) $inputs => $output);
    //     create_mock!(@create_expect_methods $($tail)*);
    // };
    // (@create_expect_methods $name:ident($key:expr) $inputs:ty; $($tail:tt)*) => {
    //     create_expect_method!($name($key) $inputs);
    //     create_mock!(@create_expect_methods $($tail)*);
    // };
    // (@create_expect_methods $name:ident($key:expr); $($tail:tt)*) => {
    //     create_expect_method!($name($key));
    //     create_mock!(@create_expect_methods $($tail)*);
    // };

#[macro_export]
macro_rules! create_mock {
    // tuplefy - create an input tuple from a method signature
    (@tuplefy $($sig:tt)*) => {
        ()
    };

    // create_expect_methods
    (@create_expect_methods) => {};
    (@create_expect_methods
        $expect_name:ident($key:expr):
        fn $method_name:ident($($sig:tt)*);
        $($tail:tt)*
    ) => {
        create_expect_method!($expect_name($key) create_mock!(@tuplefy $($sig:tt)*));
        create_mock!(@create_expect_methods $($tail)*);
    };
    (@create_expect_methods
        $expect_name:ident($key:expr):
        fn $method_name:ident($($sig:tt)*)
        -> $output:ty;
        $($tail:tt)*
    ) => {
        create_expect_method!($expect_name($key) create_mock!(@tuplefy $($sig:tt)*) => $output);
        create_mock!(@create_expect_methods $($tail)*);
    };

    // create_stub_methods

    // main
    (impl $trait_name:ident for $mock_name:ident {
        $($method_info:tt)*
    }) => {
        pub struct $mock_name {
            e: Expectations
        }

        impl $mock_name {
            pub fn new() -> Self {
                Self {
                    e: Expectations::new()
                }
            }

            pub fn then(&mut self) -> &mut Self {
                self.e.then();
                self
            }

            create_mock!(@create_expect_methods $($method_info)*);
        }

        // impl $trait_name for $mock_name {
        //     create_mock!(@create_stub_methods $($method_info)*);
        // }
    };
}
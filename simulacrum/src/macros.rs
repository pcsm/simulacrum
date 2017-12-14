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

#[macro_export]
macro_rules! create_stub_method {
    ($key:expr, $inputs:ty => $output:ty, ($params:expr), $original_sig:tt) => {
        fn $name $original_sig -> $output {
            self.e.was_called_returning::<$inputs, $output>($key, $params)
        }
    };
    ($key:expr, $inputs:ty, ($params:expr), $original_sig:tt) => {
        fn $name $original_sig {
            self.e.was_called::<$inputs, ()>($key, $params)
        }
    };
}

#[macro_export]
macro_rules! create_mock {
    // Coerce a capture into a particular kind.
    // See https://danielkeep.github.io/tlborm/book/blk-ast-coercion.html
    (@as_ty $token:ty) => { $token };
    (@as_expr $token:expr) => { $token };

    // tuplefy
    //
    // Create an input tuple from a method signature tt.
    // Uses push-down accumulation pattern:
    // see https://danielkeep.github.io/tlborm/book/pat-push-down-accumulation.html
    // tuplefy - Strip off parentheses
    (@tuplefy $mode:tt ($($param:tt)*) -> ($($result:tt)*)) => {
        create_mock!(@tuplefy_loop $mode ($($param)*) -> ())
    };

    // tuplefy_loop - For each param, get the type. Ignore &self and &mut self.

    // If there are no params left, coerce the final result to a type with
    // parentheses around it.
    (@tuplefy_loop kind () -> ($($result:tt)*)) => {
        create_mock!(@as_ty ( $($result)* ))
    };
    (@tuplefy_loop name () -> ($($result:tt)*)) => {
        create_mock!(@as_expr ( $($result)* ))
    };
    
    // Ignore &self and &mut self.
    (@tuplefy_loop $mode:tt (& self) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop $mode () -> ($($result)*) )
    };
    (@tuplefy_loop $mode:tt (& mut self) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop $mode () -> ($($result)*) )
    };
    (@tuplefy_loop $mode:tt (& self, $($tail:tt)*) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop $mode ($($tail)*) -> ($($result)*) )
    };
    (@tuplefy_loop $mode:tt (& mut self, $($tail:tt)*) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop $mode ($($tail)*) -> ($($result)*) )
    };

    // Accept &'static params.
    (@tuplefy_loop kind ($name:ident: &'static $kind:ty) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop kind () -> ($($result)* &'static $kind) )
    };
    (@tuplefy_loop kind ($name:ident: &'static $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop kind ($($tail)*) -> ($($result)* &'static $kind,) )
    };

    // Convert & and &mut params to *const and *mut.
    (@tuplefy_loop kind ($name:ident: & $kind:ty) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop kind () -> ($($result)* *const $kind) )
    };
    (@tuplefy_loop kind ($name:ident: & mut $kind:ty) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop kind () -> ($($result)* *mut $kind) )
    };
    (@tuplefy_loop kind ($name:ident: & $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop kind ($($tail)*) -> ($($result)* *const $kind,) )
    };
    (@tuplefy_loop kind ($name:ident: & mut $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop kind ($($tail)*) -> ($($result)* *mut $kind,) )
    };

    // Get the type of the parameter and move on.
    (@tuplefy_loop kind ($name:ident: $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop kind ($($tail)*) -> ($($result)* $kind,) )
    };
    (@tuplefy_loop kind ($name:ident: $kind:ty) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop kind () -> ($($result)* $kind) )
    };

    // Get the name of the parameter and move on.
    (@tuplefy_loop name ($name:ident: $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop name ($($tail)*) -> ($($result)* $name,) )
    };
    (@tuplefy_loop name ($name:ident: $kind:ty) -> ($($result:tt)*)) => {
        create_mock!( @tuplefy_loop name () -> ($($result)* $name) )
    };

    // create_expect_methods
    (@create_expect_methods) => {};
    (@create_expect_methods
        $expect_name:ident($key:expr):
        fn $method_name:ident $sig:tt;
        $($tail:tt)*
    ) => {
        create_expect_method!($expect_name($key) create_mock!(@tuplefy kind $sig -> ()));
        create_mock!(@create_expect_methods $($tail)*);
    };
    (@create_expect_methods
        $expect_name:ident($key:expr):
        fn $method_name:ident $sig:tt -> $output:ty;
        $($tail:tt)*
    ) => {
        create_expect_method!($expect_name($key) create_mock!(@tuplefy kind $sig -> ()) => $output);
        create_mock!(@create_expect_methods $($tail)*);
    };

    // create_stub_methods
    (@create_stub_methods) => {};
    (@create_stub_methods
        $expect_name:ident($key:expr):
        fn $method_name:ident $sig:tt;
        $($tail:tt)*
    ) => {
        create_stub_method!(
            $key,
            create_mock!(@tuplefy kind $sig -> ()), 
            create_mock!(@tuplefy name $sig -> ()), 
            $sig);
        create_mock!(@create_stub_methods $($tail)*);
    };
    (@create_stub_methods
        $expect_name:ident($key:expr):
        fn $method_name:ident $sig:tt -> $output:ty;
        $($tail:tt)*
    ) => {
        create_stub_method!(
            $key,
            create_mock!(@tuplefy kind $sig -> ()) => $output,
            create_mock!(@tuplefy name $sig -> ()),
            $sig);
        create_mock!(@create_stub_methods $($tail)*);
    };

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

        impl $trait_name for $mock_name {
            create_mock!(@create_stub_methods $($method_info)*);
        }
    };
}
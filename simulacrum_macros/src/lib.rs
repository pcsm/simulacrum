extern crate simulacrum_mock;

pub use simulacrum_mock::{Expectations, Method};

/// Use this macro to create an `.expect_METHOD_NAME()` method.
#[macro_export]
macro_rules! create_expect_method {
    ($name:ident($key:expr) $inputs:ty => $output:ty) => {
        #[allow(non_snake_case)]
        pub fn $name(&mut self) -> $crate::Method<$inputs, $output> {
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

/// Use this macro to do a `self.e.was_called` `self.e.was_called_returning` with
/// a shorter interface.
#[macro_export]
macro_rules! was_called {
    ($self_:ident, $key:expr, $sig:tt -> $output:ty) => {
        #[allow(unused_parens)]
        $self_.e.was_called_returning::<simulacrum_tuplefy!(kind $sig -> ()), $output>($key, simulacrum_tuplefy!(name $sig -> ()))
    };
    ($self_:ident, $key:expr, $sig:tt) => {
        #[allow(unused_parens)]
        $self_.e.was_called::<simulacrum_tuplefy!(kind $sig -> ()), ()>($key, simulacrum_tuplefy!(name $sig -> ()))
    };
    ($self_:ident, $key:expr) => {
        #[allow(unused_parens)]
        $self_.e.was_called::<(), ()>($key, ())
    };
}

// Create an input tuple from a method signature tt.
// Uses push-down accumulation pattern:
// see https://danielkeep.github.io/tlborm/book/pat-push-down-accumulation.html
#[macro_export]
macro_rules! simulacrum_tuplefy {
    // Coerce a capture into a particular kind.
    // See https://danielkeep.github.io/tlborm/book/blk-ast-coercion.html
    (@as_ty $token:ty) => { $token };
    (@as_expr $token:expr) => { $token };

    // main - Strip off parentheses
    ($mode:tt ($($param:tt)*) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!(@inner $mode ($($param)*) -> ())
    };

    // simulacrum_tuplefy - For each param, get the type. Ignore &self and &mut self.

    // If there are no params left, coerce the final result to a type with
    // parentheses around it.
    (@inner kind () -> ($($result:tt)*)) => {
        simulacrum_tuplefy!(@as_ty ( $($result)* ))
    };
    (@inner name () -> ($($result:tt)*)) => {
        simulacrum_tuplefy!(@as_expr ( $($result)* ))
    };
    
    // Ignore &self and &mut self.
    (@inner $mode:tt (& self) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner $mode () -> ($($result)*) )
    };
    (@inner $mode:tt (& mut self) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner $mode () -> ($($result)*) )
    };
    (@inner $mode:tt (& self, $($tail:tt)*) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner $mode ($($tail)*) -> ($($result)*) )
    };
    (@inner $mode:tt (& mut self, $($tail:tt)*) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner $mode ($($tail)*) -> ($($result)*) )
    };

    // Accept &'static params.
    (@inner kind ($name:ident: &'static $kind:ty) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner kind () -> ($($result)* &'static $kind) )
    };
    (@inner kind ($name:ident: &'static $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner kind ($($tail)*) -> ($($result)* &'static $kind,) )
    };

    // Convert & and &mut params to *const and *mut.
    (@inner kind ($name:ident: & $kind:ty) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner kind () -> ($($result)* *const $kind) )
    };
    (@inner kind ($name:ident: & mut $kind:ty) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner kind () -> ($($result)* *mut $kind) )
    };
    (@inner kind ($name:ident: & $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner kind ($($tail)*) -> ($($result)* *const $kind,) )
    };
    (@inner kind ($name:ident: & mut $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner kind ($($tail)*) -> ($($result)* *mut $kind,) )
    };

    // Get the type of the parameter and move on.
    (@inner kind ($name:ident: $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner kind ($($tail)*) -> ($($result)* $kind,) )
    };
    (@inner kind ($name:ident: $kind:ty) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner kind () -> ($($result)* $kind) )
    };

    // Get the name of the parameter and move on.
    (@inner name ($name:ident: $kind:ty, $($tail:tt)*) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner name ($($tail)*) -> ($($result)* $name,) )
    };
    (@inner name ($name:ident: $kind:ty) -> ($($result:tt)*)) => {
        simulacrum_tuplefy!( @inner name () -> ($($result)* $name) )
    };
}

#[macro_export]
macro_rules! create_mock_struct {
    (@create_expect_methods) => {};
    (@create_expect_methods $name:ident($key:expr) $inputs:ty => $output:ty; $($tail:tt)*) => {
        create_expect_method!($name($key) $inputs => $output);
        create_mock_struct!(@create_expect_methods $($tail)*);
    };
    (@create_expect_methods $name:ident($key:expr) $inputs:ty; $($tail:tt)*) => {
        create_expect_method!($name($key) $inputs);
        create_mock_struct!(@create_expect_methods $($tail)*);
    };
    (@create_expect_methods $name:ident($key:expr); $($tail:tt)*) => {
        create_expect_method!($name($key));
        create_mock_struct!(@create_expect_methods $($tail)*);
    };
    (struct $name:ident: {
        $($methods:tt)*
    }) => {
        #[allow(non_snake_case)]
        pub struct $name {
            e: $crate::Expectations
        }

        #[allow(non_snake_case)]
        impl $name {
            pub fn new() -> Self {
                Self {
                    e: $crate::Expectations::new()
                }
            }

            pub fn then(&mut self) -> &mut Self {
                self.e.then();
                self
            }

            create_mock_struct!(@create_expect_methods $($methods)*);
        }
    };
}

#[macro_export]
macro_rules! create_mock {
    // create_mock_struct
    (@create_mock_struct($mock_name:ident, ()) -> ($($result:tt)*)) => {
        create_mock_struct! {
            struct $mock_name: {
                $($result)*
            }
        }
    };
    (@create_mock_struct
        ($mock_name:ident, (
            $expect_name:ident($key:expr):
            fn $method_name:ident $sig:tt;
            $($tail:tt)*
        )) -> ($($result:tt)*)
    ) => {
        create_mock!(@create_mock_struct ($mock_name, ($($tail)*)) -> (
            $($result)* 
            $expect_name($key) simulacrum_tuplefy!(kind $sig -> ());
        ));
    };
    (@create_mock_struct
        ($mock_name:ident, (
            $expect_name:ident($key:expr):
            fn $method_name:ident $sig:tt -> $output:ty;
            $($tail:tt)*
        )) -> ($($result:tt)*)
    ) => {
        create_mock!(@create_mock_struct ($mock_name, ($($tail)*)) -> (
            $($result)* 
            $expect_name($key) simulacrum_tuplefy!(kind $sig -> ()) => $output;
        ));
    };
    (@create_mock_struct
        ($mock_name:ident, (
            $expect_name:ident($key:expr):
            unsafe fn $method_name:ident $sig:tt;
            $($tail:tt)*
        )) -> ($($result:tt)*)
    ) => {
        create_mock!(@create_mock_struct ($mock_name, ($($tail)*)) -> (
            $($result)* 
            $expect_name($key) simulacrum_tuplefy!(kind $sig -> ());
        ));
    };
    (@create_mock_struct
        ($mock_name:ident, (
            $expect_name:ident($key:expr):
            unsafe fn $method_name:ident $sig:tt -> $output:ty;
            $($tail:tt)*
        )) -> ($($result:tt)*)
    ) => {
        create_mock!(@create_mock_struct ($mock_name, ($($tail)*)) -> (
            $($result)* 
            $expect_name($key) simulacrum_tuplefy!(kind $sig -> ()) => $output;
        ));
    };

    // create_stub_methods
    (@create_stub_methods ($self_:ident)) => {};
    (@create_stub_methods ($self_:ident)
        $expect_name:ident($key:expr):
        fn $method_name:ident $sig:tt;
        $($tail:tt)*
    ) => {
        #[allow(warnings)]
        fn $method_name $sig {
            was_called!($self_, $key, $sig)
        }
        create_mock!(@create_stub_methods ($self_) $($tail)*);
    };
    (@create_stub_methods ($self_:ident)
        $expect_name:ident($key:expr):
        fn $method_name:ident $sig:tt -> $output:ty;
        $($tail:tt)*
    ) => {
        #[allow(warnings)]
        fn $method_name $sig -> $output {
            was_called!($self_, $key, $sig -> $output)
        }
        create_mock!(@create_stub_methods ($self_) $($tail)*);
    };
    (@create_stub_methods ($self_:ident)
        $expect_name:ident($key:expr):
        unsafe fn $method_name:ident $sig:tt;
        $($tail:tt)*
    ) => {
        #[allow(warnings)]
        unsafe fn $method_name $sig {
            was_called!($self_, $key, $sig)
        }
        create_mock!(@create_stub_methods ($self_) $($tail)*);
    };
    (@create_stub_methods ($self_:ident)
        $expect_name:ident($key:expr):
        unsafe fn $method_name:ident $sig:tt -> $output:ty;
        $($tail:tt)*
    ) => {
        #[allow(warnings)]
        unsafe fn $method_name $sig -> $output {
            was_called!($self_, $key, $sig -> $output)
        }
        create_mock!(@create_stub_methods ($self_) $($tail)*);
    };

    // main
    (impl $trait_name:ident for $mock_name:ident ($self_:ident) {
        $($method_info:tt)*
    }) => {
        create_mock!(@create_mock_struct ($mock_name, ($($method_info)*)) -> ());

        impl $trait_name for $mock_name {
            create_mock!(@create_stub_methods ($self_) $($method_info)*);
        }
    };
}
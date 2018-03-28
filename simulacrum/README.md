Simulacrum [![Docs](https://docs.rs/simulacrum/badge.svg)](https://docs.rs/simulacrum) [![Crates.io](https://img.shields.io/crates/v/simulacrum.svg)](https://crates.io/crates/simulacrum)
==================================================================

A minimal library for creating mock objects by hand using stable Rust.

To install, add this line to your Cargo.toml:

```toml
[dependencies]
simulacrum = "0.3.0"
```

Note that this crate has not yet reached version 1.0, so the API may change drastically between releases.

## Using Mock Objects

Simulacrum mock objects provide a consistent interface that features call counts, parameter matching, return values, and modifying parameters by mutable reference.

```rust
// Create a mock object
let mut mock = CoolTraitMock::new();

// Set up expectations for it
mock.expect_foo()
    .called_once();
mock.then()
    .expect_goop()
    .called_once()
    .with(true)
    .returning(|_| 5);

// Execute test code
m.foo();
assert_eq!(m.goop(true), 5);

// When the mock object is dropped, its expectations will be evaluated
```

See [`macros_high.rs`](https://github.com/pcsm/simulacrum/blob/master/simulacrum/examples/macros_high.rs) for a full run-through of the mock object user API.

## Creating Mock Objects

Simulacrum provides several APIs at different levels of abstraction, so you can create mock objects with the level of control you desire. All mock objects created with Simulacrum expose the same user API, no matter which API level is used to create them.

The following examples all show how to mock this trait:

```rust
trait CoolTrait {
    fn foo(&self);
    fn goop(&mut self, flag: bool) -> u32;
    fn store(&self, val: &i64);
}
```

Note that the macro API only supports creating mocks from a trait, while the manual API allows you to create mock objects to stand in for structs as well.

### High-Level Macro Example

The `create_mock!` macro creates a mock object from a trait. Just copy over the trait's interface and annotate it:

```rust
#[macro_use]
extern crate simulacrum;

create_mock! {
    impl CoolTrait for CoolTraitMock (self) {
        expect_foo("foo"):
        fn foo(&self);

        expect_goop("goop"):
        fn goop(&mut self, flag: bool) -> u32;

        // & params are mocked as *const and &mut are mocked as *mut.
        expect_store("store"):
        fn store(&self, val: &i64);
    }
}
```

See [`macros_high.rs`](https://github.com/pcsm/simulacrum/blob/master/simulacrum/examples/macros_high.rs) for more examples of how to mock out different types of methods with `create_mock!`.

### Mid-Level Macros Example

If you need more control than the high-level macro offers, you can use the `create_mock_struct!` and `was_called!` macros. This is useful if you'd like to create mock objects with features that the high-level macro doesn't support, like generic methods. Note that you can mix-and-match these macros with the manual interface as well.

```rust
#[macro_use]
extern crate simulacrum;

create_mock_struct! {
    struct CoolTraitMock: {
        expect_foo("foo");
        expect_goop("goop") bool => u32;
        // Note that we've used *const instead of & for shared references.
        expect_store("store") *const i64;
    }
}

impl CoolTrait for CoolTraitMock {
    fn foo(&self) {
        was_called!(self, "foo")
    }

    fn goop(&mut self, flag: bool) -> u32 {
        was_called!(self, "goop", (flag: bool) -> u32)
    }

    fn store(&self, val: &i64) {
        // Again note the use of *const instead of & for shared references.
        was_called!(self, "store", (val: *const i64))
    }
}

```

See [`macros_mid.rs`](https://github.com/pcsm/simulacrum/blob/master/simulacrum/examples/macros_mid.rs) for more examples of how to mock out different types of methods with the Mid-Level Macros.

## Manual Example

Create your mock objects manually for ultimate control. With this API, you can even create mocks to stand in for structs instead of traits. For a detailed example of the API, see the [`manual.rs`](https://github.com/pcsm/simulacrum/blob/master/simulacrum/examples/manual.rs) example.

```rust
extern crate simulacrum;

use simulacrum::mock::*;

trait CoolTrait {
    fn foo(&self);
    fn goop(&mut self, flag: bool) -> u32;
}

pub struct CoolTraitMock {
    e: Expectations
}

impl CoolTraitMock {
    pub fn new() -> Self {
        Self {
            e: Expectations::new()
        }
    }

    pub fn then(&mut self) -> &mut Self {
        self.e.then();
        self
    }

    pub fn expect_foo(&mut self) -> Method<(), ()> {
        self.e.expect::<(), ()>("foo")
    }

    pub fn expect_goop(&mut self) -> Method<bool, u32> {
        self.e.expect::<bool, u32>("goop")
    }
}

impl CoolTrait for CoolTraitMock {
    fn foo(&self) {
        self.e.was_called::<(), ()>("foo", ())
    }

    fn goop(&mut self, flag: bool) -> u32 {
        self.e.was_called_returning::<bool, u32>("goop", flag)
    }
}
```
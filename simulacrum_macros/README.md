Simulacrum Macros
==================================================================

These macros ease the creation of mock objects using [`simulacrum`](https://github.com/pcsm/simulacrum/tree/master/simulacrum). The objects 
created using these macros can be used just like objects created manually with
the [`simulacrum`](https://github.com/pcsm/simulacrum/tree/master/simulacrum) API.

To install, add this line to your Cargo.toml:

```toml
[dependencies]
simulacrum_macros = "0.1.0"
```

Note that this crate has not yet reached version 1.0, so the API may change drastically between releases.

## Examples

This crate contains macros that operate at two levels of abstraction. These 
examples will show you how to mock this trait:

```rust
trait CoolTrait {
    fn foo(&self);
    fn goop(&mut self, flag: bool) -> u32;
    fn store(&self, val: &i64);
}
```

### High-Level Macro

The `create_mock!` macro does it all for you! Just copy over your interface,
annotate it, and a mock object will be created and your trait impl'd for it.

```rust
use simulacrum_macros::*;

create_mock! {
    impl CoolTrait for CoolTraitMock (self) {
        expect_foo("foo"):
        fn foo(&self);

        expect_goop("goop"):
        fn goop(&mut self, flag: bool) -> u32;

        // & params are automatically mocked as *const and &mut are mocked as *mut.
        expect_store("store"):
        fn store(&self, val: &i64);
    }
}
```

See [`high_level.rs`](https://github.com/pcsm/simulacrum/blob/master/simulacrum_macros/examples/high_level.rs) for more examples of how to mock out different types of methods with `create_mock!`.

### Mid-Level Macros

If you need more control than the high-level macro offers, you can use the 
`create_mock_struct!` and `was_called!` macros to make creating macros a little
bit speedier.

```rust
use simulacrum::*;

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
        // Again note the use of *const
        was_called!(self, "store", (val: *const i64))
    }
}

```

See [`mid_level.rs`](https://github.com/pcsm/simulacrum/blob/master/simulacrum_macros/examples/mid_level.rs) for more examples of how to mock out different types of methods with the Mid-Level Macros.
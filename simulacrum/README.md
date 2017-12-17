Simulacrum
==================================================================

Simulacrum is a small library for creating mock objects by hand using stable Rust.

To install, add this line to your Cargo.toml:

```toml
[dependencies]
simulacrum = "0.1.0"
```

Note that this crate has not yet reached version 1.0, so the API may change drastically between releases.

## Example

You can use this crate to create your mock objects manually for ultimate control. For a
detailed example of the API, see the [`everything.rs`](https://github.com/pcsm/simulacrum/blob/master/simulacrum/examples/everything.rs) example.

If you'd like a to do the same mocking with less typing, you can use the macros in
[`simulacrum_macros`](https://github.com/pcsm/simulacrum/tree/master/simulacrum_macros).

```rust
extern crate simulacrum;

use simulacrum::*;

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

fn main() {
    // Set up expectations
    let mut m = CoolTraitMock::new();
    m.expect_foo().called_once();
    m.then().expect_goop().called_once().with(true).returning(|_| 5);

    // Execute test code
    m.foo();
    assert_eq!(m.goop(true), 5);

    // When the Expectations struct is dropped, each of its expectations will be evaluated
}

```
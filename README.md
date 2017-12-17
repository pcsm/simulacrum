Simulacrum
==================================================================

Simulacrum is a small library for creating mock objects in Rust.

It consists of two crates:

- [`simulacrum`](https://github.com/pcsm/simulacrum/tree/master/simulacrum) - The core API that can be used to build mock objects by hand in stable Rust.
- [`simulacrum_macros`](https://github.com/pcsm/simulacrum/tree/master/simulacrum_macros)- Macros that make it easier to create mock objects with the core API.

This repository also includes one in-progress crate that should not be used yet:

- [`simulacrum_auto`](https://github.com/pcsm/simulacrum/tree/master/simulacrum_auto) - A work-in-progress procedural macro that lets you automatically create mock objects (nightly Rust only).

Note that none of these crates have reached version 1.0 yet, so their APIs may change drastically between releases.
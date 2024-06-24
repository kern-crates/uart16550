# uart16550

[![CI](https://github.com/YdrMaster/uart16550/actions/workflows/workflow.yml/badge.svg?branch=main)](https://github.com/YdrMaster/uart16550/actions)
[![Latest version](https://img.shields.io/crates/v/uart16550.svg)](https://crates.io/crates/uart16550)
[![issue](https://img.shields.io/github/issues/YdrMaster/uart16550)](https://github.com/YdrMaster/uart16550/issues)
[![Documentation](https://docs.rs/uart16550/badge.svg)](https://docs.rs/uart16550)
![license](https://img.shields.io/github/license/YdrMaster/uart16550)

[Design(Chinese)](https://github.com/YdrMaster/awesome-device)

Provide definition of 16550 uart registers.

This crate is ported from [uart16550](https://github.com/YdrMaster/uart16550.git) but with no unsafe code.


## Usage
```rust
#[derive(Debug, Clone)]
pub struct SafeIORegion {
    range: Range<PhysAddr>,
}
impl Uart16550IO<u8> for SafeIORegion {
    fn read_at(&self, offset: usize) -> u8 {}

    fn write_at(&self, offset: usize, value: u8) {}
}

fn main() {
    let io_region = SafeIORegion::new();
    let uart = Uart16550::new(Box::new(io_region));
}
```

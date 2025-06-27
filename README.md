# Drake Copper

A Rust project integrating [Copper](https://github.com/sonos/copper) for real-time control and [Totsu](https://github.com/convex-optimization/totsu) for convex optimization.

## Features

- Real-time control with Copper
- Convex optimization using Totsu
- Written in safe, modern Rust

## Build Instructions

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)
- `cargo` package manager

### Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
copper = "0.7"
totsu = "0.7"
```

### Build

```sh
cargo build --release
```

### Run

```sh
cargo run --release
```

## Example Usage

```rust
use copper::prelude::*;
use totsu::prelude::*;

// Your real-time control and optimization logic here
```

## License

MIT
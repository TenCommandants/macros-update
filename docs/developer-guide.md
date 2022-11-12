# Developer Guide

## Setup

1. Setup the Rust toolchain. See [the Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html) for details.
2. Setup neo4j (via default port 7681). See [the neo4j documentation](https://neo4j.com/docs/operations-manual/current/installation/) for details.
3. Setup Etcd. See [the etcd documentation](https://etcd.io/docs/latest/install/) for details.
4. Build and test
    - `cargo build`
    - `cargo test`
    - `RUST_LOG=info cargo run --bin demo`

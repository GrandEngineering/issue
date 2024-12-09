# Rust Lang Sync Arc Deref Issue
To Recreate the the experiment follow.

```bash
rustup default 1.82
cargo build
cargo run
```
You will see that it runs fine.
Now to see the segfault.

```bash
rustup default 1.83
cargo build
cargo run
```

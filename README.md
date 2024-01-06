# async-trait playground

## Context

Rust 1.75 stabilized impl trait in return positions of trait functions. That means it's possible to use specific async trait implementations depending on static feature flags. Before that the `async-trait` macro had to be used for async trait fn's, which used dynamic dispatch in the background.

A trait like this can be defined and implemented now:

```rust
trait Frobnicate {
    async fn frobnicate(&self) -> &str;
}
```

If we require further trait bounds (e.g we have to add `Send` b/c we want to call the fn in a multithreaded async runtime) we can desuger the async expression and add the bound:

```rust
trait Frobnicate {
    fn frobnicate(&self) -> impl Future<Output = &str> + Send;
}
```

### Sample

```bash
cargo r --features=bar -- ohai &
curl http://localhost:3000
ohai
```

```bash
cargo r -- notfoo &
curl http://localhost:3000
foo
```


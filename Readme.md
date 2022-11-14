Fast Walker with Multimodal
===========================

Multi-threaded walker in directory, with async support.

> Whether multi-threaded in async mode depends on whether the runtime is multi-threaded.

## Fast Usage

```toml
fast-walker = "*"
```

```rust
use fast_walker::WalkPlan;

#[test]
fn list_depth_less2() {
    let plan = WalkPlan::new(root)
        .reject_if(|path, depth| depth > 2 || path.starts_with("."));
    for item in plan.into_iter().take(10) {
        println!("{:?}", item);
    }
}
```




```rust
let plan = WalkPlan::new(root)
    .reject_if(|path, _| path.starts_with("."));
for item in plan.into_iter().take(10) {
    println!("{:?}", item);
}
```
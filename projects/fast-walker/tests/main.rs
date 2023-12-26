use fast_walker::WalkPlan;
use std::path::Path;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let root = "C:\\P4Root";
    let plan = WalkPlan::new(root).reject_if(|item| item.path.starts_with("."));
    for item in plan.into_iter().take(10) {
        println!("{:?}", item);
    }
}

#[test]
fn test_reverse() {
    let root = Path::new("./").canonicalize().unwrap();
    let plan = WalkPlan::new(root);
    for item in plan.ancestors() {
        println!("{:?}", item);
    }
}

use fast_walker::WalkPlan;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let root = "C:\\P4Root";
    let plan = WalkPlan::new(root).reject_if(|path, _| path.starts_with("."));
    for item in plan.into_iter().take(10) {
        println!("{:?}", item);
    }
}

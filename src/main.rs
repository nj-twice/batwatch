use battery;

fn main() {
    let manager = battery::Manager::new().unwrap();

    // Assume this is a laptop and there's only one single battery
    assert_eq!(manager.batteries().iter().count(), 1)
}

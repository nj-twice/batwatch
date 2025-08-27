use std::{thread, time::Duration};

const THREE_MINUTES: u64 = 180;
const THIRTY_PERCENT: f32 = 0.3;
const NINETY_PERCENT: f32 = 0.9;
const ONE_SECOND: u64 = 1;

fn main() {
    let manager = battery::Manager::new().unwrap();

    // Assume this is a laptop and there's only one single battery
    assert_eq!(manager.batteries().iter().count(), 1);

    let mut my_battery = manager.batteries().unwrap().nth(0).unwrap().unwrap();

    loop {
        manager.refresh(&mut my_battery).unwrap();

        let mut wait_time = Duration::from_secs(THREE_MINUTES);

        if my_battery.state() == battery::State::Discharging
            && my_battery.state_of_charge().value <= THIRTY_PERCENT
        {
            wait_time = Duration::from_secs(ONE_SECOND);

            // TODO: brightness blink
        } else if my_battery.state() == battery::State::Charging
            && my_battery.state_of_charge().value >= NINETY_PERCENT
            || my_battery.state() == battery::State::Full
        {
            wait_time = Duration::from_secs(ONE_SECOND);

            // TODO: brightness slow pulse
        };

        thread::sleep(wait_time);
    }
}

use brightness::blocking::{self as bb, Brightness};
use std::{thread, time::Duration};

const THREE_MINUTES: u64 = 180;
const THIRTY_PERCENT: f32 = 0.3;
const NINETY_PERCENT: f32 = 0.9;
const ONE_SECOND: u64 = 1;
const SLOW_PULSE_TIME_INCR: u64 = 10;

fn main() {
    let manager = battery::Manager::new().unwrap();

    // Assume this is a laptop and there's only one battery
    assert_eq!(manager.batteries().iter().count(), 1);

    // Again, assume a single screen
    assert_eq!(bb::brightness_devices().count(), 1);

    let mut my_battery = manager.batteries().unwrap().nth(0).unwrap().unwrap();
    let my_screen = bb::brightness_devices().nth(0).unwrap().unwrap();

    loop {
        manager.refresh(&mut my_battery).unwrap();

        let mut wait_time = Duration::from_secs(THREE_MINUTES);

        if my_battery.state() == battery::State::Discharging
            && my_battery.state_of_charge().value <= THIRTY_PERCENT
        {
            wait_time = Duration::from_secs(ONE_SECOND);

            // Blink
            for _ in 1..=5 {
                my_screen.set(5).unwrap();
                thread::sleep(Duration::from_millis(250));
                my_screen.set(75).unwrap();
                thread::sleep(Duration::from_millis(250));
            }
        } else if my_battery.state() == battery::State::Charging
            && my_battery.state_of_charge().value >= NINETY_PERCENT
            || my_battery.state() == battery::State::Full
        {
            wait_time = Duration::from_secs(ONE_SECOND);

            // Slow pulse
            for percentage in (5..=100).filter(|x| x % 5 == 0) {
                my_screen.set(percentage).unwrap();
                thread::sleep(Duration::from_millis(SLOW_PULSE_TIME_INCR));
            }

            for percentage in (5..=100).rev().filter(|x| x % 5 == 0) {
                my_screen.set(percentage).unwrap();
                thread::sleep(Duration::from_millis(SLOW_PULSE_TIME_INCR));
            }
        };

        thread::sleep(wait_time);
    }
}

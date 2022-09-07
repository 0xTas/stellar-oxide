use std::io;
use std::io::Write;
use clearscreen;
use rand::Rng;
use rand::thread_rng as rng;
use std::time::{self, Duration};
use stellar_bodies::stars::Star;
use stellar_bodies::planets::Planet;

pub mod stellar_bodies;

pub fn create_random_star() -> Star<'static> {
    // TODO: source random names and subtypes
    Star::new("Lorem Ipsum", "surprise me", None)
}

pub fn create_random_planet() -> Planet<'static> {
    Planet::new("Lorem Ipsum 1", "random")
}

/* Utility Functions */
pub fn flush() {
    io::stdout().flush().expect("Write to console failed!");
}

pub fn cls() {
    clearscreen::clear().unwrap();
}

pub fn dur(amount: u32) -> Duration {
    time::Duration::from_millis(amount.into())
}

pub fn input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from Stdin!");

    user_input
}

pub fn decide_ringed() -> bool {
    let chance = rng().gen_range(0..=1337);
    if chance == 1337 || chance == 420 || chance == 69 {
        return true;
    }else if chance < 7 {
        return true;
    }else {
        return false;
    };
}

pub fn is_landable(pressure: f64, temp: i32) -> bool {
    if pressure <= 4.20 && temp <= 666 {
        return true;
    }else {
        return false;
    };
}

pub fn is_explorable(pressure: f64, temp: i32, gravity: f64) -> bool {
    if pressure <= 2.25 && temp <= 370 && gravity <= 4.20 {
        return true;
    }else {
        return false;
    };
}
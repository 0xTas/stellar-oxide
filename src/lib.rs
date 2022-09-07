use std::io;
use std::io::Write;
use clearscreen;
use std::time::{self, Duration};
use stellar_bodies::stars::Star;

pub mod stellar_bodies;

pub fn create_random_star() -> Star<'static> {
    // TODO: source random names and subtypes
    Star::new("Lorem Ipsum", "surprise me", None)
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
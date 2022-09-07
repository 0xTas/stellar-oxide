use std::io;
use std::io::Write;
use std::time::{self, Duration};
use stellar_bodies::stars::Star;

pub mod stellar_bodies;

pub fn create_random_star() -> Star<'static> {
    Star::new("Lorem Ipsum", "surprise me", None)
}

/* Utility Functions */
pub fn flush() {
    io::stdout().flush().expect("Write to console failed!");
}

pub fn cls() {
    for _ in 0..420 {
        println!("");
    };
}

pub fn dur(amount: u32) -> Duration {
    time::Duration::from_millis(amount.into())
}
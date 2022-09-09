use std::io;
use std::io::Write;
use clearscreen;
use std::time::Duration;
use stellar_bodies::stars::Star;
use stellar_bodies::planets::Planet;

pub mod stellar_bodies;


#[derive(Debug)]
pub enum Rarity {
    VeryCommon,
    Common,
    Uncommon,
    Rare,
    VeryRare,
    ExtremelyRare,
    Legendary,
}

impl Rarity {
    pub fn new(rarity: &str) -> Self {
        match rarity {
            "VC" => Self::VeryCommon,
            "C" => Self::Common,
            "UC" => Self::Uncommon,
            "R" => Self::Rare,
            "VR" => Self::VeryRare,
            "ER" => Self::ExtremelyRare,
            "L" => Self::Legendary,
            _ => panic!("Invalid Rarity!"),
        }
    }

    pub fn fetch_rarity(&self) -> &str {
        match self {
            Rarity::VeryCommon => "Very Common",
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Rare => "Rare",
            Rarity::VeryRare => "Very Rare",
            Rarity::ExtremelyRare => "Extremely Rare",
            Rarity::Legendary => "Legendary",
        }
    }
}


pub fn create_random_star() -> Star<'static> {
    // TODO: source random names and subtypes
    Star::new("Lorem Ipsum", "surprise me")
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
    Duration::from_millis(amount.into())
}

pub fn input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from Stdin!");

    user_input
}
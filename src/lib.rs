use std::io;
use std::io::Write;
use rand::Rng;
use clearscreen;
use std::time::Duration;
use std::thread::sleep;
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
    /// Returns a new instance of the *Rarity* enum based on an identifier *&str*.
    /// 
    /// **Valid Identifiers:**
    /// [VC, C, UC, R, VR, ER, L]
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

    /// Returns a stringified representation of the calling *Rarity* enum.
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


/// Returns a randomly-initialized instance of the *Star* struct.
pub fn create_random_star() -> Star<'static> {
    let name: String = format!("EC_Entry_{:#02x}", rand::thread_rng().gen_range(0x00..=0xffffff));
    Star::new(name, "random")
}

/// Returns a randomly-initialized instance of the *Planet* struct.
pub fn create_random_planet() -> Planet<'static> {
    let name: String = format!("planet_{:#02x}", rand::thread_rng().gen_range(0x00..=0xffffff));
    Planet::new(name, "random")
}


/* Utility Functions */

/// Flushes stdout, panics upon failure.
pub fn flush() {
    io::stdout().flush().expect("Write to console failed!");
}

/// Flushes stdout, and then sleeps for the provided time in miliseconds
/// **(panics upon failure)**.
pub fn wait(duration: u32) {
    flush();
    let duration: Duration = Duration::from_millis(duration.into());
    sleep(duration);
}

/// Clears the console screen. Panics upon failure.
pub fn cls() {
    clearscreen::clear().unwrap();
}

/// Prompts the user for input and returns it as a *String*. Panics upon failure.
pub fn input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from Stdin!");

    user_input
}
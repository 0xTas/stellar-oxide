use std::time::Duration;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub struct PlanetTypeProperties<'a> {
    pub type_name: &'a str,
    pub description: &'a str,
    pub is_moon: bool,
    pub low_temp: i32, // Kelvin
    pub high_temp: i32,
    pub dist_from_arrival: u32, // Light-Seconds
    pub earth_masses: u32,
    pub gravity: u32,
    pub orbital_period: Duration,
    pub rotational_period: Duration,
    pub radius: f64, // Kilometers
    pub ringed: bool,
    pub surface_pressure: u32, // Factors of Earth's Atmosphere
    pub surface_temp: i32, // Kelvin
}


#[derive(Debug)]
pub enum PlanetType<'a> {
    AW(PlanetTypeProperties<'a>),
    WW(PlanetTypeProperties<'a>),
    WG(PlanetTypeProperties<'a>),
    RKB(PlanetTypeProperties<'a>),
    ICB(PlanetTypeProperties<'a>),
    ELW(PlanetTypeProperties<'a>),
    HMC(PlanetTypeProperties<'a>),
    RIW(PlanetTypeProperties<'a>),
    MRB(PlanetTypeProperties<'a>),
    HGG(PlanetTypeProperties<'a>),
    GGG(PlanetTypeProperties<'a>),
    CIGG(PlanetTypeProperties<'a>),
    CIIGG(PlanetTypeProperties<'a>),
    CIIIGG(PlanetTypeProperties<'a>),
    CIVGG(PlanetTypeProperties<'a>),
    CVGG(PlanetTypeProperties<'a>),
    HRGG(PlanetTypeProperties<'a>),
    GGWABL(PlanetTypeProperties<'a>),
    GGWWBL(PlanetTypeProperties<'a>),
}

impl<'a> Distribution<PlanetType<'a>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PlanetType<'a> {
        match rng.gen_range(0..19) {
            0 => PlanetType::AW(())
        }
    }
}

impl<'a> PlanetType<'a> {

    /* PlanetType statistics taken from https://edastro.com/records */
    pub fn new(planet_type: &str) -> Self {
        match planet_type {
            "AW" => {
                const MIN_TEMP: i32 = 27;
                const MAX_TEMP: i32 = 409;
                let surface_temp: i32 = rand::thread_rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_RADIUS: f64 = 2699.66775;
                const MAX_RADIUS: f64 = 30741.622;
                let radius: f64 = rand::thread_rng().gen_range(MIN_RADIUS..=MAX_RADIUS);
            }
        }
    }
}
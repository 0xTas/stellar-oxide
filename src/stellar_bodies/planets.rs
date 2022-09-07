use std::time::Duration;
use crate::decide_ringed;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
    thread_rng as rng,
};


#[derive(Debug)]
pub struct PlanetTypeProperties<'a> {
    pub ringed: bool,
    pub type_name: &'a str,
    pub description: &'a str,
    pub rarity: &'a str,
    pub dist_from_arrival: f64, // Light-Seconds
    pub low_temp: i32,
    pub high_temp: i32,
    pub surface_temp: i32, // Kelvin
    pub surface_pressure: f64, // Factors of Earth's Atmosphere
    pub radius: f64, // Kilometers
    pub earth_masses: f64,
    pub gravity: f64, // Factors of Earth's Gravity
    pub orbital_period: Duration,
    pub rotational_period: Duration,
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
            "AW" => { // Ammonia World
                let ringed: bool = decide_ringed();

                let type_name: &str;
                if ringed {
                    type_name = "Ringed Ammonia World";
                }else {
                    type_name = "Ammonia World";
                };

                let description: &str;
                if ringed {
                    description = "Terrestrial ammonia world";
                }else {
                    description = "TODO";
                };

                let rarity: &str = "Very Rare";

                const MIN_DIST: f64 = 7.0;
                const MAX_DIST: f64 = 81_7190.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_TEMP: i32 = 27;
                const MAX_TEMP: i32 = 409;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 4_983_498.11683198;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_RADIUS: f64 = 2_699.66775;
                const MAX_RADIUS: f64 = 30_741.622;
                let radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_MASSES: f64 = 0.07346;
                const MAX_MASSES: f64 = 1_327.610718;
                let earth_masses: f64 = rng().gen_range(MIN_MASSES..=MAX_MASSES);

                const MIN_GRAVITY: f64 = 0.249672518138;
                const MAX_GRAVITY: f64 = 91.80329702145;
                let gravity: f64 = rng().gen_range(MIN_GRAVITY..=MAX_GRAVITY);

                const MIN_ORBITAL: f64 = 0.258450279086;
                const MAX_ORBITAL: f64 = 747_992.070736713;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.211842411736;
                const MAX_ROTATIONAL: f64 = 4_442.4380215662;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::AW(PlanetTypeProperties {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    dist_from_arrival,
                    low_temp: MIN_TEMP,
                    high_temp: MAX_TEMP,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "WW" => { // Water World
                let ringed: bool = decide_ringed();

                let type_name: &str;
                if ringed {
                    type_name = "Ringed Water World";
                }else {
                    type_name = "Water World";
                };

                let description: &str;
                if ringed {
                    description = "TODO";
                }else {
                    description = "TODO";
                };

                let rarity: &str = "Rare";

                const MIN_DIST: f64 = 3.0;
                const MAX_DIST: f64 = 4_217_470.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_TEMP: i32 = 150;
                const MAX_TEMP: i32 = 902;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_PRESSURE: f64 = 0.07; 
                const MAX_PRESSURE: f64 = 6_319_180.5;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_RADIUS: f64 = 2_640.894;
                const MAX_RADIUS: f64 = 29_011.342;
                let radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_MASSES: f64 = 0.0687;
                const MAX_MASSES: f64 = 741.438171;
                let earth_masses: f64 = rng().gen_range(MIN_MASSES..=MAX_MASSES);

                const MIN_GRAVITY: f64 = 0.250068247718;
                const MAX_GRAVITY: f64 = 46.013426658406;
                let gravity: f64 = rng().gen_range(MIN_GRAVITY..=MAX_GRAVITY);

                const MIN_ORBITAL: f64 = 0.003370370512;
                const MAX_ORBITAL: f64 = 570_992.687407407;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.124829452037; // Tidal Egg
                const MAX_ROTATIONAL: f64 = 71_900.1459814641;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::WW(PlanetTypeProperties {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    dist_from_arrival,
                    low_temp: MIN_TEMP,
                    high_temp: MAX_TEMP,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "WG" => { // Water Giant
                let ringed: bool = decide_ringed();

                let type_name: &str;
                if ringed {
                    type_name = "Ringed Water Giant";
                }else {
                    type_name = "Water Giant";
                };

                let description: &str;
                if ringed {
                    description = "CHANGE ME (WITH RINGS)";
                }else {
                    description = "CHANGE ME";
                };

                let rarity: &str = "Very Rare";

                const MIN_DIST: f64 = 21.0;
                const MAX_DIST: f64 = 690_129.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_TEMP: i32 = 136;
                const MAX_TEMP: i32 = 2_715;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_PRESSURE: f64 = 1337.4206969;
                const MAX_PRESSURE: f64 = 29_501_937_664.0;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_RADIUS: f64 = 15_892.973;
                const MAX_RADIUS: f64 = 30_942.572;
                let radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_MASSES: f64 = 17.23122;
                const MAX_MASSES: f64 = 1_961.928589;
                let earth_masses: f64 = rng().gen_range(MIN_MASSES..=MAX_MASSES);

                const MIN_GRAVITY: f64 = 2.298430556431;
                const MAX_GRAVITY: f64 = 193.710734791427;
                let gravity: f64 = rng().gen_range(MIN_GRAVITY..=MAX_GRAVITY);

                const MIN_ORBITAL: f64 = 0.780787489155;
                const MAX_ORBITAL: f64 = 38_728.2255623079;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.159246328854;
                const MAX_ROTATIONAL: f64 = 3_489.59481481481;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::WG(PlanetTypeProperties {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    dist_from_arrival,
                    low_temp: MIN_TEMP,
                    high_temp: MAX_TEMP,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "RKB" => {
                let ringed: bool = decide_ringed();

                let type_name: &str;
                if ringed {
                    type_name = "Ringed Rocky Body";
                }else {
                    type_name = "Rocky Body";
                };

                let description: &str;
                if ringed {
                    description = "CHANGE ME (WITH RINGS)";
                }else {
                    description = "CHANGE ME";
                };

                let rarity: &str = "Very Common";

                const MIN_DIST: f64 = 3.0;
                const MAX_DIST: f64 = 7_492_300.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_TEMP: i32 = 20;
                const MAX_TEMP: i32 = 51_171;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 2_516_369_920.0;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_RADIUS: f64 = 181.887875;
                const MAX_RADIUS: f64 = 21_765.112;
                let radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_MASSES: f64 = 0.0001;
                const MAX_MASSES: f64 = 527.839539;
                let earth_masses: f64 = rng().gen_range(MIN_MASSES..=MAX_MASSES);

                const MIN_GRAVITY: f64 = 0.007895669291;
                const MAX_GRAVITY: f64 = 50.039830862644;
                let gravity: f64 = rng().gen_range(MIN_GRAVITY..=MAX_GRAVITY);

                const MIN_ORBITAL: f64 = 0.001000000046;
                const MAX_ORBITAL: f64 = 12_163.6164409143;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.100663452148;
                const MAX_ROTATIONAL: f64 = 166_276.93037037;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::RKB(PlanetTypeProperties {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    dist_from_arrival,
                    low_temp: MIN_TEMP,
                    high_temp: MAX_TEMP,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "ICB" => {
                let ringed: bool = decide_ringed();

                let type_name: &str;
                if ringed {
                    type_name = "Ringed Icy Body";
                }else {
                    type_name = "Icy Body";
                };

                let description: &str;
                if ringed {
                    description = "CHANGE ME (WITH RINGS)";
                }else {
                    description = "CHANGE ME";
                };

                let rarity: &str = "Very Common";

                const MIN_DIST: f64 = 1.37026;
                const MAX_DIST: f64 = 15_653_000.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_TEMP: i32 = 1;
                const MAX_TEMP: i32 = 4_020;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 204_413_011.941219;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_RADIUS: f64 = 160.0;
                const MAX_RADIUS: f64 = 31_232.91;
                let radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_MASSES: f64 = 0.0001;
                const MAX_MASSES: f64 = 2_214.019287;
                let earth_masses: f64 = rng().gen_range(MIN_MASSES..=MAX_MASSES);

                const MIN_GRAVITY: f64 = 0.004758505708;
                const MAX_GRAVITY: f64 = 236.648152852392;
                let gravity: f64 = rng().gen_range(MIN_GRAVITY..=MAX_GRAVITY);

                const MIN_ORBITAL: f64 = 0.000104166667;
                const MAX_ORBITAL: f64 = 1_257_206_278.81862;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.081735393383;
                const MAX_ROTATIONAL: f64 = 2_479_320.4190602;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::CHANGEME(PlanetTypeProperties {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    dist_from_arrival,
                    low_temp: MIN_TEMP,
                    high_temp: MAX_TEMP,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            }
            _ => {

                /* Template for efficiency purposes */

                // let ringed: bool = decide_ringed();

                // let type_name: &str;
                // if ringed {
                //     type_name = "CHANGE ME (WITH RINGS)";
                // }else {
                //     type_name = "CHANGE ME";
                // };

                // let description: &str;
                // if ringed {
                //     description = "CHANGE ME (WITH RINGS)";
                // }else {
                //     description = "CHANGE ME";
                // };

                // let rarity: &str = "CHANGE ME";

                // const MIN_DIST: u32 = 
                // const MAX_DIST: u32 = 
                // let dist_from_arrival: u32 = rng().gen_range(MIN_DIST..=MAX_DIST);

                // const MIN_TEMP: i32 = 
                // const MAX_TEMP: i32 = 
                // let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                // const MIN_PRESSURE: f64 = 
                // const MAX_PRESSURE: f64 = 
                // let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                // const MIN_RADIUS: f64 = 
                // const MAX_RADIUS: f64 = 
                // let radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                // const MIN_MASSES: f64 = 
                // const MAX_MASSES: f64 = 
                // let earth_masses: f64 = rng().gen_range(MIN_MASSES..=MAX_MASSES);

                // const MIN_GRAVITY: f64 = 
                // const MAX_GRAVITY: f64 = 
                // let gravity: f64 = rng().gen_range(MIN_GRAVITY..=MAX_GRAVITY);

                // const MIN_ORBITAL: f64 = 
                // const MAX_ORBITAL: f64 = 
                // let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                // let orbital_secs: f64 = orbital_range * 86400.0;
                // let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                // const MIN_ROTATIONAL: f64 = 
                // const MAX_ROTATIONAL: f64 = 
                // let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                // let rotational_secs: f64 = rotational_range * 86400.0;
                // let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                // return Self::CHANGEME(PlanetTypeProperties {
                //     ringed,
                //     type_name,
                //     description,
                //     rarity,
                //     dist_from_arrival,
                //     low_temp: MIN_TEMP,
                //     high_temp: MAX_TEMP,
                //     surface_temp,
                //     surface_pressure,
                //     radius,
                //     earth_masses,
                //     gravity,
                //     orbital_period,
                //     rotational_period,
                // });
            }
        }
    }
}
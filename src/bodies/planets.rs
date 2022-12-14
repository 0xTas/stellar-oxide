use std::time::Duration;
use crate::Rarity;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
    thread_rng as rng,
};


#[derive(Debug)]
pub struct PlanetTypeProperties<'a> {
    pub type_label: &'a str,
    pub type_name: &'a str,
    pub description: &'a str,
    pub rarity: Rarity,
    pub ringed: bool,
    pub landable: bool,
    pub explorable: bool,
    pub dist_from_arrival: f64, // Light-Seconds
    pub surface_temp: f64, // Kelvin
    pub surface_pressure: f64, // Factors of Earth's Atmosphere
    pub radius: f64, // Kilometers
    pub earth_masses: f64,
    pub gravity: f64, // Factors of Earth's Gravity
    pub orbital_period: Duration,
    pub rotational_period: Duration,
}

#[derive(Debug)]
pub struct Stats<'a> {
    pub label: &'a str,
    pub type_name: &'a str,
    pub description: &'a str,
    pub rarity: &'a str,
    pub ringed: bool,
    pub landable: bool,
    pub explorable: bool,
    pub dist_from_arrival: f64,
    pub surface_temp: f64,
    pub surface_pressure: f64,
    pub radius: f64,
    pub earth_masses: f64,
    pub gravity: f64,
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
    GGGG(PlanetTypeProperties<'a>),
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

    // TODO: Implement random PlanetType picker weighted by rarity instead of linear rng
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PlanetType<'a> {
        match rng.gen_range(0..=18) {
            0 => PlanetType::new("AW"),
            1 => PlanetType::new("WW"),
            2 => PlanetType::new("WG"),
            3 => PlanetType::new("RKB"),
            4 => PlanetType::new("ICB"),
            5 => PlanetType::new("ELW"),
            6 => PlanetType::new("HMC"),
            7 => PlanetType::new("RIW"),
            8 => PlanetType::new("MRB"),
            9 => PlanetType::new("HGG"),
            10 => PlanetType::new("GGGG"),
            11 => PlanetType::new("CIGG"),
            12 => PlanetType::new("CIIGG"),
            13 => PlanetType::new("CIIIGG"),
            14 => PlanetType::new("CIVGG"),
            15 => PlanetType::new("CVGG"),
            16 => PlanetType::new("HRGG"),
            17 => PlanetType::new("GGWABL"),
            18 => PlanetType::new("GGWWBL"),
            _ => panic!("Invalid PlanetType!"),
        }
    }
}

impl<'a> PlanetType<'a> {

    /// Returns a new instance of the *PlanetType* enum with randomly-initialized values.
    /// Accepts a type identifier as a &str, but will return a random *PlanetType* if an invalid identifier is provided.
    /// 
    /// **Valid Identifiers:**
    /// [AW, WW, WG, RKB, ICB, ELW, HMC, RIW, MRB, HGG, GGGG, CIGG, CIIGG, CIIIGG, CIVGG, CVGG, HRGG, GGWABL, GGWWBL]
    pub fn new(planet_type: &str) -> Self {

        // TODO: Calculate stat numbers with a weighted bias towards the middle.
        // TODO: Calculate ring probability based on planet type.
        match planet_type {
            "AW" => { // Ammonia World
                let ringed: bool = rng().gen_bool(1.0 / 42.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Ammonia World";
                    description = "Terrestrial ammonia world";
                    type_label = "AW(R)";
                }else {
                    type_name = "Ammonia World";
                    description = "TODO";
                    type_label = "AW";
                };

                let rarity: Rarity = Rarity::VeryRare;
                
                const MIN_DIST: f64 = 7.0;
                const MAX_DIST: f64 = 81_7190.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 4_983_498.11683198;
                let rel_percentage_pressure: f64 = random_relative_percentage(MIN_PRESSURE, MAX_PRESSURE);
                let surface_pressure: f64 = val_from_relative_percentage(MIN_PRESSURE, MAX_PRESSURE, rel_percentage_pressure);

                const MIN_TEMP: f64 = 27.0;
                const MAX_TEMP: f64 = 409.0;
                let surface_temp: f64 = val_from_relative_percentage(MIN_TEMP, MAX_TEMP, rel_percentage_pressure);

                const MIN_MASSES: f64 = 0.07346;
                const MAX_MASSES: f64 = 1_327.610718;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 2_699.66775;
                const MAX_RADIUS: f64 = 30_741.622;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.249672518138;
                const MAX_GRAVITY: f64 = 91.80329702145;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

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

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::AW(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
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
                let ringed: bool = rng().gen_bool(1.0 / 42.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Water World";
                    description = "TODO";
                    type_label = "WW(R)";
                }else {
                    type_name = "Water World";
                    description = "TODO";
                    type_label = "WW";
                };

                let rarity: Rarity = Rarity::Rare;

                const MIN_DIST: f64 = 3.0;
                const MAX_DIST: f64 = 4_217_470.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.07; 
                const MAX_PRESSURE: f64 = 6_319_180.5;
                let rel_percentage_pressure: f64 = random_relative_percentage(MIN_PRESSURE, MAX_PRESSURE);
                let surface_pressure: f64 = val_from_relative_percentage(MIN_PRESSURE, MAX_PRESSURE, rel_percentage_pressure);

                const MIN_TEMP: f64 = 150.0;
                const MAX_TEMP: f64 = 902.0;
                let surface_temp: f64 = val_from_relative_percentage(MIN_TEMP, MAX_TEMP, rel_percentage_pressure);

                const MIN_MASSES: f64 = 0.0687;
                const MAX_MASSES: f64 = 741.438171;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 2_640.894;
                const MAX_RADIUS: f64 = 29_011.342;
                let radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

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

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::WW(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
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
                let ringed: bool = rng().gen_bool(1.0 / 15.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Water Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "WG(R)";
                }else {
                    type_name = "Water Giant";
                    description = "CHANGE ME";
                    type_label = "WG";
                };

                let rarity: Rarity = Rarity::VeryRare;

                const MIN_DIST: f64 = 21.0;
                const MAX_DIST: f64 = 690_129.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 1337.4206969;
                const MAX_PRESSURE: f64 = 29_501_937_664.0;
                let rel_percentage_pressure: f64 = random_relative_percentage(MIN_PRESSURE, MAX_PRESSURE);
                let surface_pressure: f64 = val_from_relative_percentage(MIN_PRESSURE, MAX_PRESSURE, rel_percentage_pressure);

                const MIN_TEMP: f64 = 136.0;
                const MAX_TEMP: f64 = 2_715.0;
                let surface_temp: f64 = val_from_relative_percentage(MIN_TEMP, MAX_TEMP, rel_percentage_pressure);

                const MIN_MASSES: f64 = 17.23122;
                const MAX_MASSES: f64 = 1_961.928589;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 15_892.973;
                const MAX_RADIUS: f64 = 30_942.572;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 2.298430556431;
                const MAX_GRAVITY: f64 = 193.710734791427;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

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

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::WG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "RKB" => { // Rocky Body
                let ringed: bool = rng().gen_bool(1.0 / 20.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Rocky Body";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "RKB(R)";
                }else {
                    type_name = "Rocky Body";
                    description = "CHANGE ME";
                    type_label = "RKB";
                };

                let rarity: Rarity = Rarity::VeryCommon;

                const MIN_DIST: f64 = 3.0;
                const MAX_DIST: f64 = 7_492_300.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 2_516_369_920.0;
                let rel_percentage_pressure: f64 = random_relative_percentage(MIN_PRESSURE, MAX_PRESSURE);
                let surface_pressure: f64 = val_from_relative_percentage(MIN_PRESSURE, MAX_PRESSURE, rel_percentage_pressure);

                const MIN_TEMP: f64 = 20.0;
                const MAX_TEMP: f64 = 51_171.0;
                let surface_temp: f64 = val_from_relative_percentage(MIN_TEMP, MAX_TEMP, rel_percentage_pressure);

                const MIN_MASSES: f64 = 0.0001;
                const MAX_MASSES: f64 = 527.839539;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 181.887875;
                const MAX_RADIUS: f64 = 21_765.112;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.007895669291;
                const MAX_GRAVITY: f64 = 50.039830862644;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

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

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::RKB(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "ICB" => { // Icy Body
                let ringed: bool = rng().gen_bool(1.0 / 20.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Icy Body";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "ICB(R)";
                }else {
                    type_name = "Icy Body";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "ICB";
                };

                let rarity: Rarity = Rarity::VeryCommon;

                const MIN_DIST: f64 = 1.37026;
                const MAX_DIST: f64 = 15_653_000.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 204_413_011.941219;
                let rel_percentage_pressure: f64 = random_relative_percentage(MIN_PRESSURE, MAX_PRESSURE);
                let surface_pressure: f64 = val_from_relative_percentage(MIN_PRESSURE, MAX_PRESSURE, rel_percentage_pressure);

                const MIN_TEMP: f64 = 1.0;
                const MAX_TEMP: f64 = 4_020.0;
                let surface_temp: f64 = val_from_relative_percentage(MIN_TEMP, MAX_TEMP, rel_percentage_pressure);

                const MIN_MASSES: f64 = 0.0001;
                const MAX_MASSES: f64 = 2_214.019287;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 160.0;
                const MAX_RADIUS: f64 = 31_232.91;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.004758505708;
                const MAX_GRAVITY: f64 = 236.648152852392;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

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

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::ICB(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "ELW" => { // Earth-like World
                let ringed: bool = rng().gen_bool(1.0 / 420.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Earth-like World";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "ELW(R)";
                }else {
                    type_name = "Earth-like World";
                    description = "CHANGE ME";
                    type_label = "ELW";
                };

                let rarity: Rarity = Rarity::ExtremelyRare;

                const MIN_DIST: f64 = 6.0;
                const MAX_DIST: f64 = 736_306.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.24206969;
                const MAX_PRESSURE: f64 = 7.291643844066;
                let rel_percentage_pressure: f64 = random_relative_percentage(MIN_PRESSURE, MAX_PRESSURE);
                let surface_pressure: f64 = val_from_relative_percentage(MIN_PRESSURE, MAX_PRESSURE, rel_percentage_pressure);

                const MIN_TEMP: f64 = 260.0;
                const MAX_TEMP: f64 = 497.0;
                let surface_temp: f64 = val_from_relative_percentage(MIN_TEMP, MAX_TEMP, rel_percentage_pressure);

                const MIN_MASSES: f64 = 0.026;
                const MAX_MASSES: f64 = 7.1;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 1_944.26225;
                const MAX_RADIUS: f64 = 11_914.006;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.279545410512;
                const MAX_GRAVITY: f64 = 2.553103251365;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.279575634606;
                const MAX_ORBITAL: f64 = 271_840.426666667;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.25040603397;
                const MAX_ROTATIONAL: f64 = 5_591.70194340926;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::ELW(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "HMC" => { // High Metal Content Planet
                let ringed: bool = rng().gen_bool(1.0 / 20.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed High Metal Content Planet";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "HMC(R)";
                }else {
                    type_name = "High Metal Content Planet";
                    description = "CHANGE ME";
                    type_label = "HMC"
                };

                let rarity: Rarity = Rarity::Common;

                const MIN_DIST: f64 = 0.147454;
                const MAX_DIST: f64 = 7_488_550.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 38_894_529_198.7091;
                let rel_percentage_pressure: f64 = random_relative_percentage(MIN_PRESSURE, MAX_PRESSURE);
                let surface_pressure: f64 = val_from_relative_percentage(MIN_PRESSURE, MAX_PRESSURE, rel_percentage_pressure);

                const MIN_TEMP: f64 = 20.0;
                const MAX_TEMP: f64 = 46_100.0;
                let surface_temp: f64 = val_from_relative_percentage(MIN_TEMP, MAX_TEMP, rel_percentage_pressure);

                const MIN_MASSES: f64 = 0.0001;
                const MAX_MASSES: f64 = 1_397.998047;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 210.242671875;
                const MAX_RADIUS: f64 = 72_253.984;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.028504229273;
                const MAX_GRAVITY: f64 = 228.220131339448;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.005607748738;
                const MAX_ORBITAL: f64 = 111_160_422.502844;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.055748183634;
                const MAX_ROTATIONAL: f64 = 141_426.654814815;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::HMC(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "RIW" => { // Rocky Ice World
                let ringed: bool = rng().gen_bool(1.0 / 25.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Rocky Ice World (Ringed)";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "RIW(R)";
                }else {
                    type_name = "Rocky Ice World";
                    description = "CHANGE ME";
                    type_label = "RIW";
                };

                let rarity: Rarity = Rarity::Uncommon;

                const MIN_DIST: f64 = 5.3542;
                const MAX_DIST: f64 = 5_339_010.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 253_668_685.603375;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 20.0;
                const MAX_TEMP: f64 = 15_742.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 0.000107;
                const MAX_MASSES: f64 = 298.62381;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 276.0;
                const MAX_RADIUS: f64 = 28_515.804;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.001378452377;
                const MAX_GRAVITY: f64 = 17.259812728912;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.167619572396;
                const MAX_ORBITAL: f64 = 58_634_326.6897731;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.147149262604;
                const MAX_ROTATIONAL: f64 = 47_808.7140740741;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::RIW(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "MRB" => { // Metal-Rich Body
                let ringed: bool = rng().gen_bool(1.0 / 30.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Metal-Rich Body (Ringed)";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "MRB(R)";
                }else {
                    type_name = "Metal-Rich Body";
                    description = "CHANGE ME";
                    type_label = "MRB";
                };

                let rarity: Rarity = Rarity::Uncommon;

                const MIN_DIST: f64 = 0.087741;
                const MAX_DIST: f64 = 7_489_630.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 43_050_307_445.3848;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 20.0;
                const MAX_TEMP: f64 = 47_991.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 0.0001;
                const MAX_MASSES: f64 = 715.209778;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 137.38325;
                const MAX_RADIUS: f64 = 20_739.046;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.029231388904;
                const MAX_GRAVITY: f64 = 199.958389460213;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.005403750475;
                const MAX_ORBITAL: f64 = 70_018_026.7018299;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.046768454097;
                const MAX_ROTATIONAL: f64 = 5_578.24185185;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::MRB(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "HGG" => { // Helium Gass Giant
                let ringed: bool = rng().gen_bool(1.0 / 7.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Helium Gas Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "HGG(R)";
                }else {
                    type_name = "Helium Gas Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "HGG";
                };

                let rarity: Rarity = Rarity::VeryRare;

                const MIN_DIST: f64 = 159.044;
                const MAX_DIST: f64 = 5_542.96;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 30_887.2179620035;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 53.0;
                const MAX_TEMP: f64 = 1_701.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 9.003934;
                const MAX_MASSES: f64 = 5_781.101074;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 16_762.012;
                const MAX_RADIUS: f64 = 75_900.72;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 1.30247301576;
                const MAX_GRAVITY: f64 = 515.948083392392;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 30.12353209434;
                const MAX_ORBITAL: f64 = 10_178.4751922996;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.517331237708;
                const MAX_ROTATIONAL: f64 = 105.243145496817;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::HGG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "GGGG" => { // Glowing Green Gas Giant
                let ringed: bool = rng().gen_bool(1.0 / 5.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Glowing Green Gas Giant (Ringed)";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "GGGG(R)";
                }else {
                    type_name = "Glowing Green Gas Giant";
                    description = "CHANGE ME";
                    type_label = "GGGG";
                };

                let rarity: Rarity = Rarity::Legendary;

                const MIN_DIST: f64 = 7.77777;
                const MAX_DIST: f64 = 4_200_069.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 4.20696969;
                const MAX_PRESSURE: f64 = 420.696969;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 100.0;
                const MAX_TEMP: f64 = 150.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 1.77777777;
                const MAX_MASSES: f64 = 910.69696969;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 9_010.666;
                const MAX_RADIUS: f64 = 77_777.420;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.2420420420;
                const MAX_GRAVITY: f64 = 20.4;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.01337;
                const MAX_ORBITAL: f64 = 3_333_420.696969;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.17777777777777;
                const MAX_ROTATIONAL: f64 = 317_808.789789789;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::GGGG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "CIGG" => { // Class I Gas Giant
                let ringed: bool = rng().gen_bool(1.0 / 3.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Class I Gas Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "CIGG(R)";
                }else {
                    type_name = "Class I Gas Giant";
                    description = "CHANGE ME";
                    type_label = "CIIGG";
                };

                let rarity: Rarity = Rarity::Common;

                const MIN_DIST: f64 = 3.0;
                const MAX_DIST: f64 = 4_404_300.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 0.00379244308;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 1.0;
                const MAX_TEMP: f64 = 150.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 0.734365;
                const MAX_MASSES: f64 = 911.079224;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 8_079.091;
                const MAX_RADIUS: f64 = 77_787.584;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.191422307729;
                const MAX_GRAVITY: f64 = 19.845980443942;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.001108796332;
                const MAX_ORBITAL: f64 = 584_242_168.67597;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.001365740741;
                const MAX_ROTATIONAL: f64 = 7_691_317.0962963;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::CIGG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "CIIGG" => { // Class II Gas Giant
                let ringed: bool = rng().gen_bool(1.0 / 3.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Class II Gas Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "CIIGG(R)";
                }else {
                    type_name = "Class II Gas Giant";
                    description = "CHANGE ME";
                    type_label = "CIIGG";
                };

                let rarity: Rarity = Rarity::Rare;

                const MIN_DIST: f64 = 4.41403;
                const MAX_DIST: f64 = 841_899.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 0.243458896875;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 61.0;
                const MAX_TEMP: f64 = 250.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 2.641097;
                const MAX_MASSES: f64 = 1_368.457764;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 10_222.803;
                const MAX_RADIUS: f64 = 90_000.0;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.252062726485;
                const MAX_GRAVITY: f64 = 24.584892915592;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.135475762743;
                const MAX_ORBITAL: f64 = 20_275_545.7549183;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.142548313032;
                const MAX_ROTATIONAL: f64 = 402_652.586939491;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::CIIGG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "CIIIGG" => { // Class III Gas Giant
                let ringed: bool = rng().gen_bool(1.0 / 3.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Class III Gas Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "CIIIGG";
                }else {
                    type_name = "Class III Gas Giant";
                    description = "CHANGE ME";
                    type_label = "CIIIGG";
                };

                let rarity: Rarity = Rarity::Common;

                const MIN_DIST: f64 = 1.61936;
                const MAX_DIST: f64 = 7_492_280.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 20_305.728515625;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 115.0;
                const MAX_TEMP: f64 = 800.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 4.296463;
                const MAX_MASSES: f64 = 3_457.905762;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 12_120.956;
                const MAX_RADIUS: f64 = 77_849.944;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.274524836901;
                const MAX_GRAVITY: f64 = 199.304829579928;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.018557870653;
                const MAX_ORBITAL: f64 = 106_016_638.240329;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.000862268519;
                const MAX_ROTATIONAL: f64 = 218_760.441309931;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::CIIIGG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "CIVGG" => { // Class IV Gas Giant
                let ringed: bool = rng().gen_bool(1.0 / 3.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Class IV Gas Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "CIVGG(R)";
                }else {
                    type_name = "Class IV Gas Giant";
                    description = "CHANGE ME";
                    type_label = "CIVGG";
                };

                let rarity: Rarity = Rarity::Uncommon;

                const MIN_DIST: f64 = 0.971942;
                const MAX_DIST: f64 = 7_492_300.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.420;
                const MAX_PRESSURE: f64 = 30_707.168942;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 800.0;
                const MAX_TEMP: f64 = 1450.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 16.754765;
                const MAX_MASSES: f64 = 5_403.108398;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 17_305.224;
                const MAX_RADIUS: f64 = 78_291.304;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.441846796047;
                const MAX_GRAVITY: f64 = 71.175610458804;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.01965354213;
                const MAX_ORBITAL: f64 = 4_126_230.75555556;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.057442621296;
                const MAX_ROTATIONAL: f64 = 10_070_851.1288889;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::CIVGG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "CVGG" => { // Class V Gas Giant
                let ringed: bool = rng().gen_bool(1.0 / 3.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Class V Gas Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "CVGG(R)";
                }else {
                    type_name = "Class V Gas Giant";
                    description = "CHANGE ME";
                    type_label = "CVGG";
                };

                let rarity: Rarity = Rarity::Rare;

                const MIN_DIST: f64 = 0.319073;
                const MAX_DIST: f64 = 697_615.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.420;
                const MAX_PRESSURE: f64 = 33_333.333;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 1_400.0;
                const MAX_TEMP: f64 = 13_712.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 32.504833;
                const MAX_MASSES: f64 = 13_063.395508;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 20_016.274;
                const MAX_RADIUS: f64 = 77_806.056;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.593483029343;
                const MAX_GRAVITY: f64 = 404.862526550861;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.007973187153;
                const MAX_ORBITAL: f64 = 4_133_050.70941095;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.062575856586;
                const MAX_ROTATIONAL: f64 = 2_817.69796296296;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::CVGG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "HRGG" => { // Helium-Rich Gas Giant
                let ringed: bool = rng().gen_bool(1.0 / 7.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Ringed Helium-Rich Gas Giant";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "HRGG(R)";
                }else {
                    type_name = "Helium-Rich Gas Giant";
                    description = "CHANGE ME";
                    type_label = "HRGG";
                };

                let rarity: Rarity = Rarity::VeryRare;

                const MIN_DIST: f64 = 2.47592;
                const MAX_DIST: f64 = 731_999.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 0.0;
                const MAX_PRESSURE: f64 = 37_777.7;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 1.0;
                const MAX_TEMP: f64 = 7_787.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 1.028593;
                const MAX_MASSES: f64 = 4_764.864258;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 9_557.561;
                const MAX_RADIUS: f64 = 77_743.44;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.244444247333;
                const MAX_GRAVITY: f64 = 81.245461374845;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.16154257787;
                const MAX_ORBITAL: f64 = 2_572_298.80888889;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.063662365671;
                const MAX_ROTATIONAL: f64 = 2_029.4611030485;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::HRGG(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "GGWABL" => { // Gas Giant with Ammonia Based Life
                let ringed: bool = rng().gen_bool(1.0 / 5.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Gas Giant With Ammonia-Based Life (Ringed)";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "GGWABL(R)";
                }else {
                    type_name = "Gas Giant With Ammonia-Based Life";
                    description = "CHANGE ME";
                    type_label = "GGWABL";
                };

                let rarity: Rarity = Rarity::VeryRare;

                const MIN_DIST: f64 = 7.19408;
                const MAX_DIST: f64 = 4_217_110.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 4.20;
                const MAX_PRESSURE: f64 = 69_696.9;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 100.0;
                const MAX_TEMP: f64 = 150.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 1.791545;
                const MAX_MASSES: f64 = 909.972778;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 10_265.455;
                const MAX_RADIUS: f64 = 77_844.096;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.245295452982;
                const MAX_GRAVITY: f64 = 18.147572568807;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.015803241023;
                const MAX_ORBITAL: f64 = 3_331_479.58196979;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.15729662816;
                const MAX_ROTATIONAL: f64 = 316_824.865185185;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::GGWABL(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            "GGWWBL" => { // Gas Giant with Water Baed Life
                let ringed: bool = rng().gen_bool(1.0 / 5.0);

                let type_name: &str;
                let description: &str;
                let type_label: &str;
                if ringed {
                    type_name = "Gas Giant With Water-Based Life (Ringed)";
                    description = "CHANGE ME (WITH RINGS)";
                    type_label = "GGWWBL(R)";
                }else {
                    type_name = "Gas Giant With Water-Based Life";
                    description = "CHANGE ME";
                    type_label = "GGWWBL";
                };

                let rarity: Rarity = Rarity::Rare;

                const MIN_DIST: f64 = 5.81925;
                const MAX_DIST: f64 = 4_214_690.0;
                let dist_from_arrival: f64 = rng().gen_range(MIN_DIST..=MAX_DIST);

                const MIN_PRESSURE: f64 = 4.20;
                const MAX_PRESSURE: f64 = 69_420.420691337;
                let surface_pressure: f64 = rng().gen_range(MIN_PRESSURE..=MAX_PRESSURE);

                const MIN_TEMP: f64 = 150.0;
                const MAX_TEMP: f64 = 250.0;
                let surface_temp: f64 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_MASSES: f64 = 2.590262;
                const MAX_MASSES: f64 = 1_367.778809;
                let rel_percentage_mass: f64 = random_relative_percentage(MIN_MASSES, MAX_MASSES);
                let earth_masses: f64 = val_from_relative_percentage(MIN_MASSES, MAX_MASSES, rel_percentage_mass);

                const MIN_RADIUS: f64 = 10_265.455;
                const MAX_RADIUS: f64 = 77_844.096;
                let radius: f64 = val_from_relative_percentage(MIN_RADIUS, MAX_RADIUS, rel_percentage_mass);

                const MIN_GRAVITY: f64 = 0.252151647889;
                const MAX_GRAVITY: f64 = 26.734562844827;
                let gravity: f64 = val_from_relative_percentage(MIN_GRAVITY, MAX_GRAVITY, rel_percentage_mass);

                const MIN_ORBITAL: f64 = 0.002035879559;
                const MAX_ORBITAL: f64 = 16_447_556.1016134;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.114944627546;
                const MAX_ROTATIONAL: f64 = 458_802.441481481;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let landable: bool = is_landable(surface_pressure, surface_temp);
                let explorable: bool = is_explorable(surface_pressure, surface_temp, gravity);

                return Self::GGWWBL(PlanetTypeProperties {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    landable,
                    explorable,
                    dist_from_arrival,
                    surface_temp,
                    surface_pressure,
                    radius,
                    earth_masses,
                    gravity,
                    orbital_period,
                    rotational_period,
                });
            },
            _ => rand::random(),
        }
    }

    fn extract_stats(&self) -> Stats {

        match self {
            PlanetType::AW(i) | PlanetType::WW(i) | PlanetType::WG(i)
            | PlanetType::RKB(i) | PlanetType::ICB(i) | PlanetType::ELW(i)
            | PlanetType::HMC(i) | PlanetType::RIW(i) | PlanetType::MRB(i)
            | PlanetType::HGG(i) | PlanetType::GGGG(i) | PlanetType::CIGG(i)
            | PlanetType::CIIGG(i) | PlanetType::CIIIGG(i) | PlanetType::CIVGG(i)
            | PlanetType::CVGG(i) | PlanetType::HRGG(i) | PlanetType::GGWABL(i)
            | PlanetType::GGWWBL(i) => 
            {
                return Stats{
                    label: i.type_label,
                    type_name: i.type_name,
                    description: i.description,
                    rarity: i.rarity.fetch_rarity(),
                    ringed: i.ringed,
                    landable: i.landable,
                    explorable: i.explorable,
                    dist_from_arrival: i.dist_from_arrival,
                    surface_temp: i.surface_temp,
                    surface_pressure: i.surface_pressure,
                    radius: i.radius,
                    earth_masses: i.earth_masses,
                    gravity: i.gravity,
                    orbital_period: i.orbital_period,
                    rotational_period: i.rotational_period,
                };
            },
        };
    }

}


#[derive(Debug)]
pub struct Planet<'a> {
    pub name: String,
    pub ptype: PlanetType<'a>,
}

impl<'a> Planet<'a> {
    /// Returns an instance of the *Planet* struct with randomly-initialized properties.
    /// Expects a name and a planet-type identifier, but will provide a random *PlanetType* if an invalid identifier is provided.
    ///
    /// **Valid Identifiers:**
    /// [AW, WW, WG, RKB, ICB, ELW, HMC, RIW, MRB, HGG, GGGG, CIGG, CIIGG, CIIIGG, CIVGG, CVGG, HRGG, GGWABL, GGWWBL]
    pub fn new(name: String, ptype: &'a str) -> Self {
        Self{ name, ptype: PlanetType::new(ptype)}
    }

    /// Returns the *Stats* struct for the currently-calling *Planet* instance.
    pub fn stats(&self) -> Stats {
        let stats = &self.ptype;
        stats.extract_stats()
    }
}


/* Planetary Util Functions */

/// Uses pressure and tempurature to determine whether or not the environment is safe to land a spaceship on.
fn is_landable(pressure: f64, temp: f64) -> bool {
    if pressure <= 4.20 && temp <= 666.0 {
        return true;
    }else {
        return false;
    };
}

/// Uses pressure, tempurature, and gravity to determine if the environment is fit for on-foot exploration.
fn is_explorable(pressure: f64, temp: f64, gravity: f64) -> bool {
    if pressure <= 2.25 && temp <= 370.0 && gravity <= 4.20 {
        return true;
    }else {
        return false;
    };
}

/// Returns a random position within a range of numbers (min, max), represented as a relative percentage of that range.
fn random_relative_percentage(min: f64, max: f64) -> f64 {
    let n = rng().gen_range(min..=max);

    // Formula for relative percentage of the range.
    ((n - min) / (max - min)) * 100.0
}

/// Returns a value representing the position found at a given percentage of a given range of numbers.
fn val_from_relative_percentage(min: f64, max: f64, percentage: f64) -> f64 {
    let perc_as_dec = percentage / 100.0;

    min + (max - min) * perc_as_dec
}
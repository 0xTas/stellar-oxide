use std::time::Duration;
use crate::Rarity;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
    thread_rng as rng,
};

#[derive(Debug)]
pub struct ClassInfo<'a> {
    pub ringed: bool,
    pub type_name: &'a str,
    pub description: &'a str,
    pub rarity: Rarity,
    pub scoopable: bool,
    pub boostable: bool,
    pub supergiant: bool,
    pub is_moon: bool,
    pub is_primary: bool,
    pub age: u64, // Unit of measurement is "millions of years"
    pub solar_masses: f64, // Masses of the (our) sun
    pub solar_radius: f64, // Radius in relation to our own sun's
    pub surface_temp: i32, // Kelvin
    pub orbital_period: Duration,
    pub rotational_period: Duration,

}

#[derive(Debug)]
pub struct Stats<'a> {
    pub ringed: bool,
    pub class_name: &'a str,
    pub description: &'a str,
    pub rarity: &'a str,
    pub can_fuel_scoop: bool,
    pub can_fsd_boost: bool,
    pub is_supergiant: bool,
    pub is_primary: bool,
    pub is_moon: bool,
    pub age: u64,
    pub solar_masses: f64,
    pub solar_radii: f64,
    pub surface_temp: i32,
    pub orbital_period: Duration,
    pub rotational_period: Duration,
}

#[derive(Debug)]
pub enum StarClass<'a> {
    O(ClassInfo<'a>),
    B(ClassInfo<'a>),
    A(ClassInfo<'a>),
    F(ClassInfo<'a>),
    G(ClassInfo<'a>),
    K(ClassInfo<'a>),
    M(ClassInfo<'a>),
    L(ClassInfo<'a>),
    T(ClassInfo<'a>),
    Y(ClassInfo<'a>),
    Proto(ClassInfo<'a>),
    Carbon(ClassInfo<'a>),
    WR(ClassInfo<'a>),
    WD(ClassInfo<'a>),
    NS(ClassInfo<'a>),
    BH(ClassInfo<'a>),
}

/* Enables Fetching a random variant of the StarClass enum */
impl<'a> Distribution<StarClass<'a>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StarClass<'a> {
        match rng.gen_range(0..15) {
            0 => StarClass::new("O"),
            1 => StarClass::new("B"),
            2 => StarClass::new("A"),
            3 => StarClass::new("F"),
            4 => StarClass::new("G"),
            5 => StarClass::new("K"),
            6 => StarClass::new("M"),
            7 => StarClass::new("L"),
            8 => StarClass::new("T"),
            9 => StarClass::new("Y"),
            10 => StarClass::new("Proto"),
            11 => StarClass::new("Carbon"),
            12 => StarClass::new("WR"),
            13 => StarClass::new("WD"),
            14 => StarClass::new("NS"),
            15 => StarClass::new("BH"),
            _ => panic!("Invalid StarClass!"),
        }
    }
}

impl<'a> StarClass<'a> {
    pub fn new(class: &str) -> Self {

        /* Here is where most property values of different star classes are assigned */
        match class {
            "O" => {
                let class_name: &str;
                let description: &str;

                let scoopable: bool = true;
                let boostable: bool = false;
                let supergiant: bool = false;
                let ringed: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.42);
                let is_moon: bool = rng().gen_bool(1.0 / 42_069.1337);

                if is_moon {
                    is_primary =  false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 1_020;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 2.960938;
                const MAX_MASS: f64 = 119.9375;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.044562874458;
                const MAX_RADIUS: f64 = 223.009942683172;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 11_298;
                const MAX_TEMP: i32 = 115_905;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.035106634236;
                const MAX_ORBITAL: f64 = 2_579_866.94298409;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.014184147535;
                const MAX_ROTATIONAL: f64 = 131.100810185185;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let rarity: Rarity;
                if solar_radius >= 360.0 {
                    supergiant = true;
                    is_moon = false;
                    is_primary = true;
                    rarity = Rarity::new("L");
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed Class O Star";
                    description = "Ringed Class O Star";
                    rarity = Rarity::new("L");
                }else {
                    class_name = "Class O Star";
                    description = "Class O Star";
                    rarity = Rarity::new("VR");
                };

                return Self::O(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "B" => {
                let class_name: &str;
                let description: &str;

                let supergiant: bool;
                let scoopable: bool = true;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 20.0);
                let is_primary: bool = rng().gen_bool(1.0 / 1.7);
                let is_moon: bool = rng().gen_bool(1.0 / 17.0);

                if is_moon {
                    is_primary =  false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 12_040;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.183594;
                const MAX_MASS: f64 = 120.0;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.041514797369;
                const MAX_RADIUS: f64 = 446.9525046844;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 3_835;
                const MAX_TEMP: i32 = 113_827;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.112869194884;
                const MAX_ORBITAL: f64 = 105_308_641.990026;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.00495087537;
                const MAX_ROTATIONAL: f64 = 235.707662037037;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 297.0 {
                    supergiant = true;
                    is_moon = false;
                    is_primary = true;
                    ringed = false;
                    rarity = Rarity::new("VR");
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed B Class Star";
                    description = "CHANGE ME (Ringed)";
                    rarity = Rarity::new("VR");
                }else {
                    class_name = "B Class Star";
                    description = "CHANGE ME";
                    rarity = Rarity::new("R");
                };


                return Self::B(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "A" => {
                let class_name: &str;
                let description: &str;

                let supergiant: bool;
                let scoopable: bool = true;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 420_069.0);
                let is_primary: bool = rng().gen_bool(1.0 / 1.4);
                let is_moon: bool = rng().gen_bool(1.0 / 420_069.0);

                if is_moon {
                    is_primary =  false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 12_224;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.074219;
                const MAX_MASS: f64 = 105.1875;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.009967209266;
                const MAX_RADIUS: f64 = 499.437983194824;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 4_271;
                const MAX_TEMP: i32 = 34_945;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.093999310498;
                const MAX_ORBITAL: f64 = 66_930_624.0699909;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.042729657257;
                const MAX_ROTATIONAL: f64 = 254.093055555556;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 150.0 {
                    supergiant = true;
                    is_moon = false;
                    is_primary = true;
                    ringed = false;
                    rarity = Rarity::new("VR");
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed A Class Star";
                    description = "Ringed A Class Star";
                    rarity = Rarity::new("ER");
                }else {
                    class_name = "A Class (Blue-White) Star";
                    description = "CHANGE ME";
                    rarity = Rarity::new("UC");
                };


                return Self::A(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "F" => {
                let class_name: &str;
                let description: &str;

                let supergiant: bool;
                let scoopable: bool = true;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 52.0);
                let is_primary: bool = rng().gen_bool(1.0 / 1.14269);
                let is_moon: bool = rng().gen_bool(1.0 / 69.420);

                if is_moon {
                    is_primary =  false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_038;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.492188;
                const MAX_MASS: f64 = 17.0;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.499396823391;
                const MAX_RADIUS: f64 = 214.740608599247;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 3_798;
                const MAX_TEMP: i32 = 34_130;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.086331758854;
                const MAX_ORBITAL: f64 = 273_578_917.538678;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.198873008519;
                const MAX_ROTATIONAL: f64 = 414.054418727419;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 50.0 {
                    supergiant = true;
                    is_moon = false;
                    is_primary = true;
                    ringed = false;
                    rarity = Rarity::new("VR");
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed F Class Star";
                    description = "CHANGE ME (Ringed)";
                    rarity = Rarity::new("VR");
                }else {
                    class_name = "F Class Star";
                    description = "CHANGE ME";
                    rarity = Rarity::new("C");
                };


                return Self::F(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "G" => {
                let class_name: &str;
                let description: &str;

                let supergiant: bool;
                let scoopable: bool = true;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 142.0);
                let is_primary: bool = rng().gen_bool(1.0 / 1.0420);
                let is_moon: bool = rng().gen_bool(1.0 / 100.0);

                if is_moon {
                    is_primary =  false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.421875;
                const MAX_MASS: f64 = 6.253906;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.509979237958;
                const MAX_RADIUS: f64 = 142.490128563623;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 3_346;
                const MAX_TEMP: i32 = 18_407;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.086580284965;
                const MAX_ORBITAL: f64 = 465_068_317.673824;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.460332842512;
                const MAX_ROTATIONAL: f64 = 540.618148148148;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 42.0 {
                    supergiant = true;
                    is_moon = false;
                    is_primary = true;
                    ringed = false;
                    rarity = Rarity::new("ER");
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed G Class Star";
                    description = "CHANGE ME (Ringed)";
                    rarity = Rarity::new("ER");
                }else {
                    class_name = "G Class Star";
                    description = "CHANGE ME";
                    rarity = Rarity::new("C");
                };


                return Self::G(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "K" => {
                let class_name: &str;
                let description: &str;

                let supergiant: bool;
                let scoopable: bool = true;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 27.0);
                let is_primary: bool = rng().gen_bool(1.0 / 1.6666667);
                let is_moon: bool = rng().gen_bool(1.0 / 30.0);


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_062;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.449218988419;
                const MAX_MASS: f64 = 13.097656;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.104853824281;
                const MAX_RADIUS: f64 = 907.527721837034;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 3_255;
                const MAX_TEMP: i32 = 8_442;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.086368436458;
                const MAX_ORBITAL: f64 = 393_208_219.77324;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.549157895694;
                const MAX_ROTATIONAL: f64 = 4_491.19099445159;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 58.0 {
                    supergiant = true;
                    is_moon = false;
                    is_primary = true;
                    ringed = false;
                    rarity = Rarity::new("VR");
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed K Class Star";
                    description = "CHANGE ME (Ringed)";
                    rarity = Rarity::new("VR");
                }else {
                    class_name = "K Class Star";
                    description = "CHANGE ME";
                    rarity = Rarity::new("C");
                };

                if is_moon {
                    is_primary =  false;
                };


                return Self::K(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "M" => {
                let class_name: &str;
                let description: &str;

                let supergiant: bool;
                let scoopable: bool;
                let boostable: bool;
                let ringed: bool = rng().gen_bool(1.0 / 17.0);
                let is_primary: bool = rng().gen_bool(1.0 / 1.97778);
                let is_moon: bool = rng().gen_bool(1.0 / 20.0);

                if is_moon {
                    is_primary =  false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.019531;
                const MAX_MASS: f64 = 8.589844;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.086108154063;
                const MAX_RADIUS: f64 = 67.189511913731;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 508;
                const MAX_TEMP: i32 = 21_999;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.104032965764;
                const MAX_ORBITAL: f64 = 577_522_496.93429;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.312319726782;
                const MAX_ROTATIONAL: f64 = 1_635.77264233275;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 420.0 {
                    supergiant = true;
                    is_moon = false;
                    is_primary = true;
                    ringed = false;
                    rarity = Rarity::new("R");
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed M Class Star";
                    description = "CHANGE ME (Ringed)";
                    rarity = Rarity::new("R");
                }else {
                    class_name = "M Class Star";
                    description = "CHANGE ME";
                };


                return Self::M(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "L" => {
                let class_name: &str;
                let description: &str;

                let scoopable: bool = false;
                let boostable: bool = false;
                let supergiant: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 15.0);
                let is_primary: bool = rng().gen_bool(1.0 / 7.0);
                let is_moon: bool = rng().gen_bool(1.0 / 11.0);

                if is_moon {
                    is_primary =  false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.039063;
                const MAX_MASS: f64 = 1.132813;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.088881902519;
                const MAX_RADIUS: f64 = 1.081970196981;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 1_300;
                const MAX_TEMP: i32 = 2_478;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.186782516539;
                const MAX_ORBITAL: f64 = 29_584_303.597037;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.330786200625;
                const MAX_ROTATIONAL: f64 = 5_774.96615652396;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    class_name = "Ringed Class L Brown Dwarf";
                    description = "CHANGE ME (Ringed)";
                    if is_primary {
                        rarity = Rarity::new("UC");
                    }else {
                        rarity = Rarity::new("C");
                    };
                }else {
                    class_name = "Class L Brown Dwarf";
                    description = "CHANGE ME";
                    rarity = Rarity::new("VC");
                };


                return Self::L(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "T" => {
                let class_name: &str;
                let description: &str;

                let supergiant: bool = false;
                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 3.0);
                let is_primary: bool = rng().gen_bool(1.0 / 2.0);
                let is_moon: bool = rng().gen_bool(1.0 / 5.0);

                if is_moon {
                    is_primary = false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.019531;
                const MAX_MASS: f64 = 0.117188;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.079890076687;
                const MAX_RADIUS: f64 = 0.282836911574;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 545;
                const MAX_TEMP: i32 = 1_299;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.054129729097;
                const MAX_ORBITAL: f64 = 28_190_283.0357905;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.174115841296;
                const MAX_ROTATIONAL: f64 = 7_354.71481481481;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    class_name = "Ringed Class T Brown Dwarf";
                    description = "CHANGE ME (Ringed)";
                    if is_moon {
                        rarity = Rarity::new("C");
                    }else {
                        rarity = Rarity::new("UC");
                    };
                }else {
                    class_name = "Class T Brown Dwarf";
                    description = "CHANGE ME";
                    rarity = Rarity::new("C");
                };


                return Self::T(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "Y" => {
                let class_name: &str;
                let description: &str;

                let scoopable: bool = false;
                let boostable: bool = false;
                let supergiant: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 2.0);
                let is_primary: bool = rng().gen_bool(1.0 / 7.0);
                let is_moon: bool = rng().gen_bool(1.0 / 3.0);

                if is_primary {
                    is_moon = false;
                };


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.007799999788;
                const MAX_MASS: f64 = 6.0;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.022277830338;
                const MAX_RADIUS: f64 = 3.895288853146;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 128;
                const MAX_TEMP: i32 = 699;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.063383751725;
                const MAX_ORBITAL: f64 = 52_604_529.7306997;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.075849250694;
                const MAX_ROTATIONAL: f64 = 3_905_486.45233072;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    class_name = "Ringed Class Y Brown Dwarf";
                    description = "CHANGE ME (Ringed)";
                    if is_primary {
                        rarity = Rarity::new("UC");
                    }else {
                        rarity = Rarity::new("C");
                    };
                    
                }else {
                    class_name = "Class Y Brown Dwarf";
                    description = "CHANGE ME";
                    rarity = Rarity::new("C");
                };


                return Self::Y(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_moon,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            _ => {


                // let class_name: &str;
                // let description: &str;

                // let scoopable: bool;
                // let boostable: bool;
                // let supergiant: bool;
                // let ringed: bool;
                // let is_primary: bool;
                // let is_moon: bool;

                // if is_moon {
                //     is_primary = false;
                // };


                // const MIN_AGE: u64;
                // const MAX_AGE: u64;
                // let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                // const MIN_MASS: f64;
                // const MAX_MASS: f64;
                // let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                // const MIN_RADIUS: f64;
                // const MAX_RADIUS: f64;
                // let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                // const MIN_TEMP: i32;
                // const MAX_TEMP: i32;
                // let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                // const MIN_ORBITAL: f64;
                // const MAX_ORBITAL: f64;
                // let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                // let orbital_secs: f64 = orbital_range * 86_400.0;
                // let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                // const MIN_ROTATIONAL: f64;
                // const MAX_ROTATIONAL: f64;
                // let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                // let rotational_secs: f64 = orbital_range * 86_400.0;
                // let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                // let rarity: Rarity;
                // if solar_radius >= 420.0 {
                //     supergiant = true;
                //     is_moon = false;
                //     is_primary = true;
                //     ringed = false;
                //     rarity = Rarity::new("");
                // }else {
                //     supergiant = false;
                // };

                // if ringed {
                //     class_name = "CHANGE ME (Ringed)";
                //     description = "CHANGE ME (Ringed)";
                //     rarity = Rarity::new("");
                // }else {
                //     class_name = "CHANGE ME";
                //     description = "CHANGE ME";
                //     rarity = Rarity::new("");
                // };


                // return Self::CHANGEME(ClassInfo {
                //     ringed,
                //     type_name,
                //     description,
                //     rarity,
                //     scoopable,
                //     boostable,
                //     supergiant,
                //     is_moon,
                //     is_primary,
                //     age,
                //     solar_masses,
                //     solar_radius,
                //     surface_temp,
                //     orbital_period,
                //     rotational_period,
                // });
            },
        }
    }

    fn extract_stats(&self) -> Stats {
        match self {
            Self::O(i) | Self::B(i) | Self::A(i) | Self::F(i)
             | Self::G(i) | Self::K(i) | Self::M(i) | Self::L(i) | Self::T(i) 
             | Self::Y(i) | Self::Proto(i) | Self::Carbon(i) | Self::WR(i) | Self::WD(i)
             | Self::NS(i) | Self::BH(i) =>
             {
                return Stats{
                    ringed: i.ringed,
                    class_name: i.type_name,
                    description: i.description,
                    rarity: i.rarity,
                    can_fuel_scoop: i.scoopable,
                    can_fsd_boost: i.boostable,
                    is_supergiant: i.supergiant,
                    is_primary: i.is_primary,
                    is_moon: i.is_moon,
                    age: i.age,
                    solar_masses: i.solar_masses,
                    solar_radii: i.solar_radius,
                    surface_temp: i.surface_temp,
                    orbital_period: i.orbital_period,
                    rotational_period: i.rotational_period,
                };
             },
        };
    }


}

#[derive(Debug)]
pub struct Star<'a> {
    pub name: &'a str,
    pub class: StarClass<'a>,
    pub subtype: Option<&'a str>,
}

impl<'a> Star<'a> {
    pub fn new(name: &'a str, class: &'a str, subtype: Option<&'a str>) -> Self {
        let class = StarClass::new(class);
        Star {
            name,
            class,
            subtype,
        }
    }

    pub fn stats(&self) -> Stats {
        let x: &StarClass = &self.class;
        x.extract_stats()
    }
}
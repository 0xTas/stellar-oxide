use std::time::Duration;
use crate::Rarity;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
    thread_rng as rng,
};

#[derive(Debug)]
pub struct ClassInfo<'a> {
    pub type_label: &'a str,
    pub type_name: &'a str,
    pub description: &'a str,
    pub rarity: Rarity,
    pub ringed: bool,
    pub scoopable: bool,
    pub boostable: bool,
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
    pub label: &'a str,
    pub class_name: &'a str,
    pub description: &'a str,
    pub rarity: &'a str,
    pub can_fuel_scoop: bool,
    pub can_fsd_boost: bool,
    pub is_moon: bool,
    pub is_primary: bool,
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
    OG(ClassInfo<'a>),
    B(ClassInfo<'a>),
    BG(ClassInfo<'a>),
    A(ClassInfo<'a>),
    AG(ClassInfo<'a>),
    F(ClassInfo<'a>),
    FG(ClassInfo<'a>),
    G(ClassInfo<'a>),
    GG(ClassInfo<'a>),
    K(ClassInfo<'a>),
    KG(ClassInfo<'a>),
    M(ClassInfo<'a>),
    MG(ClassInfo<'a>),
    L(ClassInfo<'a>),
    T(ClassInfo<'a>),
    Y(ClassInfo<'a>),
    AEBE(ClassInfo<'a>),
    TTS(ClassInfo<'a>),
    C(ClassInfo<'a>),
    CJ(ClassInfo<'a>),
    CN(ClassInfo<'a>),
    MS(ClassInfo<'a>),
    S(ClassInfo<'a>),
    W(ClassInfo<'a>),
    WC(ClassInfo<'a>),
    WN(ClassInfo<'a>),
    WO(ClassInfo<'a>),
    NS(ClassInfo<'a>),
    PS(ClassInfo<'a>),
    MGT(ClassInfo<'a>),
    D(ClassInfo<'a>),
    DA(ClassInfo<'a>),
    DAB(ClassInfo<'a>),
    DAV(ClassInfo<'a>),
    DAZ(ClassInfo<'a>),
    DB(ClassInfo<'a>),
    DBV(ClassInfo<'a>),
    DBZ(ClassInfo<'a>),
    DC(ClassInfo<'a>),
    DCV(ClassInfo<'a>),
    DQ(ClassInfo<'a>),
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
            "O" => { // Class O (Blue) Stars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "O";

                let scoopable: bool = true;
                let boostable: bool = false;
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
                if ringed {
                    type_name = "Ringed Blue Star";
                    description = "Ringed Class O Star";
                    rarity = Rarity::new("L");
                }else {
                    type_name = "Blue Star";
                    description = "Class O Star";
                    rarity = Rarity::new("VR");
                };

                return Self::O(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "OG" => { // Class O BLue Giants+
                let type_name: &str;
                let description: &str;
                let type_label: &str = "O";

                let ringed: bool = false;
                let is_moon: bool = false;
                let scoopable: bool = true;
                let boostable: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.07);


                const MIN_AGE: u64 = 2;
                const MAX_AGE: u64 = 1_420;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 2.960938;
                const MAX_MASS: f64 = 119.9375;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 200.044562874458;
                const MAX_RADIUS: f64 = 300.009942683172;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 7_108;
                const MAX_TEMP: i32 = 105_105;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.035106634236;
                const MAX_ORBITAL: f64 = 2_579_866.94298409;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 50.014184147535;
                const MAX_ROTATIONAL: f64 = 151.100810185185;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 200.0 {
                    type_name = "Blue Supergiant";
                    description = "Class O Supergiant Star";
                    rarity = Rarity::new("L");
                }else {
                    type_name = "Blue Giant";
                    description = "Class O Giant Star";
                    rarity = Rarity::new("ER");
                };


                return Self::OG(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "B" => { // Class B (Blue-White) Stars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "B";

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
                const MAX_RADIUS: f64 = 300.9525046844;
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
                if ringed {
                    type_name = "Ringed Blue-White Star";
                    description = "Ringed Class B Star";
                    rarity = Rarity::new("VR");
                }else {
                    type_name = "Blue-White Star";
                    description = "Class B Star";
                    rarity = Rarity::new("R");
                };


                return Self::B(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "BG" => { // Class B Blue-White Giants+
                let type_name: &str;
                let description: &str;
                let type_label: &str = "B";

                let ringed: bool = false;
                let is_moon: bool = false;
                let scoopable: bool = true;
                let boostable: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.07);


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 4_976;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 1.398438;
                const MAX_MASS: f64 = 106.828125;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 250.067210081282;
                const MAX_RADIUS: f64 = 499.839161512581;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 6_456;
                const MAX_TEMP: i32 = 30_699;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.074489089769;
                const MAX_ORBITAL: f64 = 7_381_527.65013553;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.026924294051;
                const MAX_ROTATIONAL: f64 = 302.311684412442;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 300.0 {
                    type_name = "Blue-White Supergiant";
                    description = "Class B Supergiant Star";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "Blue-White Giant";
                    description = "Class B Giant Star";
                    rarity = Rarity::new("VR");
                };


                return Self::BG(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "A" => { // Class A (Blue-White) Stars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "A";

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
                const MAX_RADIUS: f64 = 145.437983194824;
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
                if ringed {
                    type_name = "Ringed Blue-White Star";
                    description = "Ringed A Class Star";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "Blue-White Star";
                    description = "Class A Star";
                    rarity = Rarity::new("UC");
                };


                return Self::A(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "AG" => { // Class A Blue-White Giants+
                let type_name: &str;
                let description: &str;
                let type_label: &str = "A";

                let ringed: bool = false;
                let is_moon: bool = false;
                let scoopable: bool = true;
                let boostable: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.0007);


                const MIN_AGE: u64 = 1;
                const MAX_AGE: u64 = 4_944;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 1.398438;
                const MAX_MASS: f64 = 106.222656;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 145.009967209266;
                const MAX_RADIUS: f64 = 499.995486780733;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 6_414;
                const MAX_TEMP: i32 = 30_696;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.537583550347;
                const MAX_ORBITAL: f64 = 5_502_668.04148148;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.090871525521;
                const MAX_ROTATIONAL: f64 = 284.561458333333;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 300.0 {
                    type_name = "Blue-White Supergiant";
                    description = "Class A Supergiant Star";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "Blue-White Giant";
                    description = "Class A Giant Star";
                    rarity = Rarity::new("VR");
                };


                return Self::AG(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "F" => { // Class F (White) Stars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "F";

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
                const MAX_RADIUS: f64 = 40.740608599247;
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
                if ringed {
                    type_name = "Ringed White Star";
                    description = "Ringed Class F Star";
                    rarity = Rarity::new("VR");
                }else {
                    type_name = "White Star";
                    description = "Class F Star";
                    rarity = Rarity::new("C");
                };


                return Self::F(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "FG" => { // Class F White Giants+
                let type_name: &str;
                let description: &str;
                let type_label: &str = "F";

                let ringed: bool = false;
                let is_moon: bool = false;
                let scoopable: bool = true;
                let boostable: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.0007);


                const MIN_AGE: u64 = 256;
                const MAX_AGE: u64 = 11_994;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 45.8125;
                const MAX_MASS: f64 = 4.769531;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 40.499396823391;
                const MAX_RADIUS: f64 = 214.740608599247;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 4_493;
                const MAX_TEMP: i32 = 7_499;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 2.459441008391;
                const MAX_ORBITAL: f64 = 5_528_437.94962963;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 6.445642589225;
                const MAX_ROTATIONAL: f64 = 1_128.73787037037;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 150.0 {
                    type_name = "White Supergiant";
                    description = "Class F Supergiant Star";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "White Giant";
                    description = "Class F Giant Star";
                    rarity = Rarity::new("VR");
                };


                return Self::FG(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "G" => { // Class G (Yellow-White) Stars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "G";

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
                const MAX_RADIUS: f64 = 42.000128563623;
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
                if ringed {
                    type_name = "Ringed White-Yellow Star";
                    description = "Ringed Class G Star";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "White-Yellow Star";
                    description = "Class G Star";
                    rarity = Rarity::new("C");
                };


                return Self::G(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "GG" => { // Class G White-Yellow Giants+
                let type_name: &str;
                let description: &str;
                let type_label: &str = "G";

                let ringed: bool = false;
                let is_moon: bool = false;
                let scoopable: bool = true;
                let boostable: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.0007);


                const MIN_AGE: u64 = 2;
                const MAX_AGE: u64 = 12_792;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.796875;
                const MAX_MASS: f64 = 20.0;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 42.199875279318;
                const MAX_RADIUS: f64 = 142.490128563623;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 4_428;
                const MAX_TEMP: i32 = 7_496;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 1.012338152917;
                const MAX_ORBITAL: f64 = 9_810_303.81037037;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.786495949074;
                const MAX_ROTATIONAL: f64 = 1_262.85137953275;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 150.0 {
                    type_name = "White-Yellow Supergiant";
                    description = "Class G Supergiant Star";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "White-Yellow Giant";
                    description = "Class G Giant Star";
                    rarity = Rarity::new("VR");
                };


                return Self::GG(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "K" => { // Class K (Yellow-Orange) Stars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "K";

                let scoopable: bool = true;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 27.0);
                let is_primary: bool = rng().gen_bool(1.0 / 1.6666667);
                let is_moon: bool = rng().gen_bool(1.0 / 30.0);


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_062;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.265625;
                const MAX_MASS: f64 = 13.523438;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.188995347232;
                const MAX_RADIUS: f64 = 58.710197095615;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 2_494;
                const MAX_TEMP: i32 = 30_024;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.086368436458;
                const MAX_ORBITAL: f64 = 393_208_219.77324;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.549157895694;
                const MAX_ROTATIONAL: f64 = 427.435192176401;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Yellow-Orange Star";
                    description = "Ringed Class K Star";
                    rarity = Rarity::new("VR");
                }else {
                    type_name = "Yellow-Orange Star";
                    description = "Class K Star";
                    rarity = Rarity::new("C");
                };

                if is_moon {
                    is_primary =  false;
                };


                return Self::K(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "KG" => { // Class K Yellow-Orange Giants+
                let type_name: &str;
                let description: &str;
                let type_label: &str = "K";

                let ringed: bool = false;
                let is_moon: bool = false;
                let scoopable: bool = true;
                let boostable: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.0007);


                const MIN_AGE: u64 = 24;
                const MAX_AGE: u64 = 13_062;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.449218988419;
                const MAX_MASS: f64 = 13.097656;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 55.104853824281;
                const MAX_RADIUS: f64 = 907.527721837034;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 3_255;
                const MAX_TEMP: i32 = 8_442;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.291476440428;
                const MAX_ORBITAL: f64 = 203_291_043.91513;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.36237599088;
                const MAX_ROTATIONAL: f64 = 4_491.19099445159;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 777.0 {
                    type_name = "Yellow-Orange Hypergiant";
                    description = "Class K Hypergiant Star";
                    rarity = Rarity::new("L");
                }else if solar_radius >= 150.0 {
                    type_name = "Yellow-Orange Supergiant";
                    description = "Class K Supergiant Star";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "Yellow-Orange Giant";
                    description = "Class K Giant Star";
                    rarity = Rarity::new("VR");
                };


                return Self::KG(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "M" => { // Class M (Red Dwarf) Stars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "M";

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
                if ringed {
                    type_name = "Ringed Red Dwarf";
                    description = "Ringed Class M Red Dwarf";
                    rarity = Rarity::new("R");
                }else {
                    type_name = "Red Dwarf";
                    description = "Class M Red Dwarf";
                };


                return Self::M(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "MG" => { // Class M Red Giants+
                let type_name: &str;
                let description: &str;
                let type_label: &str = "M";

                let ringed: bool = false;
                let is_moon: bool = false;
                let scoopable: bool = true;
                let boostable: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.0007);


                const MIN_AGE: u64 = 24;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.199219;
                const MAX_MASS: f64 = 25.0;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 67.104853824281;
                const MAX_RADIUS: f64 = 1_418.28698100353;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 1_859;
                const MAX_TEMP: i32 = 5_199;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.192648970995;
                const MAX_ORBITAL: f64 = 21_461_973.522963;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.801408171296;
                const MAX_ROTATIONAL: f64 = 8_089.64444444444;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = orbital_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if solar_radius >= 1_000.0 {
                    type_name = "Red Hypergiant";
                    description = "Class M Red Hypergiant";
                    rarity = Rarity::new("ER");
                }else if solar_radius >= 602.0 {
                    type_name = "Red Supergiant";
                    description = "Class M Red Supergiant";
                    rarity = Rarity::new("VR");
                }else {
                    type_name = "Red Giant";
                    description = "Class M Red Giant";
                    rarity = Rarity::new("R");
                };


                return Self::MG(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "L" => { // Class L Brown Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "L";

                let scoopable: bool = false;
                let boostable: bool = false;
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
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "T" => { // Class T Brown Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "T";

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
                    type_name = "Ringed Class T Brown Dwarf";
                    description = "CHANGE ME (Ringed)";
                    if is_moon {
                        rarity = Rarity::new("C");
                    }else {
                        rarity = Rarity::new("UC");
                    };
                }else {
                    type_name = "Class T Brown Dwarf";
                    description = "CHANGE ME";
                    rarity = Rarity::new("C");
                };


                return Self::T(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
            "Y" => { // Class Y Brown Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str;

                let scoopable: bool = false;
                let boostable: bool = false;
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
                    type_name = "Ringed Class Y Brown Dwarf";
                    description = "CHANGE ME (Ringed)";
                    if is_primary {
                        rarity = Rarity::new("UC");
                    }else {
                        rarity = Rarity::new("C");
                    };
                    
                }else {
                    type_name = "Class Y Brown Dwarf";
                    description = "CHANGE ME";
                    rarity = Rarity::new("C");
                };


                return Self::Y(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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


                // let type_name: &str;
                // let description: &str;
                // let type_label: str;

                // let scoopable: bool;
                // let boostable: bool;
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
                // if ringed {
                //     type_name = "CHANGE ME (Ringed)";
                //     description = "CHANGE ME (Ringed)";
                //     rarity = Rarity::new("");
                // }else {
                //     type_name = "CHANGE ME";
                //     description = "CHANGE ME";
                //     rarity = Rarity::new("");
                // };


                // return Self::CHANGEME(ClassInfo {
                //     type_label,
                //     type_name,
                //     description,
                //     rarity,
                //     ringed,
                //     scoopable,
                //     boostable,
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
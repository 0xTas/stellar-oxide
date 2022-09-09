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
    pub age: u64, // Unit of measurement is "millions of years"
    pub solar_masses: f64, // Masses of the (our) sun
    pub solar_radius: f64, // Radius in relation to our own sun's
    pub surface_temp: i32, // Kelvin
    pub orbital_period: Duration,
    pub rotational_period: Duration,
}

#[derive(Debug)]
pub struct Stats<'a> {
    pub label: &'a str,
    pub class_name: &'a str,
    pub description: &'a str,
    pub rarity: &'a str,
    pub ringed: bool,
    pub can_fuel_scoop: bool,
    pub can_fsd_boost: bool,
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
    WNC(ClassInfo<'a>),
    WO(ClassInfo<'a>),
    NS(ClassInfo<'a>),
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
        match rng.gen_range(0..42) {
            0 => StarClass::new("O"),
            1 => StarClass::new("OG"),
            2 => StarClass::new("B"),
            3 => StarClass::new("BG"),
            4 => StarClass::new("A"),
            5 => StarClass::new("AG"),
            6 => StarClass::new("F"),
            7 => StarClass::new("FG"),
            8 => StarClass::new("G"),
            9 => StarClass::new("GG"),
            10 => StarClass::new("K"),
            11 => StarClass::new("KG"),
            12 => StarClass::new("M"),
            13 => StarClass::new("MG"),
            14 => StarClass::new("L"),
            15 => StarClass::new("T"),
            16 => StarClass::new("Y"),
            17 => StarClass::new("AEBE"),
            18 => StarClass::new("TTS"),
            19 => StarClass::new("C"),
            20 => StarClass::new("CJ"),
            21 => StarClass::new("CN"),
            22 => StarClass::new("MS"),
            23 => StarClass::new("S"),
            24 => StarClass::new("W"),
            25 => StarClass::new("WC"),
            26 => StarClass::new("WN"),
            27 => StarClass::new("WNC"),
            28 => StarClass::new("WO"),
            29 => StarClass::new("NS"),
            30 => StarClass::new("D"),
            31 => StarClass::new("DA"),
            32 => StarClass::new("DAB"),
            33 => StarClass::new("DAV"),
            34 => StarClass::new("DAZ"),
            35 => StarClass::new("DB"),
            36 => StarClass::new("DBV"),
            37 => StarClass::new("DBZ"),
            38 => StarClass::new("DC"),
            39 => StarClass::new("DCV"),
            40 => StarClass::new("DQ"),
            41 => StarClass::new("BH"),
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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let scoopable: bool = true;
                let boostable: bool = false;


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let scoopable: bool = true;
                let boostable: bool = false;


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let scoopable: bool = true;
                let boostable: bool = false;


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let scoopable: bool = true;
                let boostable: bool = false;


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let scoopable: bool = true;
                let boostable: bool = false;


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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


                return Self::K(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
                let scoopable: bool = true;
                let boostable: bool = false;


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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

                let scoopable: bool = true;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 17.0);


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Red Dwarf";
                    description = "Ringed Class M Red Dwarf";
                    rarity = Rarity::new("R");
                }else {
                    type_name = "Red Dwarf";
                    description = "Class M Red Dwarf";
                    rarity = Rarity::new("VC");
                };


                return Self::M(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
                let scoopable: bool = true;
                let boostable: bool = false;


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
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
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Brown Dwarf";
                    description = "Ringed Class L Brown Dwarf";
                    rarity = Rarity::new("C");
                }else {
                    type_name = "Brown Dwarf";
                    description = "Class L Brown Dwarf";
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
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity = Rarity::new("C");
                if ringed {
                    type_name = "Ringed Brown Dwarf";
                    description = "Ringed Class T Brown Dwarf";
                }else {
                    type_name = "Brown Dwarf";
                    description = "Class T Brown Dwarf";
                };


                return Self::T(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
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
                let type_label: &str = "Y";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 2.0);


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
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Brown Dwarf";
                    description = "Ringed Class Y Brown Dwarf";
                    rarity = Rarity::new("C");
                }else {
                    type_name = "Brown Dwarf";
                    description = "Class Y Brown Dwarf";
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
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "AEBE" => { // Herbig AE/BE Protostars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "AE/BE";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 1.5);


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 396;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 3.003906;
                const MAX_MASS: f64 = 119.996094;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.093811030913;
                const MAX_RADIUS: f64 = 2.775085273904;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 3_353;
                const MAX_TEMP: i32 = 6_050;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.017480700463;
                const MAX_ORBITAL: f64 = 5_081_998.18432326;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.025193639688;
                const MAX_ROTATIONAL: f64 = 2.903862659294;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity = Rarity::new("R");
                if ringed {
                    type_name = "Ringed Protostar";
                    description = "Ringed Herbig AE/BE Protostar";
                }else {
                    type_name = "Protostar";
                    description = "Herbig AE/BE Protostar";
                };


                return Self::AEBE(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "TTS" => { // T Tauri Stars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "TTS";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = rng().gen_bool(1.0 / 7.0);


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 210;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.007812	;
                const MAX_MASS: f64 = 3.0;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.050780951834;
                const MAX_RADIUS: f64 = 2.167449673616;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 202;
                const MAX_TEMP: i32 = 13_143;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.071313013148;
                const MAX_ORBITAL: f64 = 27_280_924.4968275;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.087478558576;
                const MAX_ROTATIONAL: f64 = 26_930.8207407407;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed T Tauri Star";
                    description = "Ringed T Tauri Protostar";
                    rarity = Rarity::new("C");
                }else {
                    type_name = "T Tauri Star";
                    description = "T Tauri Protostar";
                    rarity = Rarity::new("VC");
                };


                return Self::TTS(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "C" => { // Class C Carbon Stars
                let type_name: &str = "Carbon Star";
                let description: &str = "Class C Carbon Star";
                let type_label: &str = "C";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("ER");


                const MIN_AGE: u64 = 4_010;
                const MAX_AGE: u64 = 13_062;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.460938;
                const MAX_MASS: f64 = 3.0;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 13.972106766355;
                const MAX_RADIUS: f64 = 214.740608599247;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 1_043;
                const MAX_TEMP: i32 = 5_524;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 65.094700979965;
                const MAX_ORBITAL: f64 = 9_320_610.99619777;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 18.929997942026;
                const MAX_ROTATIONAL: f64 = 499.399662499653;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::C(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "CJ" => { // Class CJ Carbon Stars
                let type_name: &str = "Carbon Star";
                let description: &str = "Class CJ Carbon Star";
                let type_label: &str = "CJ";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("VR");


                const MIN_AGE: u64 = 12_000;
                const MAX_AGE: u64 = 14_000;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.929687976837;
                const MAX_MASS: f64 = 0.988281;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 29.055603687854;
                const MAX_RADIUS: f64 = 29.851347646298;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 2_675;
                const MAX_TEMP: i32 = 2_865;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 26.331756365741;
                const MAX_ORBITAL: f64 = 10_310_019.602963;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 55.016202293715;
                const MAX_ROTATIONAL: f64 = 216.036551810463;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::CJ(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "CN" => { // Class CN Carbon Stars
                let type_name: &str = "Carbon Star";
                let description: &str = "Class CN Carbon Star";
                let type_label: &str = "CN";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("VR");


                const MIN_AGE: u64 = 12_396;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.929687976837;
                const MAX_MASS: f64 = 0.988281;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 29.055603027344;
                const MAX_RADIUS: f64 = 29.851347646298;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 2_630;
                const MAX_TEMP: i32 = 2_872;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 18.508179734155;
                const MAX_ORBITAL: f64 = 10_968_790.7690251;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 63.244884673924;
                const MAX_ROTATIONAL: f64 = 269.269212962963;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::CN(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "MS" => { // Class MS Carbon Stars
                let type_name: &str = "Carbon Star";
                let description: &str = "Class MS Carbon Star";
                let type_label: &str = "MS";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("R");


                const MIN_AGE: u64 = 3_718;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.929687976837;
                const MAX_MASS: f64 = 1.589844;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 29.055603687854;
                const MAX_RADIUS: f64 = 35.790951062545;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32  = 2_633;
                const MAX_TEMP: i32 = 3_699;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 18.833890335648;
                const MAX_ORBITAL: f64 = 10_009_841.7175037;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 30.08212962963;
                const MAX_ROTATIONAL: f64 = 316.624097222222;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::MS(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "S" => { // Class S Carbon Stars
                let type_name: &str = "Carbon Star";
                let description: &str = "Class S Carbon Star";
                let type_label: &str = "S";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("R");


                const MIN_AGE: u64 = 3_702;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.53125;
                const MAX_MASS: f64 = 2.070313;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 29.055603687854;
                const MAX_RADIUS: f64 = 172.92613902404;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 1_125;
                const MAX_TEMP: i32 = 3_699;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 17.567633616701;
                const MAX_ORBITAL: f64 = 10_401_054.72;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 29.669153136088;
                const MAX_ROTATIONAL: f64 = 552.459444444444;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::S(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "W" => { // Wolf-Rayet Stars
                let type_name: &str = "Wolf-Rayet Star";
                let description: &str = "Class W Wolf-Rayet Star";
                let type_label: &str = "W";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("ER");


                const MIN_AGE: u64 = 1_268;
                const MAX_AGE: u64 = 13_042;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.46875;
                const MAX_MASS: f64 = 2.09375;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 3.040821851394;
                const MAX_RADIUS: f64 = 9.874822253497;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 31_016;
                const MAX_TEMP: i32 = 192_822;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 6.49068946088;
                const MAX_ORBITAL: f64 = 9_023_863.13235318;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 1.099536651794;
                const MAX_ROTATIONAL: f64 = 43.494833594502;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::W(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "WN" => { // Class WN Wolf-Rayet Stars
                let type_name: &str = "Wolf-Rayet Star";
                let description: &str = "Class WN Wolf-Rayet Star";
                let type_label: &str = "WN";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("ER");


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 12_798;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.625;
                const MAX_MASS: f64 = 119.953125;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 3.00021344069;
                const MAX_RADIUS: f64 = 9.999816419842;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 7;
                const MAX_TEMP: i32 = 183_987;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.139477267801;
                const MAX_ORBITAL: f64 = 2_359_243.04083542;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.843257649745;
                const MAX_ROTATIONAL: f64 = 33.847250941134;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::WN(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "WC" => { // Class WC Wolf-Rayet Stars
                let type_name: &str = "Wolf-Rayet Star";
                let description: &str = "Class WC Wolf-Rayet Star";
                let type_label: &str = "WC";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("ER");


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 12_320;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.597656;
                const MAX_MASS: f64 = 60.0;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 3.000732066139;
                const MAX_RADIUS: f64 = 9.999358527678;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 8;
                const MAX_TEMP: i32 = 159_971;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.156556012188;
                const MAX_ORBITAL: f64 = 1_509_960.62814815;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.824660282847;
                const MAX_ROTATIONAL: f64 = 39.088547751296;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::WC(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "WNC" => { // Class WNC Wolf-Rayet Stars
                let type_name: &str = "Wolf-Rayet Star";
                let description: &str = "Class WNC Wolf-Rayet Star";
                let type_label: &str = "WNC";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("ER");


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 2;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 40.023438;
                const MAX_MASS: f64 = 119.984375;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 2.998941436655;
                const MAX_RADIUS: f64 = 9.999175959741;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 1;
                const MAX_TEMP: i32 = 65_525;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.140384012859;
                const MAX_ORBITAL: f64 = 1_330_582.66074074;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.883800100498;
                const MAX_ROTATIONAL: f64 = 5.181077405301;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::WNC(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "WO" => { // Class WO Wolf-Rayet Stars
                let type_name: &str = "Wolf-Rayet Star";
                let description: &str = "Class WO Wolf-Rayet Star";
                let type_label: &str = "WO";

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;
                let rarity: Rarity = Rarity::new("VR");


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 2;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 60.003906;
                const MAX_MASS: f64 = 119.996094;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 3.001471327225;
                const MAX_RADIUS: f64 = 9.99969421711;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 2;
                const MAX_TEMP: i32 = 65_532;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.131019580984;
                const MAX_ORBITAL: f64 = 1_193_413.30962963;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.875076064028;
                const MAX_ROTATIONAL: f64 = 6.065704210069;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::WO(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "NS" => { // Neutron Stars, Pulsars, Magnetars
                let type_name: &str;
                let description: &str;
                let type_label: &str = "NS";

                let scoopable: bool = false;
                let boostable: bool = true;
                let mut ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.425781;
                const MAX_MASS: f64 = 15.972656;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                let solar_radius: f64 = 0.0000000000001;

                const MIN_TEMP: i32 = 900_001;
                const MAX_TEMP: i32 = 985_067_520;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.036306160822;
                const MAX_ORBITAL: f64 = 15_967_999.4814931;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.000100000995;
                const MAX_ROTATIONAL: f64 = 222_814_898_821.345;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);

                let mut pulsar: bool = rng().gen_bool(1.0 / 2.0);
                let magnetar: bool = rng().gen_bool(1.0 / 420_000.0);
                if pulsar {
                    ringed = false;
                };
                if magnetar {
                    ringed = true;
                    pulsar = false;
                };


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Neutron Star";
                    description = "Ringed Neutron Star";
                    rarity = Rarity::new("ER");
                }else if pulsar {
                    type_name = "Pulsar";
                    description = "Pulsar";
                    rarity = Rarity::new("R");
                }else if magnetar {
                    type_name = "Magnetar";
                    description = "Magnetar";
                    rarity = Rarity::new("L");
                } else {
                    type_name = "Neutron Star";
                    description = "Neutron Star";
                    rarity = Rarity::new("R");
                };


                return Self::NS(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            "D" => { // White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "D";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 3_792;
                const MAX_AGE: u64 = 13_062;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.203125;
                const MAX_MASS: f64 = 1.34375;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.00307855717;
                const MAX_RADIUS: f64 = 0.023012974983;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 4_701;
                const MAX_TEMP: i32 = 25_360;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.21584549515;
                const MAX_ORBITAL: f64 = 12_688_206.3017951;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006999597196;
                const MAX_ROTATIONAL: f64 = 1.910879991319;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed White Dwarf";
                    description = "Ringed Class D Wite Dwarf";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "White Dwarf";
                    description = "Class D White Dwarf";
                    rarity = Rarity::new("VR");
                };


                return Self::D(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DA" => { // Class DA White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DA";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 1_298;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.136719;
                const MAX_MASS: f64 = 1.398438;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.002255576478;
                const MAX_RADIUS: f64 = 0.024537013528;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 4_503;
                const MAX_TEMP: i32 = 27_735;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.08118039066;
                const MAX_ORBITAL: f64 = 17_641_239.144184;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006946098264;
                const MAX_ROTATIONAL: f64 = 170.867172241211;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed White Dwarf";
                    description = "Ringed Class DA Wite Dwarf";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "White Dwarf";
                    description = "Class DA White Dwarf";
                    rarity = Rarity::new("VR");
                };


                return Self::DA(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DAB" => { // Class DAB White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DAB";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 4_684;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.273438;
                const MAX_MASS: f64 = 0.710938;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.008850097052;
                const MAX_RADIUS: f64 = 0.021911620417;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 8_001;
                const MAX_TEMP: i32 = 24_015;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.090860495417;
                const MAX_ORBITAL: f64 = 16_629_121.3274074;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006948378576;
                const MAX_ROTATIONAL: f64 = 39.187155671296;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed White Dwarf";
                    description = "Ringed Class DAB Wite Dwarf";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "White Dwarf";
                    description = "Class DAB White Dwarf";
                    rarity = Rarity::new("VR");
                };


                return Self::DAB(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DAV" => { // Class DAV White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DAV";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 420.0);


                const MIN_AGE: u64 = 4_634;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.273438;
                const MAX_MASS: f64 = 0.710938;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.009155272466;
                const MAX_RADIUS: f64 = 0.021789548526;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 12_001;
                const MAX_TEMP: i32 = 24_003;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.089758178565;
                const MAX_ORBITAL: f64 = 16_942_580.2650275;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006951642919;
                const MAX_ROTATIONAL: f64 = 3.297920645255;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Pulsating White Dwarf";
                    description = "Ringed Class DAV Wite Dwarf";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "Pulsating White Dwarf";
                    description = "Class DAV White Dwarf";
                    rarity = Rarity::new("ER");
                };


                return Self::DAV(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DAZ" => { // Class DAZ White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DAZ";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 3_852;
                const MAX_AGE: u64 = 13_050;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.304688;
                const MAX_MASS: f64 = 1.332031;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.003566249246;
                const MAX_RADIUS: f64 = 0.019561765636;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 6_106;
                const MAX_TEMP: i32 = 24_638;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.226992706736;
                const MAX_ORBITAL: f64 = 16_140_341.758728;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006972483877;
                const MAX_ROTATIONAL: f64 = 11.068577835648;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Metallic White Dwarf";
                    description = "Ringed Class DAZ Wite Dwarf";
                    rarity = Rarity::new("L");
                }else {
                    type_name = "Metallic White Dwarf";
                    description = "Class DAZ White Dwarf";
                    rarity = Rarity::new("ER");
                };


                return Self::DAZ(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DB" => { // Class DB White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DB";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 4_726;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.273438;
                const MAX_MASS: f64 = 0.710938;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.008972167505;
                const MAX_RADIUS: f64 = 0.021331785766;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 12_002;
                const MAX_TEMP: i32 = 24_045;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.099657015116;
                const MAX_ORBITAL: f64 = 16_218_796.2311111;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006949402118;
                const MAX_ROTATIONAL: f64 = 47.034012683137;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed White Dwarf";
                    description = "Ringed Class DB Wite Dwarf";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "White Dwarf";
                    description = "Class DB White Dwarf";
                    rarity = Rarity::new("VR");
                };


                return Self::DB(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DBV" => { // Class DBV White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DBV";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 4_798;
                const MAX_AGE: u64 = 12_063;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.289063;
                const MAX_MASS: f64 = 0.707031;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.009063720345;
                const MAX_RADIUS: f64 = 0.020782468728;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 12_002;
                const MAX_TEMP: i32 = 24_030;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.142419310637;
                const MAX_ORBITAL: f64 = 15_581_667.5612222;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006961906574;
                const MAX_ROTATIONAL: f64 = 56.196827057847;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Pulsating White Dwarf";
                    description = "Ringed Class DBV Wite Dwarf";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "Pulsating White Dwarf";
                    description = "Class DBV White Dwarf";
                    rarity = Rarity::new("ER");
                };


                return Self::DBV(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DBZ" => { // Class DBZ White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DBZ";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 4_642;
                const MAX_AGE: u64 = 13_040;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.308594;
                const MAX_MASS: f64 = 0.710938;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.009338378145;
                const MAX_RADIUS: f64 = 0.020233153127;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 12_014;
                const MAX_TEMP: i32 = 23_967;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.207983262801;
                const MAX_ORBITAL: f64 = 14_565_756.7514317;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.007056985498;
                const MAX_ROTATIONAL: f64 = 1.977933169178;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Mettalic White Dwarf";
                    description = "Ringed Class DBZ Wite Dwarf";
                    rarity = Rarity::new("L");
                }else {
                    type_name = "Metallic White Dwarf";
                    description = "Class DBZ White Dwarf";
                    rarity = Rarity::new("ER");
                };


                return Self::DBZ(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DC" => { // Class DC White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DC";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 1_368;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.140625;
                const MAX_MASS: f64 = 1.433594;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.000792499803;
                const MAX_RADIUS: f64 = 0.024597975127;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 4_197;
                const MAX_TEMP: i32 = 25_232;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.063943922257;
                const MAX_ORBITAL: f64 = 35_001_189.2064854;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006945021377;
                const MAX_ROTATIONAL: f64 = 101.582048611111;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed White Dwarf";
                    description = "Ringed Class DC Wite Dwarf";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "White Dwarf";
                    description = "Class DC White Dwarf";
                    rarity = Rarity::new("VR");
                };


                return Self::DC(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DCV" => { // Class DCV White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DCV";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 7_488;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.273438;
                const MAX_MASS: f64 = 0.710938;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.009094237239;
                const MAX_RADIUS: f64 = 0.020782468728;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 8_001;
                const MAX_TEMP: i32 = 12_000;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.111695002801;
                const MAX_ORBITAL: f64 = 18_198_257.6571134;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.006946161771;
                const MAX_ROTATIONAL: f64 = 7.813709204468;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Pulsating White Dwarf";
                    description = "Ringed Class DCV Wite Dwarf";
                    rarity = Rarity::new("ER");
                }else {
                    type_name = "Pulsating White Dwarf";
                    description = "Class DCV White Dwarf";
                    rarity = Rarity::new("VR");
                };


                return Self::DCV(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "DQ" => { // Class DQ White Dwarfs
                let type_name: &str;
                let description: &str;
                let type_label: &str = "DQ";

                let scoopable: bool = false;
                let boostable: bool = true;
                let ringed: bool = rng().gen_bool(1.0 / 42.0);


                const MIN_AGE: u64 = 2_900;
                const MAX_AGE: u64 = 13_034;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 0.183594;
                const MAX_MASS: f64 = 1.433594;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.000701057538;
                const MAX_RADIUS: f64 = 0.02407980297;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 4_428;
                const MAX_TEMP: i32 = 25_132;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 37.316534392257;
                const MAX_ORBITAL: f64 = 2_754_284.08888889;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.142568166458;
                const MAX_ROTATIONAL: f64 = 1.567897816088;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                let rarity: Rarity;
                if ringed {
                    type_name = "Ringed Carbon Dwarf";
                    description = "Ringed Class DQ Wite Dwarf";
                    rarity = Rarity::new("L");
                }else {
                    type_name = "Carbon Dwarf";
                    description = "Class DQ White Dwarf";
                    rarity = Rarity::new("ER");
                };


                return Self::DQ(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });  
            },
            "BH" => { // Black Holes
                let type_name: &str = "Black Hole";
                let description: &str = "Stellar Remnant";
                let type_label: &str = "BH";
                let rarity: Rarity = Rarity::new("VR");

                let scoopable: bool = false;
                let boostable: bool = false;
                let ringed: bool = false;


                const MIN_AGE: u64 = 0;
                const MAX_AGE: u64 = 13_065;
                let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

                const MIN_MASS: f64 = 2.515625;
                const MAX_MASS: f64 = 220.097656;
                let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

                const MIN_RADIUS: f64 = 0.000100007223;
                const MAX_RADIUS: f64 = 0.000839237567;
                let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

                const MIN_TEMP: i32 = 3_327;
                const MAX_TEMP: i32 = 10_849;
                let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

                const MIN_ORBITAL: f64 = 0.017480700463;
                const MAX_ORBITAL: f64 = 6_576_330.64245741;
                let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
                let orbital_secs: f64 = orbital_range * 86_400.0;
                let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

                const MIN_ROTATIONAL: f64 = 0.00000001;
                const MAX_ROTATIONAL: f64 = 0.11111111;
                let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
                let rotational_secs: f64 = rotational_range * 86_400.0;
                let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


                return Self::BH(ClassInfo {
                    type_label,
                    type_name,
                    description,
                    rarity,
                    ringed,
                    scoopable,
                    boostable,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            _ => rand::random(),
        }
    }

    fn extract_stats(&self) -> Stats {
        match self {
            Self::O(i) | Self::OG(i) | Self::B(i) | Self::BG(i) | Self::A(i)
             | Self::AG(i) | Self::F(i) | Self::FG(i) | Self::G(i) | Self::GG(i)
             | Self::K(i) | Self::KG(i) | Self::M(i) | Self::MG(i) | Self::L(i)
             | Self::T(i) | Self::Y(i) | Self::AEBE(i) | Self::TTS(i) | Self::C(i)
             | Self::CJ(i) | Self::CN(i) | Self::MS(i) | Self::S(i) | Self::W(i) 
             | Self::WN(i) | Self::WC(i) | Self::WNC(i) | Self::WO(i) | Self::NS(i) 
             | Self::D(i) | Self::DA(i) | Self::DAB(i) | Self::DAV(i) | Self::DAZ(i) 
             | Self::DB(i) | Self::DBV(i) | Self::DBZ(i) | Self::DC(i) | Self::DCV(i) 
             | Self::DQ(i) | Self::BH(i) =>
             {
                return Stats{
                    label: i.type_label,
                    class_name: i.type_name,
                    description: i.description,
                    rarity: i.rarity.fetch_rarity(),
                    ringed: i.ringed,
                    can_fuel_scoop: i.scoopable,
                    can_fsd_boost: i.boostable,
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
}

impl<'a> Star<'a> {
    pub fn new(name: &'a str, class: &'a str) -> Self {
        let class = StarClass::new(class);
        Star {
            name,
            class,
        }
    }

    pub fn stats(&self) -> Stats {
        let x: &StarClass = &self.class;
        x.extract_stats()
    }
}
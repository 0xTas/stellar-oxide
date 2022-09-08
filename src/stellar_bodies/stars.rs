use std::time::Duration;

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
    pub rarity: &'a str,
    pub scoopable: bool,
    pub boostable: bool,
    pub supergiant: bool,
    pub is_planet: bool,
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
    pub is_planet: bool,
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

        match class {
            "O" => {
                let ringed: bool = false;
                let class_name: &str;
                let description: &str;
                let rarity: &str;

                let scoopable: bool = true;
                let boostable: bool = false;
                let supergiant: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.42);
                let is_planet: bool = rng().gen_bool(1.0 / 42_069.1337);


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


                if solar_radius >= 360.0 {
                    supergiant = true;
                    is_planet = false;
                    is_primary = true;
                    rarity = "Extremely Rare";
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed Class O Star";
                    description = "Ringed Class O Star";
                    rarity = "Legendary";
                }else {
                    class_name = "Class O Star";
                    description = "Class O Star";
                    rarity = "Very Rare";
                };

                return Self::O(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_planet,
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
                let ringed: bool = rng().gen_bool(1.0 / 20.0);
                let class_name: &str;
                let description: &str;
                let rarity: &str;

                let scoopable: bool = true;
                let boostable: bool = false;
                let supergiant: bool = rng().gen_bool(1.0 / 42.0);
                let is_primary: bool = rng().gen_bool(1.0 / 1.7);
                let is_planet: bool = rng().gen_bool(1.0 / 17.0);


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


                if solar_radius >= 297.0 {
                    supergiant = true;
                    is_planet = false;
                    is_primary = true;
                    ringed = false;
                    rarity = "Very Rare";
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed B Class Star";
                    description = "CHANGE ME (Ringed)";
                    rarity = "Very Rare";
                }else {
                    class_name = "B Class Star";
                    description = "CHANGE ME";
                    rarity = "Rare";
                };


                return Self::B(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_planet,
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
                let ringed: bool;
                let class_name: &str;
                let description: &str;
                let rarity: &str;

                let supergiant: bool;
                let scoopable: bool = true;
                let boostable: bool = false;
                let is_primary: bool = rng().gen_bool(1.0 / 1.4);
                let is_planet: bool = rng().gen_bool(1.0 / 42_666.0);


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


                if solar_radius >= 150.0 {
                    supergiant = true;
                    is_planet = false;
                    is_primary = true;
                    ringed = false;
                    rarity = "Very Rare"
                }else {
                    supergiant = false;
                };

                if ringed {
                    class_name = "Ringed A Class Star";
                    description = "Ringed A Class Star";
                    rarity = "Extremely Rare";
                }else {
                    class_name = "A Class (Blue-White) Star";
                    description = "CHANGE ME";
                    rarity = "Uncommon";
                };


                return Self::A(ClassInfo {
                    ringed,
                    type_name,
                    description,
                    rarity,
                    scoopable,
                    boostable,
                    supergiant,
                    is_planet,
                    is_primary,
                    age,
                    solar_masses,
                    solar_radius,
                    surface_temp,
                    orbital_period,
                    rotational_period,
                });
            },
            // "F" => {
            //     let ringed: bool;
            //     let class_name: &str;
            //     let description: &str;
            //     let rarity: &str;

            //     let scoopable: bool;
            //     let boostable: bool;
            //     let supergiant: bool;
            //     let is_primary: bool;
            //     let is_planet: bool;


            //     const MIN_AGE: u64;
            //     const MAX_AGE: u64;
            //     let age: u64 = rng().gen_range(MIN_AGE..=MAX_AGE);

            //     const MIN_MASS: f64;
            //     const MAX_MASS: f64;
            //     let solar_masses: f64 = rng().gen_range(MIN_MASS..=MAX_MASS);

            //     const MIN_RADIUS: f64;
            //     const MAX_RADIUS: f64;
            //     let solar_radius: f64 = rng().gen_range(MIN_RADIUS..=MAX_RADIUS);

            //     const MIN_TEMP: i32;
            //     const MAX_TEMP: i32;
            //     let surface_temp: i32 = rng().gen_range(MIN_TEMP..=MAX_TEMP);

            //     const MIN_ORBITAL: f64;
            //     const MAX_ORBITAL: f64;
            //     let orbital_range: f64 = rng().gen_range(MIN_ORBITAL..=MAX_ORBITAL);
            //     let orbital_secs: f64 = orbital_range * 86_400.0;
            //     let orbital_period: Duration = Duration::from_secs_f64(orbital_secs);

            //     const MIN_ROTATIONAL: f64;
            //     const MAX_ROTATIONAL: f64;
            //     let rotational_range: f64 = rng().gen_range(MIN_ROTATIONAL..=MAX_ROTATIONAL);
            //     let rotational_secs: f64 = orbital_range * 86_400.0;
            //     let rotational_period: Duration = Duration::from_secs_f64(rotational_secs);


            //     if solar_radius >= 420.0 {
            //         supergiant = true;
            //         is_planet = false;
            //         is_primary = true;
            //         ringed = false;
            //     }else {
            //         supergiant = false;
            //     };

            //     if ringed {
            //         class_name = "CHANGE ME (Ringed)";
            //         description = "CHANGE ME (Ringed)";
            //     }else {
            //         class_name = "CHANGE ME";
            //         description = "CHANGE ME";
            //     };


            //     return Self::CHANGEME(ClassInfo {
            //         ringed,
            //         type_name,
            //         description,
            //         rarity,
            //         scoopable,
            //         boostable,
            //         supergiant,
            //         is_planet,
            //         is_primary,
            //         age,
            //         solar_masses,
            //         solar_radius,
            //         surface_temp,
            //         orbital_period,
            //         rotational_period,
            //     });
            // }
            _ => {


                // let ringed: bool;
                // let class_name: &str;
                // let description: &str;
                // let rarity: &str;

                // let scoopable: bool;
                // let boostable: bool;
                // let supergiant: bool;
                // let is_primary: bool;
                // let is_planet: bool;


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


                // if solar_radius >= 420.0 {
                //     supergiant = true;
                //     is_planet = false;
                //     is_primary = true;
                //     ringed = false;
                // }else {
                //     supergiant = false;
                // };

                // if ringed {
                //     class_name = "CHANGE ME (Ringed)";
                //     description = "CHANGE ME (Ringed)";
                // }else {
                //     class_name = "CHANGE ME";
                //     description = "CHANGE ME";
                // };


                // return Self::CHANGEME(ClassInfo {
                //     ringed,
                //     type_name,
                //     description,
                //     rarity,
                //     scoopable,
                //     boostable,
                //     supergiant,
                //     is_planet,
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
                    is_planet: i.is_planet,
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
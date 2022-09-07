use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub struct ClassInfo<'a> {
    pub type_name: &'a str,
    pub scoopable: bool,
    pub boostable: bool,
}

#[derive(Debug)]
pub struct Stats<'a> {
    pub class_name: &'a str,
    pub can_fuel_scoop: bool,
    pub can_fsd_boost: bool,
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
            _ => StarClass::new("Proto"),
        }
    }
}

impl<'a> StarClass<'a> {
    pub fn new(class: &str) -> Self {

        match class {
            "O" => Self::O(ClassInfo { type_name: "Class O Star", scoopable: true, boostable: false }),
            "B" => Self::B(ClassInfo { type_name: "Class B Star", scoopable: true, boostable: false }),
            "A" => Self::A(ClassInfo { type_name: "Class A Star", scoopable: true, boostable: false }),
            "F" => Self::F(ClassInfo { type_name: "Class F Star", scoopable: true, boostable: false }),
            "G" => Self::G(ClassInfo { type_name: "Class G Star", scoopable: true, boostable: false }),
            "K" => Self::K(ClassInfo { type_name: "Class K Star", scoopable: true, boostable: false }),
            "M" => Self::M(ClassInfo { type_name: "Class M Star", scoopable: true, boostable: false }),
            "L" => Self::L(ClassInfo { type_name: "Class L (Brown Dwarf)", scoopable: false, boostable: false }),
            "T" => Self::T(ClassInfo { type_name: "Class T (Brown Dwarf)", scoopable: false, boostable: false }),
            "Y" => Self::Y(ClassInfo { type_name: "Class Y (Brown Dwarf)", scoopable: false, boostable: false }),
            "Proto" => Self::Proto(ClassInfo { type_name: "Protostar", scoopable: false, boostable: false }),
            "Carbon" => Self::Carbon(ClassInfo { type_name: "Carbon Star", scoopable: false, boostable: false }),
            "WR" => Self::WR(ClassInfo { type_name: "Wolf-Rayet Star", scoopable: false, boostable: false }),
            "WD" => Self::WD(ClassInfo { type_name: "White Dwarf", scoopable: false, boostable: true }),
            "NS" => Self::NS(ClassInfo { type_name: "Neutron Star", scoopable: false, boostable: true }),
            "BH" => Self::BH(ClassInfo { type_name: "Black Hole", scoopable: false, boostable: false }),
            _ => rand::random(),
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
                    class_name: i.type_name,
                    can_fuel_scoop: i.scoopable,
                    can_fsd_boost: i.boostable,
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
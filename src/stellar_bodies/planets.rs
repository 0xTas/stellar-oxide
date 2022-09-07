use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub struct PlanetTypeProperties<'a> {
    description: &'a str,
    low_temp: i32,
    high_temp: i32,

}
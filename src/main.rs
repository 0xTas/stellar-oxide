// use rand::Rng;
use stellar_oxide::bodies::stars::Star;
use stellar_oxide::bodies::planets::Planet;
use stellar_oxide::{cls, wait, input, create_random_star, create_random_planet};


/* Current goal is to generate random star systems
    and display various info about them in a loop. */


fn main() {
    loop {

        /* Main Loop */
        cls();
        println!(" Options:");
        println!(" [O, OG, B, BG, A, AG, F, FG, G, GG, K, KG, M, MG, L, T, Y, AEBE, TTS, C, CJ, CN, MS, S, 
        W, WC, WN, WNC, WO, NS, D, DA, DAB, DAV, DAZ, DB, DBV, DBZ, DC, DCV, DQ, BH]");
        println!(" What class of star are you looking for?");
        print!("~> ");
        wait(0);
        let answer_star: String = input();
        wait(42);
        cls();

        println!(" Options (append \"(R)\" for ringed variants):");
        println!(" [AW, WW, WG, RKB, ICB, ELW, HMC, RIW, MRB, HGG, GGGG, CIGG, CIIGG, CIIIGG, CIVGG, CVGG, HRGG, GGWABL, GGWWBL]");
        println!(" What type of planet are you looking for?");
        print!("~> ");
        wait(0);
        let answer_planet: String = input();
        wait(420);

        loop {
            cls();
            let star: Star = create_random_star();
            let planet: Planet = create_random_planet();
            let answer_star: &str = answer_star.trim();
            let answer_planet: &str = answer_planet.trim();
            if star.stats().label == answer_star && planet.stats().label.to_lowercase() == answer_planet.to_lowercase() {
                println!(" Generated requested combination: '{}' and '{}'.", answer_star, answer_planet);
                println!();
                println!(" {:#?}", star);
                for _ in 0..=69 { print!("-"); };
                println!("\n {:#?}", planet);
                wait(30000);
                break;
            }else {
                println!(" Searching for combination: '{}' and '{}'...", answer_star, answer_planet);
                println!();
                println!(" {:#?}", star);
                for _ in 0..=69 { print!("-"); };
                println!("\n {:#?}", planet);
                wait(2);
            };

        }
    }
}

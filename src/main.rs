use rand::Rng;
use oasis::stellar_bodies::stars::Star;
use oasis::stellar_bodies::planets::Planet;
use oasis::{cls, wait, input, create_random_star, create_random_planet};


/* Current goal is to generate random star systems
    and display various info about them in a loop.
    
    Future goal is to use that procedural API
    as the foundation for the game's environment.*/


fn main() {
    loop {
        cls();
        wait(420);
        println!("\n Another day out here on the edge of the Galaxy..\n");
        print!(" Would you like to boot into StellarOS? [Y/n] #> ");

        wait(0);
        let answer: String = input();

        for chr in answer.chars() {
            let chr = chr.to_lowercase().to_string();
            if chr == "y" || chr == "m" || chr == "a" || chr == "d" || chr == "s" || chr == "t" {
                println!("");
                print!(" >Loading");
                for _ in 0..36 {
                    print!(".");
                    let amount = rand::thread_rng().gen_range(42..=69);
                    wait(amount);
                };
                wait(1500);
                println!("Done.");
                wait(1000);
                println!("");

                let current_star: Star = create_random_star();
                let star_stats = current_star.stats();
                // println!(" Your Star: {:#?}", current_star);
                println!(" You've been here for quite some time already...");
                println!(" Your current location is the system {}.\n It is a remote {} located on the far side of the galaxy.", current_star.name, current_star.stats().class_name);
                wait(7000);
                println!(" Here are the details of the current system's star:\n {:#?}", star_stats);
                wait(20000);
                
                let random_planet: Planet = create_random_planet();
                println!();
                println!(" There is one interesting body in this system, you stayed around for a few days to examine it further.");
                wait(2420);
                println!();
                let get = random_planet.stats();
                println!(" These are the details you discovered about the {} known as {}:\n {:#?}\n", get.type_name, random_planet.name, random_planet);
                wait(60000);
                break;
            } else {
                wait(700);
                println!("\n Well, there isn't much else to do out here...\n");
                wait(2500);
                print!(" *You sit and stare out the window of your cockpit at the stars for a while*");
                wait(3000);
                for _ in 0..3 {
                    print!(". ");
                    wait(1000);
                };
                println!("");
                wait(2420);
                println!("\n Goodbye!\n");
                wait(2000);
                break;
            }
        } 

    }
}

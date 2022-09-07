use std::{io, thread::sleep};
use rand::Rng;
use oasis::{cls, dur, flush, create_random_star, stellar_bodies::stars::Star};


/* Current goal is to generate random star systems
    and display various info about them in a loop.
    
    Future goal is to use that procedural API
    as the foundation for the game's environment.*/


fn main() {
    loop {
        cls();
        sleep(dur(420));
        println!("\n Another day out here on the edge of the Galaxy..\n");
        print!(" Would you like to boot into StellarOS? [Y/n] #> ");

        let mut answer = String::new();

        flush();
        io::stdin()
            .read_line(&mut answer)
            .expect("Stdin-read Failed!");

        for chr in answer.chars() {
            let chr = chr.to_lowercase().to_string();
            if chr == "y" || chr == "m" || chr == "a" || chr == "d" || chr == "s" || chr == "t" {
                println!("");
                print!(" >Loading");
                for _ in 0..36 {
                    print!(".");
                    flush();
                    let amount = rand::thread_rng().gen_range(42..=69);
                    sleep(dur(amount));
                };
                sleep(dur(1500));
                println!("Done.");
                sleep(dur(1000));
                println!("");

                let current_star: Star = create_random_star();

                // println!(" Your Star: {:#?}", current_star);
                println!(" You've been here for quite some time already...");
                println!(" Your current location is the system {}.\n It is a remote {} located on the far side of the galaxy.", current_star.name, current_star.class_name());
                flush();
                sleep(dur(17000));
                break;
            } else {
                sleep(dur(700));
                println!("\n Well, there isn't much else to do out here...\n");
                flush();
                sleep(dur(2500));
                print!(" *You sit and stare out the window of your cockpit at the stars*");
                flush();
                sleep(dur(3000));
                for _ in 0..3 {
                    print!(". ");
                    flush();
                    sleep(dur(1000));
                };
                println!("");
                flush();
                sleep(dur(2420));
                println!("\n Goodbye!\n");
                flush();
                sleep(dur(2000));
                break;
            }
        } 

    }
}

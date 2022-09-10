## What?

The current goal of this project is to create an API to generate data resembling pseudorandom procedural star systems
inspired by Elite Dangerous with a variety of generated values that can be viewed and used by the consumer.

Right now it creates stars and planet structures but with lousy linear rng.

So, probably don't use it. It isn't ready yet.

---

## Why?

Because I like space, and I want to learn Rust, and I need to learn proper project structuring and management.
This sounded like a fun project to tackle over time now that I've gotten through about half of the Rust Book.
Traits and lifetimes and generics and all that still confuse me a bit, but thankfully the compiler is an absolute bro,
so I've been managing alright thus far.

---

## What can I do with it?

Not much. Currently you can compile it and run the binary for a never-ending loop of randomly generated stars and planets.
It will ask you for a desired combination and then pause to let you take a look when it generates that combo by chance.

Or, you can run `cargo test --lib` and watch it pass a few trivial tests. Woohoo.

There's a create_random_star() function and a create_random_planet() function, and you can get a struct containing their randomly-generated stats.
If you think that could be of use to you, or if it seems like a good jumping-off point for your own machinations, then have fun. 
<br>
:star2: :milky_way: :ringed_planet:
<br>
---
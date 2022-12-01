mod elves;
mod input;

use elves::*;
use input::*;

fn main() {
    // the 1st star
    if let Some(cargo) = Input::load_cargo("data/1.in") {
        let mut elves = Elves::new(cargo);
        println!(
            "1. The strongest elf carries {:?} calories! What a strong beast!",
            elves.maximum_supply(),
        );
    } else {
        println!("1. That's sad! Elves forgot their food at home!");
    }
    
    // the 2nd star
    if let Some(cargo) = Input::load_cargo("data/2.in") {
        let mut elves = Elves::new(cargo);
        println!(
            "2. The strongest 3 elves carry {:?} calories! They are so thoughtful!",
            elves.total_calories_by_top(3),
        );
    } else {
        println!("2. That's sad! Elves lost their food somewhere!");
    }
}

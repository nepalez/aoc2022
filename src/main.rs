mod elves;
mod input;
mod rock_paper_scissors;

use elves::*;
use rock_paper_scissors::*;
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
    
    // the 3rd star
    if let Some(data) = Input::load_strategy("data/3.in") {
        if let Some(strategy) = Strategy::misinterpret(data) {
            println!("3. My score should be {}", strategy.score());
        } else {
            println!("3. The strategy is written in Quenya. I need the translator.");
        }
    } else {
        println!("3. Alas :(. The elf took his strategy and run away!");
    }

    // the 4th star
    if let Some(data) = Input::load_strategy("data/3.in") {
        if let Some(strategy) = Strategy::interpret(data) {
            println!("4. Oh, no! My score is {}", strategy.score());
        } else {
            println!("4. The strategy is written in Quenya. I need the translator.");
        }
    } else {
        println!("4. Alas :(. The elf took his strategy and run away!");
    }
}

mod day1;
mod day2;
mod day3;

use day1::*;
use day2::*;
use day3::*;

fn main() {
    if let Some(mut elves) = Elves::load_from("data/1.in") {
        println!("1.1: The strongest elf carries {:?} calories", elves.maximum_supply());
        println!("1.2: The strongest 3 elves carry {:?} calories", elves.total_calories_by_top(3));
    } else {
        println!("1: Cannot parse input!");
    }

    if let Some(input) = day2::Input::load_from("data/2.in") {
        if let Some(strategy) = Strategy::misinterpret(&input) {
            println!("2.1. Misinterpreted input's score is {}", strategy.score());
        }
        if let Some(strategy) = Strategy::interpret(&input) {
            println!("2.2. A proper input's score is {}", strategy.score());
        }
    } else {
        println!("2: Cannot parse input!");
    }
    
    if let Some(cargo) = Cargo::load_from("data/3.in") {
        println!("3.1: The individuals score is {:?}", cargo.individuals_score());
        println!("3.2: The groups score is {:?}", cargo.groups_score());
    } else {
        println!("3: Cannot parse input!");
    }
}

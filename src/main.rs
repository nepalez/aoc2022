mod elves;
mod input;

use elves::*;
use input::*;

fn main() {
    if let Some(cargo) = Input::load_cargo("data/1.in") {
        let elves = Elves::new(cargo);
        println!(
            "The strongest elf carries {:?} calories! What a strong beast!",
            elves.maximum_supply(),
        );
    } else {
        println!("That's sad! Elves forgot their food at home!");
    }
}

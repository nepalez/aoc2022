mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    if let Some(elves) = day1::Elves::load_from("data/1.in") {
        println!(
            "1.1: The strongest elf carries {:?} calories",
            elves.calories_carried_by_top(1)
        );
        println!(
            "1.2: The strongest 3 elves carry {:?} calories",
            elves.calories_carried_by_top(3)
        );
    } else {
        println!("1: Cannot parse the input!");
    }

    if let Some(game) = day2::Game::load_from("data/2.in") {
        println!("2.1. Misinterpreted score is {}", game.wrong_score());
        println!("2.2. A proper score is {}", game.right_score());
    } else {
        println!("2: Cannot parse the input!");
    }

    if let Some(cargo) = day3::Cargo::load_from("data/3.in") {
        println!(
            "3.1: The individuals score is {:?}",
            cargo.individuals_score()
        );
        println!("3.2: The groups score is {:?}", cargo.groups_score());
    } else {
        println!("3: Cannot parse the input!");
    }

    if let Some(pairs) = day4::Pairs::load_from("data/4.in") {
        println!(
            "4.1: The number of pairs where one assigment fully contains the other is {:?}",
            pairs.count_fully_contained()
        );
        println!(
            "4.2: The number of pairs with overlapping assignments is {:?}",
            pairs.count_overlapped()
        );
    } else {
        println!("4: Cannot parse the input!");
    }

    if let Some(crane) = day5::Crane::load_from("data/5.in") {
        println!("5.1: Unmodified crane ended with {:?}", crane.apply_old());
        println!("5.2: Modified crane ended with {:?}", crane.apply_new());
    } else {
        println!("5: Cannot parse the input!");
    }

    if let Some(stream) = day6::Stream::load_from("data/6.in") {
        println!("6.1: Packet starts at {:?}", stream.start_packet());
        println!("6.2: Message starts at {:?}", stream.start_message());
    } else {
        println!("6: Cannot parse the input!");
    }

    if let Some(tree) = day7::Tree::load_from("data/7.in") {
        println!(
            "7.1: Sum of folder sizes is {:?}",
            tree.sum_of_folders_up_to(100000)
        );
        println!("7.2: Space to drop: {:?}", tree.space_to_drop(40000000));
    } else {
        println!("7: Cannot parse the input!");
    }

    if let Some(forest) = day8::Forest::load_from("data/8.in") {
        println!(
            "8.1: The number of visible trees is {}",
            forest.count_visible()
        );
        println!("8.2: The best tree's score is {}", forest.best_score());
    } else {
        println!("8: Cannot parse the input!");
    }

    if let Some(motions) = day9::Motions::load_from("data/9.in") {
        println!(
            "9.1: The tail of 2-knot-rope visits {:?} positions",
            motions.count_tail_positions(2)
        );
        println!(
            "9.2: The tail of 10-knot-rope visits {:?} positions",
            motions.count_tail_positions(10)
        );
    } else {
        println!("9: Cannot parse the input!");
    }
}

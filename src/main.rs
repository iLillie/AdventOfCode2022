mod day_1;
mod day_2;
mod day_3;
use day_1::CalorieCounting;
use day_2::RockPaperScissors;
use day_3::RucksackReorganization;

fn main() {
    //run_day_one();
    //run_day_two();
    run_day_three();
}

fn run_day_one() {
    let mut calorie_counting: CalorieCounting = CalorieCounting::default();
    calorie_counting.init();

    // Part 1
    calorie_counting.get_highest_total();

    // Part 2
    calorie_counting.get_top_three_highest_total();
}

fn run_day_two() {
    let mut rock_paper_scissors = RockPaperScissors::default();
    rock_paper_scissors.init();
    let part_one_score = rock_paper_scissors.clone().get_player_score();
    println!("    Part 1 score: {:?}", part_one_score);

    rock_paper_scissors.recalculate_player_moves();
    let part_two_score = rock_paper_scissors.clone().get_player_score();
    println!("    Part 2 score: {:?}", part_two_score);

}

fn run_day_three() {
    let mut rocksack_reorganization = RucksackReorganization::default();
    rocksack_reorganization.init();

    let sum_of_duplicate = RucksackReorganization::get_duplicate_priorty_sum(rocksack_reorganization.rucksack_list.clone());
    println!("Day 3:");
    println!("  Part 1: {}", sum_of_duplicate);
    println!("  Part 2: {}", RucksackReorganization::solve_part2(rocksack_reorganization.rucksack_list));
}
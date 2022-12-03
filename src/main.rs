mod day_1;
mod day_2;
use day_1::CalorieCounting;
use day_2::RockPaperScissors;

fn main() {
    //run_day_one();
    run_day_two()
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
    rock_paper_scissors.get_player_score()
}
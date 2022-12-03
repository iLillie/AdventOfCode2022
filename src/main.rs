mod day_1;
use day_1::CalorieCounting;

fn main() {
    run_day_one();
}

fn run_day_one() {
    let mut calorie_counting: CalorieCounting = CalorieCounting::default();
    calorie_counting.init();
    calorie_counting.print_stats();

    // Part 1
    calorie_counting.get_highest_total();

    // Part 2
    calorie_counting.get_top_three_highest_total();
}

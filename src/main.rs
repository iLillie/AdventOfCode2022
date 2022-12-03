mod day_1;
use day_1::DayOne;

fn main() {
    run_day_one();
}

fn run_day_one() {
    let mut day_one: DayOne = DayOne { calories: vec![0] };
    day_one.init();
    day_one.part_one();
    day_one.part_two();
}
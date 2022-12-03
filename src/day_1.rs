use std::fs;

static FILE_PATH: &str = "input/day_1.txt";

pub struct DayOne {
    pub calories: Vec<i64>,
}

impl DayOne {
    pub fn init(&mut self) {
        self.calories = DayOne::get_calories(fs::read_to_string(FILE_PATH).unwrap());
    }

    pub fn part_one(&mut self) {
        let highest_calorie_count = self.calories.iter().max();
        println!("Day 1.1: {:?}", highest_calorie_count.unwrap());
    }

    pub fn part_two(&mut self) {
        self.sort_calories();
        let total_top_three: i64 = self.calories[0..3].iter().sum();
        println!("Day 1.2: {:?}", total_top_three);
    }

    fn get_calories(input: String) -> Vec<i64> {
        input
            .split("\n\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|elf_calorie| {
                elf_calorie
                    .lines()
                    .map(|calorie| calorie.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
                    .iter()
                    .sum()
            })
            .collect::<Vec<i64>>()
    }

    fn sort_calories(&mut self) {
        self.calories.sort_by(|a, b| b.cmp(a))
    }
}

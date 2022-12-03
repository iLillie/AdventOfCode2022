use std::fs;

static FILE_PATH: &str = "input/day_1.txt";

pub struct CalorieCounting {
    pub elf_total_calories: Vec<i64>,
}

impl CalorieCounting {
    pub fn init(&mut self) {
        let input = fs::read_to_string(FILE_PATH).unwrap();
        self.elf_total_calories = CalorieCounting::get_total_elf_calories(input);
        self.sort_descending();
        self.print_stats();
    }

    pub fn get_highest_total(&mut self) {
        let highest_total = self.elf_total_calories[0];
        println!("     Highest total: {:?}", highest_total);
    }

    pub fn get_top_three_highest_total(&mut self) {
        let total_top_three: i64 = self.elf_total_calories[0..3].iter().sum();
        println!("     Top 3 highest total sum: {:?}", total_top_three);
    }

    fn get_total_elf_calories(input: String) -> Vec<i64> {
        CalorieCounting::get_elf_calories(input)
            .iter()
            .map(|elf_calories| CalorieCounting::to_total_calorie(elf_calories))
            .collect::<Vec<i64>>()
    }

    fn get_elf_calories(input: String) -> Vec<String> {
        input
        .split("\n\n")
        .map(|input| input.to_string())
        .collect::<Vec<String>>()
    }

    fn to_total_calorie(calories: &String) -> i64 {
        calories
        .lines()
        .map(|calorie| calorie.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .iter()
        .sum()
    }

    fn sort_descending(&mut self) {
        self.elf_total_calories.sort_by(|a, b| b.cmp(a))
    }

    fn print_stats(&mut self) {
        println!("Day 1:");
        println!("  Calorie Counting");
        println!(
                "     Total entries: {:?}",
        self.elf_total_calories.iter().count()
        );
        println!(
                "     Lowest total: {:?}",
        self.elf_total_calories.iter().min().unwrap()
        );
    }
}

impl Default for CalorieCounting {
    fn default() -> Self {
        Self {
            elf_total_calories: vec![0],
        }
    }
}

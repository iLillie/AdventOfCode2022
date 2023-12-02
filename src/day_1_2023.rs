use std::fs;

static FILE_PATH: &str = "input/day_1_2023.txt";

pub struct Trebuchet {
    pub numbers: Vec<Vec<char>>,
    pub numbers2: Vec<Vec<char>>,
}

impl Trebuchet {
    pub fn init(&mut self) {
        let input = fs::read_to_string(FILE_PATH).unwrap();
        self.numbers = input
            .lines()
            .map(|line| Trebuchet::get_numbers(line))
            .collect();
        self.numbers2 = input
            .lines()
            .map(|line| {
                let numberswon = Trebuchet::get_numbers_v2(line);
                println!("4: {:?}", numberswon);
                return  numberswon;
            } )
            .collect();
    }

    pub fn part_1(&mut self) {
        let total: Vec<i64> = self
            .numbers
            .iter()
            .map(|number| Trebuchet::get_calibration_value(number.clone()))
            .collect();
        let sum: i64 = total.iter().sum();
        println!("{}", sum)
    }


    pub fn part_2(&mut self) {
        let total: Vec<i64> = self
            .numbers2
            .iter()
            .map(|number| Trebuchet::get_calibration_value(number.clone()))
            .collect();
        let sum: i64 = total.iter().sum();
        println!("{}", sum)
    }

    pub fn get_numbers_v2(line: &str) -> Vec<char> {
        let digit_strs = [
            ("one", "o1e"),
            ("two", "t2o"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];

        let mut originalStr = String::from(line);
        println!("{}", originalStr);
        for digit_str in digit_strs {
            let value = format!("{}", digit_str.1);
            originalStr = originalStr.replace(digit_str.0.clone(), value.as_str());
        }
        println!("{}", originalStr);
        let first_digit = originalStr.clone().find(char::is_numeric).unwrap();
        let last_digit = originalStr.clone().rfind(char::is_numeric).unwrap();
        let characters: Vec<char> = originalStr.clone().chars().collect();

        return vec![characters[first_digit], characters[last_digit]];
    }

    pub fn get_numbers(line: &str) -> Vec<char> {
        let first_digit = line.find(char::is_numeric).unwrap();
        let last_digit = line.rfind(char::is_numeric).unwrap();
        let characters: Vec<char> = line.chars().collect();

        return vec![characters[first_digit], characters[last_digit]];
    }

    pub fn get_calibration_value(numbers: Vec<char>) -> i64 {
        let mut output = String::new();
        //println!("{:?}", numbers);
        let new_numbers: Vec<char> = numbers.into_iter().filter(|num| num.is_numeric()).collect();

        if new_numbers.len() == 1 {
            output.push(new_numbers[0]);
            output.push(new_numbers[0]);
            return output.parse::<i64>().unwrap();
        }
        //println!("{:?}", new_numbers);
        output.push(*new_numbers.first().unwrap());
        output.push(*new_numbers.last().unwrap());
        return output.parse::<i64>().unwrap();
    }
}

impl Default for Trebuchet {
    fn default() -> Self {
        Self {
            numbers: vec![],
            numbers2: vec![],
        }
    }
}

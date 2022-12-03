use std::fs;

static FILE_PATH: &str = "input/day_2.txt";

#[derive(PartialEq, Eq, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
    Unknown,
}

impl Hand {
    fn get_result_score(&self, opponent: &Hand) -> i64 {
        if self.is_draw(opponent) {
            return 3;
        }
        if self.is_win(opponent) {
            return 6;
        }
        if self.is_lost(opponent) {
            return 0;
        }
        0
    }

    fn is_draw(&self, opponent: &Hand) -> bool {
        return self == opponent;
    }

    fn is_lost(&self, opponent: &Hand) -> bool {
        !self.is_win(opponent) && !self.is_draw(opponent)
    }

    fn is_win(&self, opponent: &Hand) -> bool {
        let rock_win = self == &Hand::Rock && opponent == &Hand::Scissors;
        let paper_win = self == &Hand::Paper && opponent == &Hand::Rock;
        let scissors_win = self == &Hand::Scissors && opponent == &Hand::Paper;
        rock_win || paper_win || scissors_win
    }

    fn get_lose_shape(self, opponent: &Hand) -> Hand {
        match opponent {
            Hand::Rock => return Hand::Scissors,
            Hand::Paper => return Hand::Rock,
            Hand::Scissors => return Hand::Paper,
            Hand::Unknown => return Hand::Unknown,
        }
    }

    fn get_win_shape(self, opponent: &Hand) -> Hand {
        match opponent {
            Hand::Rock => return Hand::Paper,
            Hand::Paper => return Hand::Scissors,
            Hand::Scissors => return Hand::Rock,
            Hand::Unknown => return Hand::Unknown,
        }
    }

    fn get_draw_shape(self, opponent: &Hand) -> Hand {
        opponent.clone()
    }

    fn recalculate_shape(self, opponent: &Hand) -> Hand {
        match self {
            Hand::Rock => return self.get_lose_shape(opponent),
            Hand::Paper => return self.get_draw_shape(opponent),
            Hand::Scissors => return self.get_win_shape(opponent),
            Hand::Unknown => return Hand::Unknown,
        }
    }

    fn get_score(self) -> i64 {
        match self {
            Self::Rock => return 1,
            Self::Paper => return 2,
            Self::Scissors => return 3,
            Self::Unknown => return 0,
        }
    }

    fn from_string(input: &str) -> Hand {
        match input {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => Hand::Unknown,
        }
    }
}

#[derive(Clone)]
pub struct RockPaperScissors {
    pub player_moves: Vec<Hand>,
    pub opponent_moves: Vec<Hand>,
}

impl RockPaperScissors {
    pub fn init(&mut self) {
        let input = fs::read_to_string(FILE_PATH).unwrap();
        self.moves_from_str(input);
        self.print_stats()
    }

    fn print_stats(&mut self) {
        println!("Day 2:");
        println!("  Rock Paper Scissors");
        println!("    Moves: {:?}", self.player_moves.clone().iter().count());
    }

    fn moves_from_str(&mut self, input: String) {
        self.player_moves = input
            .lines()
            .map(|line| Hand::from_string(line.split(" ").collect::<Vec<&str>>()[1]))
            .collect::<Vec<Hand>>();
        self.opponent_moves = input
            .lines()
            .map(|line| Hand::from_string(line.split(" ").collect::<Vec<&str>>()[0]))
            .collect::<Vec<Hand>>();
    }

    pub fn recalculate_player_moves(&mut self) {
        self.player_moves = self
            .player_moves
            .clone()
            .into_iter()
            .zip(self.opponent_moves.clone())
            .map(|(player, opponent)| player.recalculate_shape(&opponent))
            .collect::<Vec<Hand>>()
    }

    pub fn get_player_score(self) -> i64 {
        let result_score = self.clone().get_result_score();
        let total_shape_score = self.get_total_shape_score();
        let total = result_score + total_shape_score;
        total
    }

    fn get_total_shape_score(self) -> i64 {
        self.player_moves
            .into_iter()
            .map(|hand| hand.get_score())
            .collect::<Vec<i64>>()
            .iter()
            .sum()
    }

    fn get_result_score(self) -> i64 {
        self.player_moves
            .into_iter()
            .zip(self.opponent_moves)
            .map(|(player, opponent)| player.get_result_score(&opponent))
            .collect::<Vec<i64>>()
            .iter()
            .sum()
    }
}

impl Default for RockPaperScissors {
    fn default() -> Self {
        Self {
            player_moves: vec![],
            opponent_moves: vec![],
        }
    }
}

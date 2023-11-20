use std::fs;

static FILE_PATH: &str = "input/day_3.txt";
static UPPERCASE_ASCII_OFFSET: u32 = 38;
static LOWERCASE_ASCII_OFFSET: u32 = 96;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub letter: char,
    pub priority: u32,
}

#[derive(Clone)]
pub struct Compartment {
    pub items: Vec<Item>,
}

#[derive(Clone)]
pub struct Rucksack {
    pub compartments: (Compartment, Compartment),
    pub duplicate: Item,
}

pub struct RucksackReorganization {
    pub rucksack_list: Vec<Rucksack>,
}

impl RucksackReorganization {
    pub fn init(&mut self) {
        let input = fs::read_to_string(FILE_PATH).unwrap();
        self.rucksack_list = input.lines().map(|line| Self::to_rucksack(line)).collect();
    }

    fn get_items_from_rucksack_group(
        rucksack_group: Vec<(Rucksack, Rucksack, Rucksack)>,
    ) -> Vec<Item> {
        return rucksack_group
            .into_iter()
            .map(|group| Self::get_duplicate_badge_item(group).unwrap())
            .collect::<Vec<Item>>();
    }

    pub fn solve_part2(rucksack_list: Vec<Rucksack>) -> u32 {
        let rucksack_groups = Self::split_rucksack_group(rucksack_list);

        let items: Vec<Item> = Self::get_items_from_rucksack_group(rucksack_groups);

        let item_priorities: Vec<u32> = items.into_iter().map(|item| item.priority).collect();
        return item_priorities.iter().sum();
    }

    fn split_rucksack_group(rucksack_list: Vec<Rucksack>) -> Vec<(Rucksack, Rucksack, Rucksack)> {
        let chunks: Vec<Vec<Rucksack>> = rucksack_list.chunks(3).map(|s| s.into()).collect();
        return chunks
            .into_iter()
            .map(|vec| (vec[0].clone(), vec[1].clone(), vec[2].clone()))
            .collect();
    }

    fn combine_compartment_items(rucksack: Rucksack) -> Vec<Item> {
        let mut items_1 = rucksack.compartments.0.items.clone();
        let mut items_2 = rucksack.compartments.1.items.clone();

        items_1.append(&mut items_2);
        return items_1;
    }

    fn get_duplicate_badge_item(rucksack_list: (Rucksack, Rucksack, Rucksack)) -> Option<Item> {
        let first_items = Self::combine_compartment_items(rucksack_list.0);
        let second_items = Self::combine_compartment_items(rucksack_list.1);
        let third_items = Self::combine_compartment_items(rucksack_list.2);

        let item = first_items
            .into_iter()
            .find(|item| second_items.contains(item) && third_items.contains(item));
        return item;
    }

    pub fn get_duplicate_priorty_sum(rucksacks: Vec<Rucksack>) -> u32 {
        let priorities = rucksacks
            .iter()
            .map(|rucksack| rucksack.duplicate.priority)
            .collect::<Vec<u32>>();
        return priorities.iter().sum();
    }

    fn sort_items(items_to_sort: Vec<Item>) -> Vec<Item> {
        let mut items = items_to_sort;
        items.sort_by_key(|letter| letter.priority);
        return items;
    }

    fn find_duplicate_item(compartments: (Compartment, Compartment)) -> Option<Item> {
        let first_items = compartments.0.items;
        let second_items = compartments.1.items;

        return first_items
            .into_iter()
            .find(|item| second_items.contains(item));
    }

    fn to_rucksack(input_str: &str) -> Rucksack {
        let compartments = Self::to_compartments(input_str);
        return Rucksack {
            compartments: compartments.clone(),
            duplicate: Self::find_duplicate_item(compartments).unwrap(),
        };
    }

    fn to_priority(letter: char) -> u32 {
        let letter_ascii_value = letter as u32;

        if letter.is_uppercase() {
            return letter_ascii_value - UPPERCASE_ASCII_OFFSET;
        }

        return letter_ascii_value - LOWERCASE_ASCII_OFFSET;
    }

    fn to_compartments(compartments: &str) -> (Compartment, Compartment) {
        let split_compartments = compartments.split_at(compartments.len() / 2);
        return (
            Self::to_compartment(split_compartments.0),
            Self::to_compartment(split_compartments.1),
        );
    }

    fn to_items(string: &str) -> Vec<Item> {
        return string
            .chars()
            .map(|c| Item {
                letter: c,
                priority: Self::to_priority(c),
            })
            .collect();
    }

    fn to_compartment(compartment_string: &str) -> Compartment {
        let items = Self::to_items(compartment_string);
        let sorted_items = Self::sort_items(items);
        return Compartment {
            items: sorted_items,
        };
    }
}

impl Default for RucksackReorganization {
    fn default() -> Self {
        Self {
            rucksack_list: vec![],
        }
    }
}

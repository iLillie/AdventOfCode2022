use std::fs;

use crate::day_2::RockPaperScissors;

// Each rucksack has two large compartments.
// All items of a given type are meant to go into exactly one of the two compartments.
// The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

// The Elves have made a list of all of the items currently in each rucksack (your puzzle input),
// but they need your help finding the errors.
// Every item type is identified by a single lowercase or uppercase letter
// (that is, a and A refer to different types of items).

// The list of items for each rucksack is given as characters all on a single line.
// A given rucksack always has the same number of items in each of its two compartments,
// so the first half of the characters represent items in the first compartment
// while the second half of the characters represent items in the second compartment.

// For example, suppose you have the following list of contents from six rucksacks:
/*
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
*/

// The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp
// which means its first compartment contains the items vJrwpWtwJgWr
// while the second compartment contains the items hcsFMMfFFhFp.
// The only item type that appears in both compartments is lowercase p

// To help prioritize item rearrangement, every item type can be converted to a priority
// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
// In the above example, the priority of the item type that appears in both compartments of each rucksack
// is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s);
// the sum of these is 157.

// lowerCaseOffset = 97
// upperCaseOffset = 64

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
    pub cumpartment_tuple: (Compartment, Compartment),
    pub duplicate_item: Item,
}

pub struct RucksackReorganization {
    pub rucksack_list: Vec<Rucksack>,
}

impl RucksackReorganization {
    pub fn init(&mut self) {
        let input = fs::read_to_string(FILE_PATH).unwrap();
        self.rucksack_list = input.lines().map(|line| Self::to_rucksack(line)).collect();
    }

    pub fn solve_part2(rucksack_list: Vec<Rucksack>) -> u32 {
        let get_rucksack_groups = Self::split_rucksack_group(rucksack_list);
        
        let items = get_rucksack_groups
            .into_iter()
            .map(|group| Self::get_duplicate_badge_item(group).unwrap())
            .collect::<Vec<Item>>();

        let item_priorities: Vec<u32> = items.into_iter().map(|item| item.priority).collect();
        return item_priorities.iter().sum();
    }

    fn split_rucksack_group(rucksack_list: Vec<Rucksack>) -> Vec<(Rucksack, Rucksack, Rucksack)> {
        let chunks: Vec<Vec<Rucksack>> = rucksack_list.chunks(3).map(|s| s.into()).collect();
        let triplets = chunks
            .into_iter()
            .map(|vec| (vec[0].clone(), vec[1].clone(), vec[2].clone()))
            .collect();
        return triplets;
    }

    fn combine_compartment_items(rucksack: Rucksack) -> Vec<Item> {
        let mut new_items = rucksack.cumpartment_tuple.0.items.clone();
        let mut other_items = rucksack.cumpartment_tuple.1.items.clone();
        new_items.append(&mut other_items);
        return new_items;
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
            .map(|rucksack| rucksack.duplicate_item.priority)
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
            cumpartment_tuple: compartments.clone(),
            duplicate_item: Self::find_duplicate_item(compartments).unwrap(),
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

    fn to_compartment(compartment_string: &str) -> Compartment {
        let items = compartment_string
            .chars()
            .map(|c| Item {
                letter: c,
                priority: Self::to_priority(c),
            })
            .collect();
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

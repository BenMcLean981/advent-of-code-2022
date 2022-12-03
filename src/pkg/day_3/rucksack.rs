use std::{fmt, str::FromStr};

use super::item::Item;

#[derive(Clone)]
pub(crate) struct Rucksack {
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
}

#[derive(Debug)]
pub(crate) struct RucksackParseError;

impl fmt::Display for RucksackParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read rucksack.");
    }
}

impl FromStr for Rucksack {
    type Err = RucksackParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len: usize = s.len();
        let half_len: usize = len / 2;

        let first_half: &str = &s[0..half_len];
        let second_half: &str = &s[half_len..len];

        let first_half: Vec<char> = first_half.chars().collect();
        let second_half: Vec<char> = second_half.chars().collect();

        let first_compartment: Vec<Item> = first_half.iter().map(|c| Item::from(*c)).collect();
        let second_compartment: Vec<Item> = second_half.iter().map(|c| Item::from(*c)).collect();

        return Ok(Rucksack {
            first_compartment,
            second_compartment,
        });
    }
}

impl Rucksack {
    pub fn get_priority_of_common_item(&self) -> u32 {
        let common_item = self.get_common_item().expect("No common items.");

        return common_item.get_priority();
    }

    fn get_common_item(&self) -> Option<&Item> {
        return self
            .first_compartment
            .iter()
            .find(|i1| self.second_compartment.contains(i1));
    }

    pub fn get_all_items(&self) -> Vec<&Item> {
        let mut result: Vec<&Item> = vec![];

        result.extend(&self.first_compartment);
        result.extend(&self.second_compartment);

        return result;
    }
}

use super::{item::Item, rucksack::Rucksack};

pub(crate) struct ElfGroup(Rucksack, Rucksack, Rucksack);

impl ElfGroup {
    pub fn new(r1: Rucksack, r2: Rucksack, r3: Rucksack) -> Self {
        return Self(r1, r2, r3);
    }

    pub fn get_badge_pririty(&self) -> u32 {
        return self.get_badge().get_priority();
    }

    fn get_badge(&self) -> &Item {
        let common_between_first_and_second: Vec<&Item> =
            ElfGroup::get_common_items(self.0.get_all_items(), self.1.get_all_items());
        let common_between_all_three: Vec<&Item> =
            ElfGroup::get_common_items(common_between_first_and_second, self.2.get_all_items());

        return common_between_all_three[0];
    }

    fn get_common_items<'a>(v1: Vec<&'a Item>, v2: Vec<&'a Item>) -> Vec<&'a Item> {
        return v1.iter().filter(|i| v2.contains(i)).map(|i| *i).collect();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HouseWithPresents {
    pub house: u32,
    pub amount: u64,
}

pub struct PresentsIter {
    next_house: u32,
}

impl Default for PresentsIter {
    fn default() -> Self {
        Self { next_house: 1 }
    }
}

impl Iterator for PresentsIter {
    type Item = HouseWithPresents;

    fn next(&mut self) -> Option<Self::Item> {
        let house = self.next_house;
        self.next_house += 1;
        let amount = Some(presents_for_house(house));
        amount.map(|amount| HouseWithPresents { house, amount })
    }
}

fn presents_for_house(house: u32) -> u64 {
    let elves = 1..=house;
    elves.map(|elf| presents_for_house_by_elf(house, elf)).sum()
}

fn presents_for_house_by_elf(house: u32, elf: u32) -> u64 {
    if house % elf == 0 {
        elf as u64 * 10
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presents_works() {
        let mut presents = PresentsIter::default();

        let want_presents = [10, 30, 40, 70, 60, 120, 80, 150, 130];

        for (&amount, house) in want_presents.iter().zip(1..) {
            assert_eq!(
                presents.next().unwrap(),
                HouseWithPresents { house, amount }
            );
        }
    }
}

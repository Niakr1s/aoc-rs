use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct HouseWithPresents {
    pub house: u32,
    pub amount: u64,
}

pub struct PresentsIter<D> {
    next_house: u32,
    deliver: D,
}

impl<D> PresentsIter<D>
where
    D: Default,
{
    pub fn new() -> Self {
        Self {
            next_house: 1,
            deliver: D::default(),
        }
    }
}

impl<D> Iterator for PresentsIter<D>
where
    D: Delieve,
{
    type Item = HouseWithPresents;

    fn next(&mut self) -> Option<Self::Item> {
        let house = self.next_house;
        self.next_house += 1;
        let amount = self.deliver.deliver(house);
        Some(HouseWithPresents { house, amount })
    }
}

pub trait Delieve {
    fn deliver(&mut self, house: u32) -> u64;
}

#[derive(Default)]
pub struct Deliver1;

impl Delieve for Deliver1 {
    fn deliver(&mut self, house: u32) -> u64 {
        let elves = find_divisors(house);
        elves.into_iter().map(|elf| elf as u64 * 10).sum()
    }
}

#[derive(Default)]
pub struct Deliver2 {
    elf_visited: HashMap<u32, u32>,
}

impl Deliver2 {
    pub fn new() -> Self {
        Self {
            elf_visited: HashMap::new(),
        }
    }
}

impl Delieve for Deliver2 {
    fn deliver(&mut self, house: u32) -> u64 {
        let elves = find_divisors(house);
        elves
            .into_iter()
            .filter_map(|elf| {
                let entry = self.elf_visited.entry(elf).or_insert(0);
                if *entry >= 50 {
                    return None;
                }
                if house % elf == 0 {
                    *entry += 1;
                    Some(elf as u64 * 11)
                } else {
                    None
                }
            })
            .sum()
    }
}

fn find_divisors(n: u32) -> Vec<u32> {
    let mut divisors = Vec::new();

    let sqrt_n = (n as f64).sqrt() as u32;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);

            if n / i != i {
                divisors.push(n / i);
            }
        }
    }

    divisors.sort();

    divisors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presents_deliver1_works() {
        let mut presents = PresentsIter::<Deliver1>::new();

        let want_presents = [10, 30, 40, 70, 60, 120, 80, 150, 130];

        for (&amount, house) in want_presents.iter().zip(1..) {
            assert_eq!(
                presents.next().unwrap(),
                HouseWithPresents { house, amount }
            );
        }
    }

    #[test]
    fn presents_deliver2_works() {
        let mut presents = PresentsIter::<Deliver2>::new();

        let want_presents = [11, 33, 44, 77, 66, 132, 88, 165, 143];

        for (&amount, house) in want_presents.iter().zip(1..) {
            assert_eq!(
                presents.next().unwrap(),
                HouseWithPresents { house, amount }
            );
        }

        let house51 = presents
            .by_ref()
            .skip_while(|house| house.house < 51)
            .next()
            .unwrap();

        assert_eq!(house51.house, 51);
        assert_eq!(house51.amount, (3 + 17 + 51) * 11);

        let house52 = presents.by_ref().next().unwrap();
        assert_eq!(house52.house, 52);
        assert_eq!(house52.amount, (2 + 4 + 13 + 26 + 52) * 11);
    }

    #[test]
    fn find_divisors_works() {
        assert_eq!(find_divisors(1), [1]);
        assert_eq!(find_divisors(51), [1, 3, 17, 51]);
        assert_eq!(find_divisors(52), [1, 2, 4, 13, 26, 52]);
    }
}

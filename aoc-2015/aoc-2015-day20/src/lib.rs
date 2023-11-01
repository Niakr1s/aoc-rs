#[derive(Debug, PartialEq, Eq)]
pub struct HouseWithPresents {
    pub house: u32,
    pub amount: u64,
}

pub struct PresentsIter {
    next_house: u32,
    deliver: fn(u32) -> u64,
}

impl PresentsIter {
    pub fn new_deliver1() -> Self {
        Self {
            next_house: 1,
            deliver: deliver1,
        }
    }

    pub fn new_deliver2() -> Self {
        Self {
            next_house: 1,
            deliver: deliver2,
        }
    }
}

impl Iterator for PresentsIter {
    type Item = HouseWithPresents;

    fn next(&mut self) -> Option<Self::Item> {
        let house = self.next_house;
        self.next_house += 1;
        let amount = (self.deliver)(house);
        Some(HouseWithPresents { house, amount })
    }
}

fn deliver1(house: u32) -> u64 {
    let elves = find_divisors(house);
    elves.into_iter().map(|elf| elf as u64 * 10).sum()
}

fn deliver2(house: u32) -> u64 {
    // we won't use find_divisors function, because it's not more than 50 elves
    let start_elf = house.checked_sub(50).unwrap_or(0) + 1;
    let elves = start_elf..=house;
    elves
        .map(|elf| if house % elf == 0 { elf as u64 * 11 } else { 0 })
        .sum()
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
        let mut presents = PresentsIter::new_deliver1();

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
        let mut presents = PresentsIter::new_deliver2();

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
        assert_eq!(house52.amount, (4 + 13 + 26 + 52) * 11);
    }

    #[test]
    fn find_divisors_works() {
        assert_eq!(find_divisors(1), [1]);
        assert_eq!(find_divisors(51), [1, 3, 17, 51]);
        assert_eq!(find_divisors(52), [1, 2, 4, 13, 26, 52]);
    }
}

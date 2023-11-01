use aoc_2015_day20::{Deliver1, Deliver2, PresentsIter};

const USAGE: &str = "Usage: aoc-2015-day20 part1|part2";

fn main() {
    let part = std::env::args().skip(1).next().expect(USAGE);
    if part.ends_with("1") {
        part1();
    } else if part.ends_with("2") {
        part2();
    } else {
        panic!("{USAGE}");
    }
}

fn part1() {
    println!("Running part 1...");
    let house = PresentsIter::new(Deliver1)
        .skip_while(|house| house.amount < 36000000)
        .next()
        .unwrap();
    println!("Part 1: {house:?}");
}

fn part2() {
    println!("Running part 2...");
    let house = PresentsIter::new(Deliver2::new())
        .skip_while(|house| house.amount < 36000000)
        .next()
        .unwrap();
    println!("Part 2: {house:?}");
}

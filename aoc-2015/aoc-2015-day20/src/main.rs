use aoc_2015_day20::{Deliver1, Deliver2, PresentsIter};

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("Running part 1...");
    let house = PresentsIter::<Deliver1>::new()
        .skip_while(|house| house.amount < 36000000)
        .next()
        .unwrap();
    println!("Part 1: {house:?}");
}

fn part2() {
    println!("Running part 2...");
    let house = PresentsIter::<Deliver2>::new()
        .skip_while(|house| house.amount < 36000000)
        .next()
        .unwrap();
    println!("Part 2: {house:?}");
}

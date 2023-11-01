use aoc_2015_day20::PresentsIter;

fn main() {
    let presents = PresentsIter::default();

    let house = presents
        // .skip(1)
        // .step_by(2)
        // .take(20)
        .skip_while(|house| house.amount < 36000000)
        // .for_each(|house| println!("{house:?}"));
        .next()
        .unwrap();

    println!("Part 1: {house:?}");
}

use aoc_2015_day10::look_and_say::look_and_say_n_times;

fn main() {
    run_part1();
}

fn run_part1() {
    let res = look_and_say_n_times("1113222113", 40);
    println!("Part 1: {}", res.len());
}

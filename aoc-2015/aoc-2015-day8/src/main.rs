use aoc_2015_day8::line::Line;
use helpers::get_filepath_from_args;

fn main() {
    let filepath = get_filepath_from_args();
    let input = std::fs::read_to_string(filepath).unwrap();

    let lines = input
        .lines()
        .map(|line| Line::new(line.to_owned()).unwrap())
        .collect::<Vec<_>>();

    println!("Part 1:");
    run_part1(&lines);
    println!("Part 2:");
    run_part2(lines);
}

fn count<'a>(lines: impl Iterator<Item = &'a Line>) {
    let (mut code_len, mut memory_len) = (0, 0);
    for line in lines {
        code_len += line.len_in_code();
        memory_len += line.len_in_memory();
    }
    println!("code len: {}", code_len);
    println!("memory len: {}", memory_len);
    println!("difference: {}", code_len - memory_len);
}

fn run_part1(lines: &Vec<Line>) {
    count(lines.iter());
}

fn run_part2(lines: Vec<Line>) {
    let lines = lines
        .into_iter()
        .map(|line| line.encode())
        .collect::<Vec<_>>();

    count(lines.iter());
}

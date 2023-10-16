use aoc_2015_day8::line::Line;
use helpers::get_filepath_from_args;

fn main() {
    let filepath = get_filepath_from_args();
    let input = std::fs::read_to_string(filepath).unwrap();

    let (mut code_len, mut memory_len) = (0, 0);
    for line in input.lines() {
        let line = Line::new(line.to_owned()).unwrap();
        code_len += line.len_in_code();
        memory_len += line.len_in_memory();
    }
    println!("code len: {}", code_len);
    println!("memory len: {}", memory_len);
    println!("difference: {}", code_len - memory_len);
}

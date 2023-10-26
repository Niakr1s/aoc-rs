use std::str::FromStr;

use aoc_2015_day18::light::Grid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let contents = std::fs::read_to_string(filepath)?;

    let mut grid = Grid::from_str(&contents)?;
    for _ in 0..100 {
        grid = grid.next_step();
    }
    println!("Part1: {} lights are on after 100 steps", grid.count_on());

    Ok(())
}

use std::str::FromStr;

use aoc_2015_day18::light::Grid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let contents = std::fs::read_to_string(filepath)?;

    let grid = Grid::from_str(&contents)?;

    let part1_res = lights_on_after_100_steps(grid.clone());
    println!("Part1: {} lights are on after 100 steps", part1_res);

    let grid = grid.with_stucked_corners();
    let part2_res = lights_on_after_100_steps(grid);
    println!("Part2: {} lights are on after 100 steps", part2_res);

    Ok(())
}

fn lights_on_after_100_steps(mut grid: Grid) -> usize {
    for _ in 0..100 {
        grid = grid.next_step();
    }
    grid.count_on()
}

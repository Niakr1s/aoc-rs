use aoc_2015_day19::replacements::Replacements;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let input = std::fs::read_to_string(filepath)?;
    let input = input.trim();
    let (replacements, molecula) = input
        .trim()
        .split_at(input.rfind('\n').expect("No newline before formula"));
    let replacements = replacements.parse::<Replacements>()?;
    let distinct_moleculas_count = replacements.distinct_moleculas(molecula).unique().count();
    println!("Part1: {}", distinct_moleculas_count);

    Ok(())
}

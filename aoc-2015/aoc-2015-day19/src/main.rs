use aoc_2015_day19::replacements::{min_downgrade_steps, Replacements};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    // let filepath = std::path::PathBuf::from("aoc-2015/input/aoc-2015-day19.txt");
    let input = std::fs::read_to_string(filepath)?;
    let input = input.trim();
    let (replacements, molecula) = input
        .trim()
        .split_at(input.rfind('\n').expect("No newline before formula"));
    let molecula = molecula.trim();
    let replacements = replacements.parse::<Replacements>()?;
    let upgraded_moleculas_count = replacements.upgraded_moleculas(molecula).count();
    println!("Part1: {}", upgraded_moleculas_count);

    let downgrade_steps_count = min_downgrade_steps(molecula, &replacements);
    println!("Part2: {}", downgrade_steps_count);

    Ok(())
}

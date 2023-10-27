use aoc_2015_day19::replacements::{downgrade_steps, Replacements};

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
    let distinct_moleculas_count = replacements.distinct_moleculas(molecula).count();
    println!("Part1: {}", distinct_moleculas_count);

    let steps_count = downgrade_steps(molecula, &replacements);
    println!("Part2: {}", steps_count);

    Ok(())
}

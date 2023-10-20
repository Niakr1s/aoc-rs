use aoc_2015_day12::summarize::Summarize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath).unwrap();

    let json: serde_json::Value = serde_json::from_reader(file)?;

    let c = <serde_json::Value as Summarize>::count(&json)?;
    println!("Part 1: {}", c);

    Ok(())
}

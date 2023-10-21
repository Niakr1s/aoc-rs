use aoc_2015_day12::summarize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath).unwrap();

    let json: serde_json::Value = serde_json::from_reader(file)?;

    let part1_res = summarize::count(&json)?;
    println!("Part 1: {}", part1_res);

    let should_count = |v: &serde_json::Value| {
        let mut should_skip = false;
        if let serde_json::Value::Object(o) = v {
            should_skip = o.values().any(|v| {
                if let serde_json::Value::String(s) = v {
                    s == "red"
                } else {
                    false
                }
            });
        }
        !should_skip
    };

    let part2_res = summarize::count_if(&json, should_count)?;
    println!("Part 2: {}", part2_res);

    Ok(())
}

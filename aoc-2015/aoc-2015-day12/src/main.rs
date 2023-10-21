use aoc_2015_day12::summarize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath).unwrap();

    let json: serde_json::Value = serde_json::from_reader(file)?;

    let part1_res = summarize::sum(&json)?;
    println!("Part 1: {}", part1_res);

    let is_not_red_obj = |v: &serde_json::Value| {
        let mut is_red_obj = false;
        if let serde_json::Value::Object(o) = v {
            is_red_obj = o.values().any(|v| {
                if let serde_json::Value::String(s) = v {
                    s == "red"
                } else {
                    false
                }
            });
        }
        !is_red_obj
    };

    let part2_res = summarize::sum_if(&json, is_not_red_obj)?;
    println!("Part 2: {}", part2_res);

    Ok(())
}

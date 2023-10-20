use aoc_2015_day12::summarize::Summarize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath).unwrap();

    let json: serde_json::Value = serde_json::from_reader(file)?;

    // let part1_res = <serde_json::Value as Summarize>::count(&json)?;
    // println!("Part 1: {}", part1_res);

    let mut called = 0;
    let part2_res = json.count_if(move |v| {
        // let mut should_skip = false;
        // if let serde_json::Value::Object(o) = v {
        //     should_skip = o.values().any(|v| {
        //         if let serde_json::Value::String(s) = v {
        //             s == "red"
        //         } else {
        //             false
        //         }
        //     });
        // }
        // called += 1;
        // println!("Should skip = {should_skip}");
        // !should_skip
        false
    })?;
    println!("Part 2: {}", part2_res);
    println!("called {called} times");

    Ok(())
}

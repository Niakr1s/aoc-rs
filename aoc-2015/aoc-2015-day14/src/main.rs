use std::io::BufRead;

use aoc_2015_day14::{
    race::{judge::LeadingReindeerJudge, JudgedRace, NormalRace, Race},
    reindeer::Reindeer,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = helpers::get_filepath_from_args();
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);
    let reindeers = reader
        .lines()
        .flat_map(|line| line.map(|line| Reindeer::from_aoc_line(&line)))
        .collect::<Result<Vec<_>, _>>()?;

    let normal_race = NormalRace::new(&reindeers);
    println!("Part1:");
    run(normal_race)?;

    let judged_race = JudgedRace::new(&reindeers, LeadingReindeerJudge::new());
    println!("Part2:");
    run(judged_race)?;

    Ok(())
}

fn run(race: impl Race) -> Result<(), Box<dyn std::error::Error>> {
    let race = race.after(2503);
    let (winner, score) = race
        .scores()
        .into_iter()
        .enumerate()
        .max_by_key(|(_, d)| *d)
        .ok_or("No winner")?;
    println!(
        "The winner is {} with a score of {}",
        race.reindeers()[winner].name,
        score
    );
    Ok(())
}

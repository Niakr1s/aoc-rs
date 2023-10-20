use aoc_2015_day11::password::{security::SecurityElf, Password};

fn main() {
    run("hxbxwxba");
}

fn run(pass: &str) {
    let mut iter = Password::new(pass.to_string())
        .unwrap()
        .next_password_iter()
        .filter(|p| SecurityElf::is_valid(p.as_str()));

    println!("{}", iter.next().unwrap().as_str());
}

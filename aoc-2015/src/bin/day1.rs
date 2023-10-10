fn main() -> Result<()> {
    let args: std::env::Args = std::env::args();
    let directions = args.skip(1).next();
    if directions == None {
        println!("Usage:\n\tprogramm <input>\n");
        std::process::exit(1);
    }
    let directions = directions.unwrap();

    println!("Floor is: {}", count_floor(&directions)?);
    let entrance = entrance_pos(&directions, -1)?;
    match entrance {
        Some(entrance) => println!("Entrance is: {}", entrance),
        None => println!("Entrance not found"),
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInput,
}

pub type Result<T> = std::result::Result<T, Error>;

fn count_floor(input: &str) -> Result<i32> {
    let mut floor = 0;
    for ch in input.chars() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        } else {
            return Err(Error::InvalidInput);
        }
    }

    Ok(floor)
}

fn entrance_pos(input: &str, wanted_floor: i32) -> Result<Option<usize>> {
    let mut floor = 0;
    for (pos, ch) in input.chars().enumerate() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        } else {
            return Err(Error::InvalidInput);
        }
        if floor == wanted_floor {
            return Ok(Some(pos + 1));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod count_floor {
    use super::*;

    #[test]
    fn count_floor_1() {
        assert_eq!(count_floor("(())"), Ok(0));
    }

    #[test]
    fn count_floor_2() {
        assert_eq!(count_floor("()()"), Ok(0));
    }

    #[test]
    fn count_floor_3() {
        assert_eq!(count_floor("((("), Ok(3));
    }

    #[test]
    fn count_floor_4() {
        assert_eq!(count_floor("(()(()("), Ok(3));
    }

    #[test]
    fn count_floor_5() {
        assert_eq!(count_floor("))((((("), Ok(3));
    }

    #[test]
    fn count_floor_6() {
        assert_eq!(count_floor("())"), Ok(-1));
    }

    #[test]
    fn count_floor_7() {
        assert_eq!(count_floor("))("), Ok(-1));
    }

    #[test]
    fn count_floor_8() {
        assert_eq!(count_floor(")))"), Ok(-3));
    }

    #[test]
    fn count_floor_9() {
        assert_eq!(count_floor(")())())"), Ok(-3));
    }

    #[test]
    fn count_floor_invalid_1() {
        assert_eq!(count_floor("()1"), Err(Error::InvalidInput));
    }
}

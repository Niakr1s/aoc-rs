use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

fn main() -> Result<()> {
    let args = std::env::args();
    let filepath = args.skip(1).next();
    if filepath == None {
        println!("Usage:\n\tprogramm <input_filepath>\n");
        std::process::exit(1);
    }
    let filepath: PathBuf = filepath.unwrap().into();

    let moves: Moves = File::open(filepath)?.try_into()?;
    let pathway: Pathway = moves.into();

    println!("Visited houses: {}", pathway.len());

    Ok(())
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidInput,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

struct Moves(Vec<Move>);

impl TryFrom<&char> for Move {
    type Error = Error;

    fn try_from(value: &char) -> std::result::Result<Self, Self::Error> {
        match value {
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            '>' => Ok(Move::Right),
            _ => Err(Error::InvalidInput),
        }
    }
}

impl TryFrom<&str> for Moves {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let moves = value
            .chars()
            .map(|c| Move::try_from(&c))
            .collect::<Result<Vec<Move>>>()?;
        Ok(Moves(moves))
    }
}

impl TryFrom<File> for Moves {
    type Error = Error;

    fn try_from(mut file: File) -> std::result::Result<Self, Self::Error> {
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;
        Moves::try_from(contents.as_str())
    }
}

impl From<Vec<Move>> for Moves {
    fn from(value: Vec<Move>) -> Self {
        Self(value)
    }
}

impl AsRef<[Move]> for Moves {
    fn as_ref(&self) -> &[Move] {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add<&Move> for Point {
    type Output = Self;

    fn add(mut self, rhs: &Move) -> Self::Output {
        match rhs {
            Move::Up => self.y += 1,
            Move::Down => self.y -= 1,
            Move::Right => self.x += 1,
            Move::Left => self.x -= 1,
        }
        self
    }
}

struct Pathway(HashMap<Point, u32>);

impl<T: AsRef<[Move]>> From<T> for Pathway {
    fn from(moves: T) -> Self {
        let moves: &[Move] = moves.as_ref();
        let mut current = Point { x: 0, y: 0 };
        let mut pathway = HashMap::new();
        pathway.insert(current.clone(), 0);
        for mv in moves {
            current = current + mv;
            pathway
                .entry(current.clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        Pathway(pathway)
    }
}

impl Pathway {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

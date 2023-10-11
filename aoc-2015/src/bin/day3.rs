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

    let santa_report = DeliveryReport::new(
        "Santa".to_owned(),
        DeliveryMap::new().visit_by(SingleCarrier::new(Point { x: 0, y: 0 }, &moves)),
    );
    let santa_and_robot_report = DeliveryReport::new(
        "Santa and Robot".to_owned(),
        DeliveryMap::new().visit_by(TurnCarriers::new(Point { x: 0, y: 0 }, &moves, 2)),
    );

    for d in [&santa_report, &santa_and_robot_report] {
        d.print_visited_houses();
    }

    Ok(())
}

struct DeliveryReport {
    name: String,
    delivery_map: DeliveryMap,
}

impl DeliveryReport {
    fn new(name: String, delivery_map: DeliveryMap) -> Self {
        Self { name, delivery_map }
    }

    fn print_visited_houses(&self) {
        println!("{} visited houses: {}", self.name, self.delivery_map.len());
    }
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

#[derive(Debug, Clone, Copy)]
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

struct DeliveryMap(HashMap<Point, u32>);

impl DeliveryMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn visit_by<C: Carrier>(mut self, carrier: C) -> Self {
        carrier.visit(&mut self);
        self
    }

    pub fn visit_point(&mut self, point: &Point) {
        self.0
            .entry(point.clone())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

trait Carrier {
    fn visit(self, delivery_map: &mut DeliveryMap);
}

/// A single carrier, which visits every point in a path according to moves
struct SingleCarrier<'a> {
    start_point: Point,
    moves: &'a Moves,
}

impl<'a> SingleCarrier<'a> {
    fn new(start_point: Point, moves: &'a Moves) -> Self {
        Self { start_point, moves }
    }
}

impl<'a> Carrier for SingleCarrier<'a> {
    fn visit(self, delivery_map: &mut DeliveryMap) {
        let mut current = self.start_point;
        delivery_map.visit_point(&current);
        for mv in &self.moves.0 {
            current = current + mv;
            delivery_map.visit_point(&current);
        }
    }
}

/// Several carriers, which visits every point in a path according to moves,
/// which are taken in turn from common moves list.
struct TurnCarriers<'a> {
    start_point: Point,
    moves: &'a Moves,
    num_of_carriers: usize,
}

impl<'a> TurnCarriers<'a> {
    fn new(start_point: Point, moves: &'a Moves, num_of_carriers: usize) -> Self {
        Self {
            start_point,
            moves,
            num_of_carriers,
        }
    }
}

impl<'a> Carrier for TurnCarriers<'a> {
    fn visit(self, delivery_map: &mut DeliveryMap) {
        let mut do_visit = |offset: usize| {
            let santa_moves: Moves = self
                .moves
                .0
                .iter()
                .enumerate()
                .filter_map(|(i, &mv)| {
                    if i % self.num_of_carriers == offset {
                        Some(mv)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Move>>()
                .into();
            let santa = SingleCarrier::new(self.start_point.clone(), &santa_moves);
            santa.visit(delivery_map);
        };

        for offset in 0..self.num_of_carriers {
            do_visit(offset);
        }
    }
}

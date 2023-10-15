use std::{io::BufRead, path::PathBuf, str::FromStr};

use lib::{Command, Grid};

fn main() -> Result<(), error::Error> {
    let filepath: PathBuf = std::env::args()
        .skip(1)
        .next()
        .expect("Provide a file path as first argument")
        .into();
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut grid = Grid::new();
    for cmd in reader
        .lines()
        .flat_map(|line| line.map(|line| line.parse::<Command>()))
    {
        grid.apply_cmd(&cmd?)?;
    }
    Ok(())
}

mod error {
    use crate::lib;

    #[derive(Debug)]
    pub enum Error {
        Io(std::io::Error),
        Parse(lib::FromStrError),
        Grid(lib::GridError),
    }

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Self {
            Error::Io(e)
        }
    }

    impl From<lib::FromStrError> for Error {
        fn from(e: lib::FromStrError) -> Self {
            Error::Parse(e)
        }
    }

    impl From<lib::GridError> for Error {
        fn from(e: lib::GridError) -> Self {
            Error::Grid(e)
        }
    }
}

mod lib {
    use std::{num::ParseIntError, str::FromStr};

    const GRID_SZ: usize = 1000;

    #[derive(Debug)]
    pub enum FromStrError {
        ParseError,
    }

    impl From<ParseIntError> for FromStrError {
        fn from(_: ParseIntError) -> Self {
            FromStrError::ParseError
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Instruction {
        TurnOn,
        TurnOff,
        Toggle,
    }

    impl FromStr for Instruction {
        type Err = FromStrError;

        /// '.*on', '.*off', 'toggle'
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.ends_with("on") {
                Ok(Instruction::TurnOn)
            } else if s.ends_with("off") {
                Ok(Instruction::TurnOff)
            } else if s == "toggle" {
                Ok(Instruction::Toggle)
            } else {
                Err(FromStrError::ParseError)
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    impl Point {
        pub fn is_inside_grid(&self) -> bool {
            self.x < GRID_SZ && self.y < GRID_SZ
        }

        /// Returns true if `of` is on the top right of `this`.
        pub fn is_top_right_of(&self, other: &Point) -> bool {
            self.x > other.x && self.y > other.y
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Rectangle {
        pub start: Point,
        pub end: Point,
    }

    impl Rectangle {
        /// Returns true if the rectangle is inside the grid,
        /// not zero-sized and end is at top-rignt of start.
        pub fn is_valid(&self) -> bool {
            self.start.is_inside_grid()
                && self.end.is_inside_grid()
                && self.end.is_top_right_of(&self.start)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Command {
        pub instruction: Instruction,
        pub rect: Rectangle,
    }

    impl FromStr for Command {
        type Err = FromStrError;

        /// Example strings:
        /// turn on 171,630 through 656,769
        /// turn off 417,276 through 751,500
        /// toggle 559,485 through 584,534
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let split = s
                .split([' ', ','])
                .enumerate()
                .filter_map(|(i, s)| if i == 0 && s == "turn" { None } else { Some(s) })
                .collect::<Vec<_>>();
            if split.len() != 6 {
                return Err(FromStrError::ParseError);
            }

            let (instruction, sx, sy, ex, ey) = (
                split[0].parse::<Instruction>()?,
                split[1].parse::<usize>()?,
                split[2].parse::<usize>()?,
                split[4].parse::<usize>()?,
                split[5].parse::<usize>()?,
            );
            let rect = Rectangle {
                start: Point { x: sx, y: sy },
                end: Point { x: ex, y: ey },
            };

            Ok(Command { instruction, rect })
        }
    }

    #[derive(Debug)]
    pub enum GridError {
        InvalidInstruction(Instruction),
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GridCell(bool);

    pub struct Grid([[GridCell; GRID_SZ]; GRID_SZ]);

    impl Grid {
        pub fn new() -> Self {
            Self([[GridCell(false); GRID_SZ]; GRID_SZ])
        }
    }

    impl Grid {
        pub fn apply_cmd(&mut self, cmd: &Command) -> Result<(), GridError> {
            if !cmd.rect.is_valid() {
                return Err(GridError::InvalidInstruction(cmd.instruction));
            }
            match cmd.instruction {
                Instruction::TurnOn => self.turn_on(&cmd.rect),
                Instruction::TurnOff => self.turn_off(&cmd.rect),
                Instruction::Toggle => self.toggle(&cmd.rect),
            }
            Ok(())
        }

        /// Attention: valid rect should be provided.
        fn turn_on(&mut self, rect: &Rectangle) {
            for y in rect.start.y..=rect.end.y {
                for x in rect.start.x..=rect.end.x {
                    self.0[y][x] = GridCell(true);
                }
            }
        }

        /// Attention: valid rect should be provided.
        fn turn_off(&mut self, rect: &Rectangle) {
            for y in rect.start.y..=rect.end.y {
                for x in rect.start.x..=rect.end.x {
                    self.0[y][x] = GridCell(false);
                }
            }
        }

        /// Attention: valid rect should be provided.
        fn toggle(&mut self, rect: &Rectangle) {
            for y in rect.start.y..=rect.end.y {
                for x in rect.start.x..=rect.end.x {
                    let cell = self.0[y][x];
                    let toggled = GridCell(!cell.0);
                    self.0[y][x] = toggled;
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        mod instruction {
            mod from_str {
                use crate::lib::Instruction;

                macro_rules! test {
                    ($s:expr, $wanted:expr) => {
                        let instruction = $s.parse::<Instruction>();
                        assert_eq!(instruction.ok(), $wanted);
                    };
                }

                #[test]
                fn test_on() {
                    test!("on", Some(Instruction::TurnOn));
                    test!("turn_on", Some(Instruction::TurnOn));
                    test!("turn on", Some(Instruction::TurnOn));
                }

                #[test]
                fn test_off() {
                    test!("off", Some(Instruction::TurnOff));
                    test!("turn_off", Some(Instruction::TurnOff));
                    test!("turn off", Some(Instruction::TurnOff));
                }

                #[test]
                fn test_toggle() {
                    test!("toggle", Some(Instruction::Toggle));
                }

                #[test]
                fn test_invalid() {
                    test!("please toggle", None);
                    test!("on please", None);
                    test!("off please", None);
                }
            }
        }

        mod rectangle {
            mod is_valid {
                use crate::lib::{Point, Rectangle, GRID_SZ};

                const LAST: usize = GRID_SZ - 1;

                macro_rules! test {
                    (($sx:expr, $sy:expr), ($ex:expr, $ey:expr) => $wanted:expr) => {
                        assert_eq!(
                            Rectangle {
                                start: Point { x: $sx, y: $sy },
                                end: Point { x: $ex, y: $ey }
                            }
                            .is_valid(),
                            $wanted
                        );
                    };
                }

                #[test]
                fn test_eq_1() {
                    test!((0, 0), (1, 1) => true);
                }

                #[test]
                fn test_eq_2() {
                    test!((3, 3), (5, 5) => true);
                }

                #[test]
                fn test_eq_3() {
                    test!((LAST-1, LAST-1), (LAST, LAST) => true);
                }

                #[test]
                fn test_invalid_1() {
                    test!((0, 0), (0, 0) => false);
                }

                #[test]
                fn test_invalid_2() {
                    test!((5, 5), (5, 5) => false);
                }

                #[test]
                fn test_invalid_3() {
                    test!((5, 5), (4, 4) => false);
                }

                #[test]
                fn test_invalid_4() {
                    test!((5, 5), (6, 4) => false);
                }

                #[test]
                fn test_invalid_5() {
                    test!((5, 5), (4, 6) => false);
                }

                #[test]
                fn test_invalid_6() {
                    test!((LAST, LAST), (LAST, LAST) => false);
                }

                #[test]
                fn test_invalid_7() {
                    test!((GRID_SZ, GRID_SZ), (GRID_SZ+1, GRID_SZ+1) => false);
                }
            }
        }

        mod command {

            mod from_str {

                use crate::lib::{Command, Instruction, Point, Rectangle};

                #[test]
                fn test_on() {
                    assert_eq!(
                        "turn on 171,630 through 656,769".parse::<Command>().ok(),
                        Some(Command {
                            instruction: Instruction::TurnOn,
                            rect: Rectangle {
                                start: Point { x: 171, y: 630 },
                                end: Point { x: 656, y: 769 },
                            }
                        })
                    );
                }

                #[test]
                fn test_off() {
                    assert_eq!(
                        "turn off 417,276 through 751,500".parse::<Command>().ok(),
                        Some(Command {
                            instruction: Instruction::TurnOff,
                            rect: Rectangle {
                                start: Point { x: 417, y: 276 },
                                end: Point { x: 751, y: 500 },
                            }
                        })
                    );
                }

                #[test]
                fn test_toggle() {
                    assert_eq!(
                        "toggle 559,485 through 584,534".parse::<Command>().ok(),
                        Some(Command {
                            instruction: Instruction::Toggle,
                            rect: Rectangle {
                                start: Point { x: 559, y: 485 },
                                end: Point { x: 584, y: 534 },
                            }
                        })
                    );
                }

                #[test]
                fn test_invalid() {
                    const TEST_CASES: &[&str] = &[
                        "turn onn 171,630 through 656,769",
                        "turn on 171a,630 through 656,769",
                        "turn on 171,a630 through 656,769",
                        "turn on 171,630 through a656,769",
                        "turn on 171,630 through 656,a769",
                        "turn on 171,630 656,769",
                        "turn of 417,276 through 751,500",
                        "turn off a417,276 through 751,500",
                        "turn off 417,a276 through 751,500",
                        "turn off 417,276 through a751,500",
                        "turn off 417,276 through 751,a500",
                        "turn off 417,276 751,500",
                        "togle 559,485 through 584,534",
                        "toggle a559,485 through 584,534",
                        "toggle 559,a485 through 584,534",
                        "toggle 559,485 through a584,534",
                        "toggle 559,485 through 584,a534",
                        "toggle 559,485 584,534",
                    ];

                    for case in TEST_CASES {
                        assert!(case.parse::<Command>().is_err());
                    }
                }
            }
        }
    }
}

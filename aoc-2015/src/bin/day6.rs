use std::{io::BufRead, path::PathBuf};

use lib::Command;

use crate::lib::{grid_part1, grid_part2};

fn main() -> Result<(), error::Error> {
    let filepath: PathBuf = std::env::args()
        .skip(1)
        .next()
        .expect("Provide a file path as first argument")
        .into();
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut grid1 = grid_part1::Grid::new();
    let mut grid2 = grid_part2::Grid::new();
    for cmd in reader
        .lines()
        .flat_map(|line| line.map(|line| line.parse::<Command>()))
    {
        let cmd = cmd?;
        grid1.apply_cmd(&cmd)?;
        grid2.apply_cmd(&cmd)?;
    }
    println!(
        "Part1: lit cells: {}",
        grid1.count(&grid_part1::GridCell(true))
    );
    println!("Part2: total brightness: {}", grid2.total_brightness());
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

    #[allow(dead_code)]
    const GRID_SZ: usize = 1000;

    #[allow(dead_code)]
    const FIRST: usize = 0;

    #[allow(dead_code)]
    const LAST: usize = GRID_SZ - 1;

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
        /// Can be at the same line.
        pub fn is_top_right_of(&self, other: &Point) -> bool {
            self.x >= other.x && self.y >= other.y && self != other
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Rectangle {
        pub start: Point,
        pub end: Point,
    }

    impl Rectangle {
        /// Returns true if the rectangle is inside the grid,
        /// end is at top-rignt of start (can be at same line).
        /// Can be zero-sized.
        pub fn is_valid(&self) -> bool {
            self.start.is_inside_grid()
                && self.end.is_inside_grid()
                && (self.end.is_top_right_of(&self.start) || self.start == self.end)
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
        InvalidRect(Rectangle),
    }

    pub mod grid_part1 {
        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct GridCell(pub bool);

        impl GridCell {
            pub fn apply_instruction(&mut self, instruction: &Instruction) {
                match instruction {
                    Instruction::TurnOn => self.0 = true,
                    Instruction::TurnOff => self.0 = false,
                    Instruction::Toggle => self.0 = !self.0,
                }
            }
        }

        pub struct Grid(Vec<Vec<GridCell>>);

        impl Grid {
            pub fn new() -> Self {
                Self(vec![vec![GridCell(false); GRID_SZ]; GRID_SZ])
            }

            pub fn count(&self, want: &GridCell) -> usize {
                self.0
                    .iter()
                    .map(|row| row.iter().filter(|&cell| cell == want).count())
                    .sum()
            }
        }

        impl Grid {
            pub fn apply_cmd(&mut self, cmd: &Command) -> Result<(), GridError> {
                if !cmd.rect.is_valid() {
                    println!("Invalid rect: {:?}", cmd.rect);
                    return Err(GridError::InvalidRect(cmd.rect));
                }

                let Command { rect, instruction } = cmd;
                for y in rect.start.y..=rect.end.y {
                    for x in rect.start.x..=rect.end.x {
                        self.0[y][x].apply_instruction(instruction);
                    }
                }
                Ok(())
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            mod grid {
                use super::*;

                mod count {
                    use super::*;
                    use crate::lib::GRID_SZ;

                    #[test]
                    fn test_count() {
                        let grid = Grid::new();
                        assert_eq!(grid.count(&GridCell(false)), GRID_SZ * GRID_SZ);
                        assert_eq!(grid.count(&GridCell(true)), 0);
                    }
                }

                mod new {
                    use crate::lib::grid_part1::{Grid, GridCell};

                    #[test]
                    fn test_new() {
                        let grid = Grid::new();
                        assert_eq!(grid.count(&GridCell(true)), 0);
                    }
                }

                mod apply_cmd {
                    use super::*;
                    use crate::lib::{FIRST, LAST};

                    #[test]
                    fn test_turn_on() {
                        let mut grid = Grid::new();
                        let cmd = Command {
                            instruction: Instruction::TurnOn,
                            rect: Rectangle {
                                start: Point { x: FIRST, y: FIRST },
                                end: Point { x: LAST, y: LAST },
                            },
                        };
                        grid.apply_cmd(&cmd).unwrap();
                        assert_eq!(grid.count(&GridCell(false)), 0);
                    }

                    #[test]
                    fn test_turn_off() {
                        let mut grid = Grid::new();
                        let cmd = Command {
                            instruction: Instruction::TurnOn,
                            rect: Rectangle {
                                start: Point { x: FIRST, y: FIRST },
                                end: Point { x: LAST, y: LAST },
                            },
                        };
                        grid.apply_cmd(&cmd).unwrap();
                        let cmd = Command {
                            instruction: Instruction::TurnOff,
                            ..cmd
                        };
                        grid.apply_cmd(&cmd).unwrap();
                        assert_eq!(grid.count(&GridCell(true)), 0);
                    }

                    #[test]
                    fn test_toggle() {
                        let mut grid = Grid::new();
                        let cmd = Command {
                            instruction: Instruction::Toggle,
                            rect: Rectangle {
                                start: Point { x: FIRST, y: FIRST },
                                end: Point { x: LAST, y: LAST },
                            },
                        };
                        grid.apply_cmd(&cmd).unwrap();
                        assert_eq!(grid.count(&GridCell(false)), 0);
                        grid.apply_cmd(&cmd).unwrap();
                        assert_eq!(grid.count(&GridCell(true)), 0);
                    }
                }
            }
        }
    }

    pub mod grid_part2 {
        use std::vec;

        use super::*;

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct GridCell(pub usize);

        impl GridCell {
            pub fn apply_instruction(&mut self, instruction: &Instruction) {
                match instruction {
                    Instruction::TurnOn => self.0 += 1,
                    Instruction::TurnOff => {
                        if self.0 > 0 {
                            self.0 -= 1;
                        }
                    }
                    Instruction::Toggle => self.0 += 2,
                }
            }
        }

        pub struct Grid(Vec<Vec<GridCell>>);

        impl Grid {
            pub fn new() -> Self {
                Self(vec![vec![GridCell(0); GRID_SZ]; GRID_SZ])
            }

            pub fn total_brightness(&self) -> usize {
                self.0
                    .iter()
                    .map(|row| row.iter().map(|cell| cell.0).sum::<usize>())
                    .sum()
            }
        }

        impl Grid {
            pub fn apply_cmd(&mut self, cmd: &Command) -> Result<(), GridError> {
                if !cmd.rect.is_valid() {
                    println!("Invalid rect: {:?}", cmd.rect);
                    return Err(GridError::InvalidRect(cmd.rect));
                }

                let rect = cmd.rect;
                for y in rect.start.y..=rect.end.y {
                    for x in rect.start.x..=rect.end.x {
                        self.0[y][x].apply_instruction(&cmd.instruction);
                    }
                }
                Ok(())
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            mod grid {
                use super::*;

                mod total_brightness {
                    use super::*;
                    use crate::lib::GRID_SZ;

                    #[test]
                    fn test_total_brightness_1() {
                        let grid = Grid::new();
                        assert_eq!(grid.total_brightness(), 0);
                    }

                    #[test]
                    fn test_total_brightness_2() {
                        let mut grid = Grid::new();
                        for y in 0..grid.0.len() {
                            for x in 0..grid.0[y].len() {
                                grid.0[y][x] = GridCell(1);
                            }
                        }
                        assert_eq!(grid.total_brightness(), GRID_SZ * GRID_SZ);
                    }
                }

                mod new {
                    use super::*;

                    #[test]
                    fn test_new() {
                        let grid = Grid::new();
                        assert_eq!(grid.total_brightness(), 0);
                    }
                }

                mod apply_cmd {
                    use super::*;
                    use crate::lib::{FIRST, LAST};

                    #[test]
                    fn test_turn_on() {
                        let mut grid = Grid::new();
                        let cmd = Command {
                            instruction: Instruction::TurnOn,
                            rect: Rectangle {
                                start: Point { x: FIRST, y: FIRST },
                                end: Point { x: LAST, y: LAST },
                            },
                        };
                        grid.apply_cmd(&cmd).unwrap();
                        assert_eq!(grid.total_brightness(), GRID_SZ * GRID_SZ);
                    }

                    #[test]
                    fn test_turn_off() {
                        let mut grid = Grid::new();
                        let cmd = Command {
                            instruction: Instruction::TurnOn,
                            rect: Rectangle {
                                start: Point { x: FIRST, y: FIRST },
                                end: Point { x: LAST, y: LAST },
                            },
                        };
                        grid.apply_cmd(&cmd).unwrap();
                        let cmd = Command {
                            instruction: Instruction::TurnOff,
                            ..cmd
                        };
                        grid.apply_cmd(&cmd).unwrap();
                        assert_eq!(grid.total_brightness(), 0);
                    }

                    #[test]
                    fn test_toggle() {
                        let mut grid = Grid::new();
                        let cmd = Command {
                            instruction: Instruction::Toggle,
                            rect: Rectangle {
                                start: Point { x: FIRST, y: FIRST },
                                end: Point { x: LAST, y: LAST },
                            },
                        };
                        grid.apply_cmd(&cmd).unwrap();
                        assert_eq!(grid.total_brightness(), GRID_SZ * GRID_SZ * 2);
                        grid.apply_cmd(&cmd).unwrap();
                        assert_eq!(grid.total_brightness(), GRID_SZ * GRID_SZ * 4);
                    }
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
                fn test_line_1() {
                    test!((3, 3), (4, 3) => true);
                }

                #[test]
                fn test_line_2() {
                    test!((3, 3), (3, 4) => true);
                }

                #[test]
                fn test_line_invalie_1() {
                    test!((3, 3), (3, 2) => false);
                }

                #[test]
                fn test_line_invalid_2() {
                    test!((3, 3), (2, 3) => false);
                }

                #[test]
                fn test_zero_1() {
                    test!((0, 0), (0, 0) => true);
                }

                #[test]
                fn test_zero_2() {
                    test!((5, 5), (5, 5) => true);
                }

                #[test]
                fn test_zero_3() {
                    test!((LAST, LAST), (LAST, LAST) => true);
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

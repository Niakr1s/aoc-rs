use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Light {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone)]
struct Neighbours {
    me: Light,
    n: Option<Light>,
    ne: Option<Light>,
    e: Option<Light>,
    se: Option<Light>,
    s: Option<Light>,
    sw: Option<Light>,
    w: Option<Light>,
    nw: Option<Light>,
}

impl Neighbours {
    fn count_on(&self) -> usize {
        [
            self.n, self.ne, self.e, self.se, self.s, self.sw, self.w, self.nw,
        ]
        .into_iter()
        .filter_map(|x| x)
        .filter(|&x| x == Light::On)
        .count()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PosError {
    #[error("invalid row: got {got}, max {max}")]
    InvalidRow { got: usize, max: usize },
    #[error("invalid col: got {got}, max {max}")]
    InvalidCol { got: usize, max: usize },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    lights: Vec<Vec<Light>>,
    stucked: Vec<(usize, usize)>,
}

impl Grid {
    pub fn with_stucked_corners(self) -> Self {
        let (rows, cols) = (self.rows(), self.cols());
        if rows == 0 && cols == 0 {
            return self;
        }

        let mut corners = vec![(0, 0)];
        if cols > 1 {
            corners.push((0, cols - 1));
        }
        if rows > 1 {
            corners.push((rows - 1, 0));
        }
        if cols > 1 && rows > 1 {
            corners.push((rows - 1, cols - 1));
        }

        self.with_stucked(corners)
    }

    pub fn with_stucked(mut self, mut stucked: Vec<(usize, usize)>) -> Self {
        for (row, col) in &stucked {
            self.lights[*row][*col] = Light::On;
        }
        self.stucked.append(&mut stucked);
        self
    }

    pub fn next_step(&self) -> Self {
        let next_lights = self
            .lights
            .iter()
            .enumerate()
            .map(|(row, row_vec)| {
                row_vec
                    .iter()
                    .enumerate()
                    .map(|(col, &light)| {
                        if self.stucked.contains(&(row, col)) {
                            return Light::On;
                        }
                        let neighbours = self.get_neighbours(row, col).unwrap();
                        let count_on = neighbours.count_on();
                        match light {
                            Light::On => {
                                if count_on == 2 || count_on == 3 {
                                    Light::On
                                } else {
                                    Light::Off
                                }
                            }
                            Light::Off => {
                                if count_on == 3 {
                                    Light::On
                                } else {
                                    Light::Off
                                }
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Grid {
            lights: next_lights,
            stucked: self.stucked.clone(),
        }
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Result<Neighbours, PosError> {
        let me = self.get(row, col)?;
        Ok(Neighbours {
            me,
            n: self.checked_get(row.checked_sub(1), Some(col)),
            ne: self.checked_get(row.checked_sub(1), col.checked_add(1)),
            e: self.checked_get(Some(row), col.checked_add(1)),
            se: self.checked_get(row.checked_add(1), col.checked_add(1)),
            s: self.checked_get(row.checked_add(1), Some(col)),
            sw: self.checked_get(row.checked_add(1), col.checked_sub(1)),
            w: self.checked_get(Some(row), col.checked_sub(1)),
            nw: self.checked_get(row.checked_sub(1), col.checked_sub(1)),
        })
    }

    fn checked_get(&self, row: Option<usize>, col: Option<usize>) -> Option<Light> {
        match (row, col) {
            (Some(row), Some(col)) => self.get(row, col).ok(),
            _ => None,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Result<Light, PosError> {
        let me = self
            .lights
            .get(row)
            .ok_or(PosError::InvalidRow {
                got: row,
                max: self.rows(),
            })?
            .get(col)
            .ok_or(PosError::InvalidCol {
                got: col,
                max: self.cols(),
            })?;
        Ok(*me)
    }

    pub fn count_on(&self) -> usize {
        self.count(Light::On)
    }

    pub fn count_off(&self) -> usize {
        self.count(Light::Off)
    }

    fn count(&self, light: Light) -> usize {
        self.lights
            .iter()
            .flatten()
            .filter(|&l| l == &light)
            .count()
    }

    pub fn rows(&self) -> usize {
        self.lights.len()
    }

    pub fn cols(&self) -> usize {
        self.lights[0].len()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("grid is empty")]
    Empty,
    #[error("grid is not rectangular")]
    LenMismatch,
    #[error("invalid character")]
    InvalidChar,
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lights = s
            .trim()
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| -> Result<Light, Self::Err> {
                        match c {
                            '.' => Ok(Light::Off),
                            '#' => Ok(Light::On),
                            _ => Err(ParseError::InvalidChar),
                        }
                    })
                    .collect::<Result<Vec<Light>, Self::Err>>()
            })
            .collect::<Result<Vec<Vec<Light>>, Self::Err>>()?;

        if lights.len() == 0 {
            return Err(ParseError::Empty);
        }

        if lights.len() > 1 {
            let width = lights[0].len();
            if !lights.iter().all(|l| l.len() == width) {
                return Err(ParseError::LenMismatch);
            }
        }

        Ok(Grid {
            lights,
            stucked: vec![],
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod grid {
        use super::*;

        mod with_stucked_corners {
            use super::*;

            #[test]
            fn turns_on_corners() {
                const INPUT: &str = r#"
                .#.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####..
"#;

                let grid = INPUT.parse::<Grid>().unwrap();
                let grid = grid.with_stucked_corners();
                assert_eq!(grid.lights[0][0], Light::On);
                assert_eq!(grid.lights[0][5], Light::On);
                assert_eq!(grid.lights[5][0], Light::On);
                assert_eq!(grid.lights[5][5], Light::On);
            }
        }

        mod next_step {
            use super::*;

            const STEPS: &[&'static str] = &[
                r#"
                .#.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####..
"#,
                r#"
                ..##..
                ..##.#
                ...##.
                ......
                #.....
                #.##..
"#,
                r#"
                ..###.
                ......
                ..###.
                ......
                .#....
                .#....
"#,
                r#"
                ...#..
                ......
                ...#..
                ..##..
                ......
                ......
"#,
                r#"
                ......
                ......
                ..##..
                ..##..
                ......
                ......
"#,
            ];

            const STUCKED_STEPS: &[&'static str] = &[
                r#"
                ##.#.#
                ...##.
                #....#
                ..#...
                #.#..#
                ####.#
"#,
                r#"
                #.##.#
                ####.#
                ...##.
                ......
                #...#.
                #.####
"#,
                r#"
                #..#.#
                #....#
                .#.##.
                ...##.
                .#..##
                ##.###
"#,
                r#"
                #...##
                ####.#
                ..##.#
                ......
                ##....
                ####.#
"#,
                r#"
                #.####
                #....#
                ...#..
                .##...
                #.....
                #.#..#
"#,
                r#"
                ##.###
                .##..#
                .##...
                .##...
                #.#...
                ##...#
"#,
            ];

            #[test]
            fn works() {
                let mut grid = STEPS[0].parse::<Grid>().unwrap();
                for &step in &STEPS[1..] {
                    let wanted_grid: Grid = step.parse().unwrap();
                    grid = grid.next_step();
                    assert_eq!(grid, wanted_grid);
                }
            }

            #[test]
            fn works_with_stucked_corners() {
                let mut grid = STUCKED_STEPS[0]
                    .parse::<Grid>()
                    .unwrap()
                    .with_stucked_corners();
                for &step in &STUCKED_STEPS[1..] {
                    let wanted_grid: Grid = step.parse().unwrap();
                    grid = grid.next_step();
                    assert_eq!(grid.lights, wanted_grid.lights);
                }
            }
        }

        mod get_neighbours {
            use super::*;

            const INPUT: &'static str = "#.#.\n#..#\n###.";

            #[test]
            fn middle() {
                let grid: Grid = INPUT.parse().unwrap();
                let neigh = grid.get_neighbours(1, 1).unwrap();
                assert_eq!(
                    neigh,
                    Neighbours {
                        me: Light::Off,
                        n: Some(Light::Off),
                        ne: Some(Light::On),
                        e: Some(Light::Off),
                        se: Some(Light::On),
                        s: Some(Light::On),
                        sw: Some(Light::On),
                        w: Some(Light::On),
                        nw: Some(Light::On),
                    }
                )
            }

            #[test]
            fn bot_right() {
                let grid: Grid = INPUT.parse().unwrap();
                let neigh = grid.get_neighbours(2, 3).unwrap();
                assert_eq!(
                    neigh,
                    Neighbours {
                        me: Light::Off,
                        n: Some(Light::On),
                        ne: None,
                        e: None,
                        se: None,
                        s: None,
                        sw: None,
                        w: Some(Light::On),
                        nw: Some(Light::Off),
                    }
                )
            }

            #[test]
            fn top_left() {
                let grid: Grid = INPUT.parse().unwrap();
                let neigh = grid.get_neighbours(0, 0).unwrap();
                assert_eq!(
                    neigh,
                    Neighbours {
                        me: Light::On,
                        n: None,
                        ne: None,
                        e: Some(Light::Off),
                        se: Some(Light::Off),
                        s: Some(Light::On),
                        sw: None,
                        w: None,
                        nw: None,
                    }
                )
            }
        }

        mod from_str {
            use super::*;

            const INPUT: &'static str = "#.#.\n#..#\n###.";

            #[test]
            fn works() {
                let grid: Grid = INPUT.parse().unwrap();
                assert_eq!(grid.lights.len(), 3);

                assert_eq!(
                    grid.lights[0],
                    vec![Light::On, Light::Off, Light::On, Light::Off]
                );
                assert_eq!(
                    grid.lights[1],
                    vec![Light::On, Light::Off, Light::Off, Light::On]
                );
                assert_eq!(
                    grid.lights[2],
                    vec![Light::On, Light::On, Light::On, Light::Off]
                );
            }
        }
    }
}

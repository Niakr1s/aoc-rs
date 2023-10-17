use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum PathItem {
    Vertex(String),
    Edge(u32),
}

pub type Path = Vec<PathItem>;
pub type Paths = Vec<Path>;

#[derive(Debug, PartialEq)]
pub struct Edge {
    from: String,
    to: String,
    dist: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Vertex {0} not found")]
    VertexNotFound(String),
}

pub struct Graph {
    map: HashMap<String, HashMap<String, u32>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            map: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, Edge { from, to, dist }: Edge) {
        self.add_single_edge(from.clone(), to.clone(), dist);
        self.add_single_edge(to, from, dist);
    }

    fn add_single_edge(&mut self, from: String, to: String, dist: u32) {
        self.map
            .entry(from)
            .or_insert(HashMap::new())
            .insert(to, dist);
    }

    pub fn get_all_paths(&self) -> Paths {
        let mut res = Vec::new();
        for from in self.map.keys() {
            res.append(self.get_paths(from.as_str()).unwrap().as_mut());
        }
        res
    }

    pub fn get_paths(&self, from: &str) -> Result<Paths, GraphError> {
        self.get_paths_(from, HashSet::new())
    }

    fn get_paths_(&self, from: &str, mut visited: HashSet<String>) -> Result<Paths, GraphError> {
        visited.insert(from.to_owned());

        let edges = self
            .map
            .get(from)
            .ok_or(GraphError::VertexNotFound(from.to_owned()))?;

        let mut res = Vec::new();
        for (to, dist) in edges {
            let head = vec![PathItem::Vertex(from.to_owned()), PathItem::Edge(*dist)];
            if !visited.contains(to) {
                let paths = self.get_paths_(to, visited.clone())?;

                if paths.len() == 0 {
                    let mut path = head.clone();
                    path.push(PathItem::Vertex(to.to_owned()));
                    res.push(path);
                } else {
                    for mut tail in paths {
                        let mut path = head.clone();
                        path.append(&mut tail);
                        res.push(path);
                    }
                }
            }
        }
        Ok(res)
    }
}

pub fn dist(path: &Path) -> u32 {
    path.iter().fold(0, |mut acc, item| {
        if let PathItem::Edge(dist) = item {
            acc += *dist;
        }
        acc
    })
}

pub fn shortest(paths: &Paths) -> Option<&Path> {
    paths.iter().min_by(|&a, &b| dist(a).cmp(&dist(b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dist {
        use super::*;

        #[test]
        fn test_dist() {
            let path: Path = vec![
                PathItem::Vertex("A".to_owned()),
                PathItem::Edge(1),
                PathItem::Vertex("B".to_owned()),
                PathItem::Edge(2),
                PathItem::Vertex("C".to_owned()),
                PathItem::Edge(5),
                PathItem::Vertex("D".to_owned()),
            ];
            assert_eq!(dist(&path), 8);
        }
    }

    mod shortest {
        use super::*;

        #[test]
        fn test_dist() {
            let path1: Path = vec![PathItem::Edge(1), PathItem::Edge(2), PathItem::Edge(5)];
            let path2: Path = vec![PathItem::Edge(1), PathItem::Edge(3)];
            let path3: Path = vec![PathItem::Edge(2), PathItem::Edge(5), PathItem::Edge(15)];
            let paths = vec![path1, path2, path3];

            assert_eq!(shortest(&paths), Some(&paths[1]));
        }
    }

    mod graph {
        use super::*;

        macro_rules! E {
            ($from: expr, $to: expr, $dist: expr) => {
                Edge {
                    from: $from.to_owned(),
                    to: $to.to_owned(),
                    dist: $dist,
                }
            };
        }

        #[test]
        fn test_get_all_paths_simple() {
            let mut graph = Graph::new();
            graph.add_edge(E!("A", "B", 1));
            assert_eq!(graph.map.get("A").unwrap().get("B").unwrap(), &1);
            assert_eq!(graph.map.get("B").is_some(), true);

            let a_paths = graph.get_paths("A").unwrap();
            assert_eq!(a_paths.len(), 1);
            assert_eq!(a_paths[0].len(), 3);

            let mut b_paths = graph.get_paths("B").unwrap();
            assert_eq!(b_paths.len(), 1);
            assert_eq!(a_paths[0].len(), 3);

            b_paths[0].reverse();
            assert_eq!(a_paths[0], b_paths[0]);
        }

        #[test]
        fn test_get_all_paths_from_website() {
            let mut graph = Graph::new();
            graph.add_edge(E!("London", "Dublin", 464));
            graph.add_edge(E!("London", "Belfast", 518));
            graph.add_edge(E!("Dublin", "Belfast", 141));

            let london_paths = graph.get_paths("London").unwrap();
            assert_eq!(london_paths.len(), 2);

            let dublin_paths = graph.get_paths("Dublin").unwrap();
            assert_eq!(dublin_paths.len(), 2);

            let belfast_paths = graph.get_paths("Belfast").unwrap();
            assert_eq!(belfast_paths.len(), 2);
        }
    }
}

mod from_str {
    use std::str::FromStr;

    use super::*;

    #[derive(Debug, thiserror::Error)]
    pub enum IntoEdgeParseError {
        #[error("No equal sign found")]
        NoEqualSign,
        #[error("Left side format error")]
        LeftSideFormatError,
        #[error("Distance parse error")]
        DistError(#[from] std::num::ParseIntError),
    }

    impl FromStr for Edge {
        type Err = IntoEdgeParseError;

        /// London to Dublin = 464
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some((left, right)) = s.split_once('=') {
                let dist: u32 = right.trim().parse()?;
                if let Some((left, right)) = left.split_once("to") {
                    let (left, right) = (left.trim(), right.trim());
                    if left.is_empty() || right.is_empty() {
                        Err(IntoEdgeParseError::LeftSideFormatError)
                    } else {
                        Ok(Edge {
                            from: left.to_owned(),
                            to: right.to_owned(),
                            dist,
                        })
                    }
                } else {
                    Err(IntoEdgeParseError::LeftSideFormatError)
                }
            } else {
                Err(IntoEdgeParseError::NoEqualSign)
            }
        }
    }

    #[cfg(test)]
    mod test_from_str_edge {
        use super::*;

        #[test]
        fn test_valid_1() {
            assert_eq!(
                Edge::from_str("London to Dublin = 464").unwrap(),
                Edge {
                    from: "London".to_owned(),
                    to: "Dublin".to_owned(),
                    dist: 464,
                }
            );
        }

        #[test]
        #[should_panic]
        fn test_invalid_1() {
            Edge::from_str("London Dublin = 464").unwrap();
        }

        #[test]
        #[should_panic]
        fn test_invalid_2() {
            Edge::from_str("London to = 464").unwrap();
        }

        #[test]
        #[should_panic]
        fn test_invalid_3() {
            Edge::from_str("to Dublin = 464").unwrap();
        }

        #[test]
        #[should_panic]
        fn test_invalid_4() {
            Edge::from_str("London to Dublin = ").unwrap();
        }
    }
}

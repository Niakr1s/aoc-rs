use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathItem {
    Vertex(String),
    Edge(u32),
}

#[derive(
    Debug,
    PartialEq,
    Clone,
    Eq,
    Hash,
    derive_more::From,
    derive_more::AsMut,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::Index,
    derive_more::IndexMut,
)]
pub struct Path(Vec<PathItem>);

#[derive(
    Debug,
    PartialEq,
    derive_more::From,
    derive_more::AsMut,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::Index,
    derive_more::IndexMut,
)]
pub struct Paths(Vec<Path>);

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub dist: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Vertex {0} not found")]
    VertexNotFound(String),
}

pub struct Graph<'a> {
    map: HashMap<&'a str, HashMap<&'a str, u32>>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph {
            map: HashMap::new(),
        }
    }

    pub fn from_vec(edges: &'a Vec<Edge>) -> Graph<'a> {
        Graph::from_iter(edges.iter())
    }

    fn add_edge(&mut self, Edge { from, to, dist }: &'a Edge) {
        self.add_single_edge(from, to, *dist);
        self.add_single_edge(to, from, *dist);
    }

    fn add_single_edge(&mut self, from: &'a String, to: &'a String, dist: u32) {
        self.map
            .entry(from)
            .or_insert(HashMap::new())
            .insert(to, dist);
    }

    pub fn get_all_paths(&self) -> Paths {
        let mut res = Vec::new();
        for &from in self.map.keys() {
            res.append(self.get_paths(from).unwrap().as_mut());
        }
        Paths(res)
    }

    pub fn get_paths(&self, from: &str) -> Result<Paths, GraphError> {
        self.get_paths_to_inner(HashSet::new(), &from.to_owned(), None)
    }

    pub fn get_paths_to(&self, from: &str, dest: &str) -> Result<Paths, GraphError> {
        if !self.map.contains_key(dest) {
            return Err(GraphError::VertexNotFound(dest.to_owned()));
        }
        self.get_paths_to_inner(HashSet::new(), &from.to_owned(), Some(dest))
    }

    /// * `dest` -> Must be present in graph, othervise function will panic.
    /// Caller must ensure of it before calling this function.
    /// It's done to avoid checks for each iteration.
    fn get_paths_to_inner(
        &self,
        mut visited: HashSet<String>,
        from: &str,
        dest: Option<&str>,
    ) -> Result<Paths, GraphError> {
        visited.insert(from.to_owned());

        let edges = self
            .map
            .get(from)
            .ok_or(GraphError::VertexNotFound(from.to_owned()))?;

        if dest.is_some_and(|dest| from == dest) {
            return Ok(Paths(vec![Path(vec![PathItem::Vertex(from.to_owned())])]));
        }

        let mut res = Vec::new();
        for (&to, dist) in edges {
            let head = vec![PathItem::Vertex(from.to_owned()), PathItem::Edge(*dist)];
            if !visited.contains(to) {
                let paths = self.get_paths_to_inner(visited.clone(), to, dest)?;

                if paths.0.len() == 0 {
                    let mut path = head.clone();
                    path.push(PathItem::Vertex(to.to_owned()));
                    res.push(Path(path));
                } else {
                    for mut tail in paths.0 {
                        let mut path = head.clone();
                        path.append(&mut tail.0);
                        res.push(Path(path));
                    }
                }
            }
        }
        Ok(Paths(res))
    }
}

impl<'a> FromIterator<&'a Edge> for Graph<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Edge>>(iter: T) -> Self {
        let mut graph = Graph::new();
        iter.into_iter().for_each(|edge| graph.add_edge(edge));
        graph
    }
}

impl Path {
    pub fn dist(&self) -> u32 {
        self.0.iter().fold(0, |mut acc, item| {
            if let PathItem::Edge(dist) = item {
                acc += *dist;
            }
            acc
        })
    }
}

impl Paths {
    pub fn shortest(&self) -> Option<&Path> {
        self.0.iter().min_by(|&a, &b| a.dist().cmp(&b.dist()))
    }

    pub fn longest(&self) -> Option<&Path> {
        self.0.iter().max_by(|&a, &b| a.dist().cmp(&b.dist()))
    }

    pub fn sort_by_dist(&mut self) {
        self.sort_by_key(|path| (path.dist(), path.len()));
    }

    pub fn sort_by_len(&mut self) {
        self.sort_by_key(|path| (path.len(), path.dist()));
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .iter()
            .map(|item| match item {
                PathItem::Vertex(v) => v.to_owned(),
                PathItem::Edge(e) => e.to_string(),
            })
            .collect::<Vec<_>>();
        write!(f, "{}", items.join(" -> "))
    }
}

impl std::fmt::Display for Paths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for path in self.0.iter() {
            writeln!(f, "{}", path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod path {
        use super::*;

        #[test]
        fn dist() {
            let path: Path = Path(vec![
                PathItem::Vertex("A".to_owned()),
                PathItem::Edge(1),
                PathItem::Vertex("B".to_owned()),
                PathItem::Edge(2),
                PathItem::Vertex("C".to_owned()),
                PathItem::Edge(5),
                PathItem::Vertex("D".to_owned()),
            ]);
            assert_eq!(path.dist(), 8);
        }
    }

    mod paths {
        use super::*;

        #[test]
        fn shortest() {
            let path1 = vec![PathItem::Edge(1), PathItem::Edge(2), PathItem::Edge(5)];
            let path2 = vec![PathItem::Edge(1), PathItem::Edge(3)];
            let path3 = vec![PathItem::Edge(2), PathItem::Edge(5), PathItem::Edge(15)];
            let paths = Paths(vec![Path(path1), Path(path2), Path(path3)]);

            assert_eq!(paths.shortest(), Some(&paths[1]));
        }

        #[test]
        fn sort_by_dist() {
            let path1 = vec![PathItem::Edge(5)];
            let path2 = vec![PathItem::Edge(1), PathItem::Edge(1), PathItem::Edge(1)];

            let mut paths = Paths(vec![Path(path1), Path(path2)]);
            paths.sort_by_dist();
            assert_eq!(paths[0].dist(), 3);
            assert_eq!(paths[1].dist(), 5);
        }

        #[test]
        fn sort_by_len() {
            let path2 = vec![PathItem::Edge(1), PathItem::Edge(1), PathItem::Edge(1)];
            let path1 = vec![PathItem::Edge(5)];

            let mut paths = Paths(vec![Path(path1), Path(path2)]);
            paths.sort_by_len();
            assert_eq!(paths[0].len(), 1);
            assert_eq!(paths[1].len(), 3);
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
        fn get_all_paths_simple() {
            let edges: &[Edge] = &[E!("A", "B", 1)];
            let graph = Graph::from_iter(edges.into_iter());

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
        fn get_all_paths_from_website() {
            let edges: &[Edge] = &[
                E!("London", "Dublin", 464),
                E!("London", "Belfast", 518),
                E!("Dublin", "Belfast", 141),
            ];
            let graph = Graph::from_iter(edges.into_iter());

            let london_paths = graph.get_paths("London").unwrap();
            assert_eq!(london_paths.len(), 2);

            let dublin_paths = graph.get_paths("Dublin").unwrap();
            assert_eq!(dublin_paths.len(), 2);

            let belfast_paths = graph.get_paths("Belfast").unwrap();
            assert_eq!(belfast_paths.len(), 2);
        }

        #[test]
        fn get_paths_to_1() {
            let edges: &[Edge] = &[
                E!("London", "Dublin", 464),
                E!("London", "Belfast", 518),
                E!("Dublin", "Belfast", 141),
            ];
            let graph = Graph::from_iter(edges.into_iter());

            let mut paths: Paths = graph.get_paths_to("London", "Belfast").unwrap();
            assert_eq!(paths.len(), 2);
            paths.sort_by_key(|path| path.len());
            assert_eq!(paths[0].len(), 3);
            assert_eq!(paths[0].dist(), 518);
            assert_eq!(paths[1].len(), 5);
            assert_eq!(paths[1].dist(), 605);
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
    mod tests {
        use super::*;

        mod edge {
            use super::*;

            #[test]
            fn valid_1() {
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
            fn invalid_1() {
                Edge::from_str("London Dublin = 464").unwrap();
            }

            #[test]
            #[should_panic]
            fn invalid_2() {
                Edge::from_str("London to = 464").unwrap();
            }

            #[test]
            #[should_panic]
            fn invalid_3() {
                Edge::from_str("to Dublin = 464").unwrap();
            }

            #[test]
            #[should_panic]
            fn invalid_4() {
                Edge::from_str("London to Dublin = ").unwrap();
            }
        }
    }
}

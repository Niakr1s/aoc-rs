use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct Replacements(Vec<(String, String)>);

impl Replacements {
    pub fn distinct_moleculas<'a>(
        &'a self,
        molecula: &'a str,
    ) -> impl Iterator<Item = String> + 'a {
        Moleculas::new(&self.0, molecula).unique()
    }

    pub fn collapsed_moleculas<'a>(
        &'a self,
        molecula: &'a str,
    ) -> impl Iterator<Item = String> + 'a {
        CollapsedMoleculas::new(&self.0, molecula).unique()
    }
}

pub fn steps(start: &str, want: &str, replacements: &Replacements) -> Option<usize> {
    // println!("\n{} -> {}\n", start, want);
    if start == want {
        return Some(0);
    } else if start.len() >= want.len() {
        return None;
    }

    replacements
        .distinct_moleculas(start)
        .into_iter()
        .filter_map(|repl| steps(&repl, want, replacements).map(|i| i + 1))
        .min()
}

struct CollapsedMoleculas<'a> {
    replacements: std::slice::Iter<'a, (String, String)>,
    current_replacement: Option<&'a (String, String)>,
    molecula: &'a str,
    molecula_idx: usize,
}

impl<'a> CollapsedMoleculas<'a> {
    pub fn new(replacements: &'a Vec<(String, String)>, molecula: &'a str) -> Self {
        CollapsedMoleculas {
            replacements: replacements.iter(),
            current_replacement: None,
            molecula,
            molecula_idx: 0,
        }
    }
}

impl<'a> Iterator for CollapsedMoleculas<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&(ref to, ref from)) = self.current_replacement {
            let idx = self.molecula[self.molecula_idx..]
                .find(from)
                .map(|idx| idx + self.molecula_idx);
            if let Some(idx) = idx {
                self.molecula_idx = idx + from.len();

                let mut ret = String::new();
                ret.push_str(&self.molecula[..idx]);
                ret.push_str(&to);
                ret.push_str(&self.molecula[self.molecula_idx..]);
                if ret != "e" && ret.chars().any(|c| c == 'e') {
                    return self.next();
                } else {
                    return Some(ret);
                }
            } else {
                self.molecula_idx = 0;
                self.current_replacement = self.replacements.next();
                return self.next();
            }
        } else {
            self.current_replacement = self.replacements.next();
            if self.current_replacement.is_some() {
                return self.next();
            } else {
                return None;
            }
        }
    }
}

struct Moleculas<'a> {
    replacements: std::slice::Iter<'a, (String, String)>,
    current_replacement: Option<&'a (String, String)>,
    molecula: &'a str,
    molecula_idx: usize,
}

impl<'a> Moleculas<'a> {
    pub fn new(replacements: &'a Vec<(String, String)>, molecula: &'a str) -> Self {
        Moleculas {
            replacements: replacements.iter(),
            current_replacement: None,
            molecula,
            molecula_idx: 0,
        }
    }
}

impl<'a> Iterator for Moleculas<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&(ref from, ref to)) = self.current_replacement {
            let idx = self.molecula[self.molecula_idx..]
                .find(from)
                .map(|idx| idx + self.molecula_idx);
            if let Some(idx) = idx {
                self.molecula_idx = idx + from.len();

                let mut ret = String::new();
                ret.push_str(&self.molecula[..idx]);
                ret.push_str(&to);
                ret.push_str(&self.molecula[self.molecula_idx..]);
                return Some(ret);
            } else {
                self.molecula_idx = 0;
                self.current_replacement = self.replacements.next();
                return self.next();
            }
        } else {
            self.current_replacement = self.replacements.next();
            if self.current_replacement.is_some() {
                return self.next();
            } else {
                return None;
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("No delimeter")]
    NoDelimeter,
}

impl FromStr for Replacements {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = s
            .lines()
            .filter(|l| l.trim().len() != 0)
            .map(|l| -> Result<(String, String), Self::Err> {
                let (from, to) = l.split_once("=>").ok_or(ParseError::NoDelimeter)?;
                Ok((from.trim().to_owned(), to.trim().to_owned()))
            })
            .collect::<Result<_, _>>()?;
        Ok(Replacements(ret))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    mod steps {
        use super::*;

        #[test]
        fn works() {
            let r = vec![
                ("e".to_owned(), "H".to_owned()),
                ("e".to_owned(), "O".to_owned()),
                ("H".to_owned(), "HO".to_owned()),
                ("H".to_owned(), "OH".to_owned()),
                ("O".to_owned(), "HH".to_owned()),
            ];
            let r = Replacements(r);
            assert_eq!(steps("e", "HOH", &r), Some(3));
            assert_eq!(steps("e", "HOHOHO", &r), Some(6));
        }
    }

    mod collapsed_moleculas {
        use super::*;

        #[test]
        fn works() {
            let replacements = Replacements(vec![
                ("e".to_owned(), "H".to_owned()),
                ("e".to_owned(), "O".to_owned()),
                ("H".to_owned(), "HO".to_owned()),
                ("H".to_owned(), "OH".to_owned()),
                ("O".to_owned(), "HH".to_owned()),
            ]);
            let molecula = "HOH".to_owned();
            let mut d = replacements.collapsed_moleculas(&molecula);

            assert_eq!(d.next(), Some("HH".to_owned()));
            assert_eq!(d.next(), None);
        }
    }

    mod moleculas {
        use super::*;

        #[test]
        fn works() {
            let r = vec![
                ("H".to_owned(), "HO".to_owned()),
                ("H".to_owned(), "OH".to_owned()),
                ("O".to_owned(), "HH".to_owned()),
            ];
            let f = "HOH".to_owned();
            let mut d = Moleculas::new(&r, &f);
            assert_eq!(d.next(), Some("HOOH".to_owned()));
            assert_eq!(d.next(), Some("HOHO".to_owned()));
            assert_eq!(d.next(), Some("OHOH".to_owned()));
            assert_eq!(d.next(), Some("HOOH".to_owned()));
            assert_eq!(d.next(), Some("HHHH".to_owned()));
            assert_eq!(d.next(), None);
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn works() {
            let t = Replacements::from_str("abc => xyz\nFe => Ca\n").unwrap();
            assert_eq!(t.0.len(), 2);
            assert_eq!(t.0[0], ("abc".to_owned(), "xyz".to_owned()));
            assert_eq!(t.0[1], ("Fe".to_owned(), "Ca".to_owned()));
        }
    }
}

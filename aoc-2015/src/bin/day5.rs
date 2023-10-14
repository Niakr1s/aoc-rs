use std::io::BufRead;

use nice::{Check, Checker};

use crate::nice::{CheckerPart1, CheckerPart2};

fn main() -> Result<(), std::io::Error> {
    let path = std::env::args().skip(1).next().unwrap();
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    println!(
        "Number of nice strings for part1: {}",
        run(&lines, &CheckerPart1::default())
    );
    println!(
        "Number of nice strings for part2: {}",
        run(&lines, &CheckerPart2::default())
    );

    Ok(())
}

fn run(lines: &Vec<String>, checker: &impl Checker) -> usize {
    lines.iter().filter(|line| line.is_nice(checker)).count()
}

mod nice {
    use std::collections::HashSet;

    pub trait Checker {
        fn is_nice<S: AsRef<str>>(&self, string: S) -> bool;
    }

    pub trait Check {
        fn is_nice(&self, checker: &impl Checker) -> bool;
    }

    impl<T: AsRef<str>> Check for T {
        fn is_nice(&self, checker: &impl Checker) -> bool {
            checker.is_nice(self)
        }
    }

    pub struct CheckerPart1 {
        want_vowels: usize,
        want_double_letter: bool,
        exclude_strings: Vec<String>,
    }

    impl Default for CheckerPart1 {
        fn default() -> Self {
            Self::new(3, true, vec!["ab", "cd", "pq", "xy"])
        }
    }

    impl Checker for CheckerPart1 {
        fn is_nice<S: AsRef<str>>(&self, string: S) -> bool {
            let string = string.as_ref();
            if Self::has_vowels(string) < self.want_vowels {
                return false;
            }
            if self.want_double_letter ^ Self::has_double_letter(string) {
                return false;
            }
            if self.has_excluded_strings(string) {
                return false;
            }
            true
        }
    }

    impl CheckerPart1 {
        fn new<S: Into<String>>(
            want_vowels: usize,
            want_double_letter: bool,
            excluded_strings: Vec<S>,
        ) -> Self {
            Self {
                want_vowels,
                want_double_letter,
                exclude_strings: excluded_strings.into_iter().map(|s| s.into()).collect(),
            }
        }

        fn has_double_letter(string: &str) -> bool {
            let string: Vec<char> = string.chars().collect();
            string.windows(2).any(|w| w[0] == w[1])
        }

        fn has_vowels(string: &str) -> usize {
            const VOWELS: &str = "aeiou";
            string
                .chars()
                .fold(0, |acc, c| if VOWELS.contains(c) { acc + 1 } else { acc })
        }

        fn has_excluded_strings(&self, string: &str) -> bool {
            for excluded_string in &self.exclude_strings {
                if string.contains(excluded_string) {
                    return true;
                }
            }
            false
        }
    }

    pub struct CheckerPart2 {}

    impl Default for CheckerPart2 {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Checker for CheckerPart2 {
        fn is_nice<S: AsRef<str>>(&self, string: S) -> bool {
            let string = string.as_ref();
            self.has_any_non_overlapped_duplex(string) && self.has_any_symmetrical_triplex(string)
        }
    }

    impl CheckerPart2 {
        fn new() -> Self {
            Self {}
        }

        fn has_any_non_overlapped_duplex(&self, string: &str) -> bool {
            let chars: Vec<_> = string.chars().collect();
            let duplexes: Vec<_> = chars.windows(2).collect();
            if HashSet::<_>::from_iter(duplexes.iter()).len() == duplexes.len() {
                return false;
            }

            for i in 0..(duplexes.len() - 1) {
                let (c1, c2) = (duplexes[i], duplexes[i + 1]);
                if c1 == c2 {
                    return false;
                }
            }
            true
        }

        fn has_any_symmetrical_triplex(&self, string: &str) -> bool {
            let chars: Vec<_> = string.chars().collect();
            chars.windows(3).any(|w| w[0] == w[2])
        }
    }

    #[cfg(test)]
    mod tests_checker_part1 {
        use super::*;

        fn checker() -> CheckerPart1 {
            CheckerPart1::default()
        }

        #[test]
        fn test_is_nice1() {
            assert!("ugknbfddgicrmopn".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice2() {
            assert!("aaa".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice3() {
            assert!(!"jchzalrnumimnmhp".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice4() {
            assert!(!"haegwjzuvuyypxyu".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice5() {
            assert!(!"dvszwmarrgswjxmb".is_nice(&checker()));
        }
    }

    #[cfg(test)]
    mod tests_checker_part2 {
        use super::*;

        fn checker() -> CheckerPart2 {
            CheckerPart2::default()
        }

        #[test]
        fn test_is_nice1() {
            assert!("qjhvhtzxzqqjkmpb".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice2() {
            assert!("xxyxx".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice3() {
            assert!(!"uurcxstgmygtbstg".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice4() {
            assert!(!"ieodomkazucvgmuy".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice5() {
            assert!("xyxy".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice6() {
            assert!(!"aabcdefgaa".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice7() {
            assert!(!"aaa".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice8() {
            assert!(!"xyx".is_nice(&checker()));
        }

        #[test]
        fn test_is_nice9() {
            assert!(!"abcdefeghi".is_nice(&checker()));
        }
    }
}

use std::io::BufRead;

use nice::is_nice;

use crate::nice::NiceStringCheckerPart1;

fn main() -> Result<(), std::io::Error> {
    let path = std::env::args().skip(1).next().unwrap();
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    let checker = NiceStringCheckerPart1::default();
    let count: usize =
        reader
            .lines()
            .try_fold(0, |acc, line| -> Result<usize, std::io::Error> {
                let line = line?;
                if is_nice(&line, &checker) {
                    Ok(acc + 1)
                } else {
                    Ok(acc)
                }
            })?;
    println!("Number of nice strings: {count}");

    Ok(())
}

mod nice {
    pub trait NiceStringChecker {
        fn is_nice<S: AsRef<str>>(&self, string: S) -> bool;
    }

    pub struct NiceStringCheckerPart1 {
        want_vowels: usize,
        want_double_letter: bool,
        exclude_strings: Vec<String>,
    }

    impl Default for NiceStringCheckerPart1 {
        fn default() -> Self {
            Self::new(3, true, vec!["ab", "cd", "pq", "xy"])
        }
    }

    impl NiceStringChecker for NiceStringCheckerPart1 {
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

    impl NiceStringCheckerPart1 {
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

    pub fn is_nice(s: &str, checker: &impl NiceStringChecker) -> bool {
        checker.is_nice(s)
    }

    #[cfg(test)]
    mod tests_is_nice {
        use super::*;

        #[test]
        fn test_is_nice1() {
            assert!(is_nice(
                "ugknbfddgicrmopn",
                &NiceStringCheckerPart1::default()
            ));
        }

        #[test]
        fn test_is_nice2() {
            assert!(is_nice("aaa", &NiceStringCheckerPart1::default()));
        }

        #[test]
        fn test_is_nice3() {
            assert!(!is_nice(
                "jchzalrnumimnmhp",
                &NiceStringCheckerPart1::default()
            ));
        }

        #[test]
        fn test_is_nice4() {
            assert!(!is_nice(
                "haegwjzuvuyypxyu",
                &NiceStringCheckerPart1::default()
            ));
        }

        #[test]
        fn test_is_nice5() {
            assert!(!is_nice(
                "dvszwmarrgswjxmb",
                &NiceStringCheckerPart1::default()
            ));
        }
    }
}

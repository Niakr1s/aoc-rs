use std::io::BufRead;

use strings::is_nice;

fn main() -> Result<(), std::io::Error> {
    let path = std::env::args().skip(1).next().unwrap();
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let count: usize =
        reader
            .lines()
            .try_fold(0, |acc, line| -> Result<usize, std::io::Error> {
                let line = line?;
                if is_nice(&line) {
                    Ok(acc + 1)
                } else {
                    Ok(acc)
                }
            })?;
    println!("Number of nice strings: {count}");

    Ok(())
}

mod strings {
    struct NiceStringChecker {
        want_vowels: usize,
        want_double_letter: bool,
        exclude_strings: Vec<String>,
    }

    impl NiceStringChecker {
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

        fn is_nice(&self, string: &str) -> bool {
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

    pub fn is_nice(s: &str) -> bool {
        NiceStringChecker::new(3, true, vec!["ab", "cd", "pq", "xy"]).is_nice(s)
    }

    #[cfg(test)]
    mod tests_is_nice {
        #[test]
        fn test_is_nice1() {
            assert!(super::is_nice("ugknbfddgicrmopn"));
        }

        #[test]
        fn test_is_nice2() {
            assert!(super::is_nice("aaa"));
        }

        #[test]
        fn test_is_nice3() {
            assert!(!super::is_nice("jchzalrnumimnmhp"));
        }

        #[test]
        fn test_is_nice4() {
            assert!(!super::is_nice("haegwjzuvuyypxyu"));
        }

        #[test]
        fn test_is_nice5() {
            assert!(!super::is_nice("dvszwmarrgswjxmb"));
        }
    }
}

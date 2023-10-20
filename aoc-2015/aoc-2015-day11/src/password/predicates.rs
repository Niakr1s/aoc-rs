pub enum Order {
    Asc,
    Desc,
}

pub fn contains_strait(s: &str, len: usize, order: &Order) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    if chars.len() < len {
        return false;
    }
    chars.windows(len).any(|w| is_ordered(&w, &order))
}

pub fn contains_sequences(s: &str, len: usize, at_least: usize, overlap: bool) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let mut pair_count = 0;
    let mut windows = chars.windows(len);
    while let Some(w) = windows.next() {
        if w.iter().all(|&c| c == w[0]) {
            pair_count += 1;
            if !overlap {
                if let Some(skip) = len.checked_sub(2) {
                    windows.nth(skip);
                }
            }
        }
        if at_least == pair_count {
            return true;
        }
    }
    false
}

fn is_ordered(s: &[char], order: &Order) -> bool {
    s.windows(2).all(|w| match order {
        Order::Asc => w[0] < w[1],
        Order::Desc => w[1] < w[0],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod contains_sequence {
        use super::*;

        fn test(s: &str, want_over: bool, want_no_over: bool) {
            assert_eq!(contains_sequences(s, 2, 3, true), want_over);
            assert_eq!(contains_sequences(s, 2, 3, false), want_no_over);
        }

        #[test]
        fn empty() {
            test("", false, false);
        }

        #[test]
        fn a() {
            test("a", false, false);
        }

        #[test]
        fn aa() {
            test("aa", false, false);
        }

        #[test]
        fn aaa() {
            test("aaa", false, false);
        }

        #[test]
        fn aaaa() {
            test("aaaa", true, false);
        }

        #[test]
        fn aaaaa() {
            test("aaaaa", true, false);
        }

        #[test]
        fn aaaaaa() {
            test("aaaaaa", true, true);
        }

        #[test]
        fn aabaabaa() {
            test("aabaabaa", true, true);
        }

        #[test]
        fn aabaaba() {
            test("aabaaba", false, false);
        }
    }

    mod contains_strait {
        use super::*;

        #[test]
        fn asc_true_1() {
            assert!(contains_strait("abc", 3, &Order::Asc));
        }

        #[test]
        fn asc_true_2() {
            assert!(contains_strait("babczy", 3, &Order::Asc));
        }

        #[test]
        fn asc_true_3() {
            assert!(contains_strait("babczydeff", 3, &Order::Asc));
        }

        #[test]
        fn asc_false_1() {
            assert!(!contains_strait("", 3, &Order::Asc));
        }

        #[test]
        fn asc_false_2() {
            assert!(!contains_strait("ab", 3, &Order::Asc));
        }
    }
}

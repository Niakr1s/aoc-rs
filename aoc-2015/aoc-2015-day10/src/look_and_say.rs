pub fn look_and_say(input: &str) -> String {
    let mut result = String::new();

    let mut count = 0;
    let mut iter = input.chars().peekable();
    while let Some(ch) = iter.next() {
        count += 1;
        let next = iter.peek();
        if next.is_none() || next != Some(&ch) {
            result.push_str(count.to_string().as_str());
            result.push(ch);
            count = 0;
        }
    }
    result
}

pub fn look_and_say_n_times(input: &str, times: usize) -> String {
    let mut input = input.to_owned();
    for _ in 0..times {
        input = look_and_say(&input);
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    mod look_and_say {
        use super::*;

        #[test]
        fn case1() {
            assert_eq!(look_and_say("1"), "11");
        }

        #[test]
        fn case2() {
            assert_eq!(look_and_say("11"), "21");
        }

        #[test]
        fn case3() {
            assert_eq!(look_and_say("21"), "1211");
        }

        #[test]
        fn case4() {
            assert_eq!(look_and_say("1211"), "111221");
        }

        #[test]
        fn case5() {
            assert_eq!(look_and_say("111221"), "312211");
        }
    }
}

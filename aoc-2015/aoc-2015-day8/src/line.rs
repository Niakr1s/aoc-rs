#[derive(Debug, Clone, derive_more::From)]
pub struct Line {
    line: String,
    utf8_len: usize,
}

#[allow(unused_macros)]
macro_rules! L {
    ($line:expr) => {
        Line::new(String::from($line)).unwrap()
    };
}

const ESC: char = '\\';
const QOUTE: char = '\"';
const X: char = 'x';

#[derive(Debug, thiserror::Error)]
pub enum LineValidationError {
    #[error("line should be quoted")]
    ShouldBeQuoted,
    #[error("line is too short")]
    TooShort,
}

pub mod line_encode {
    use super::*;

    impl Line {
        pub fn encode(&self) -> Line {
            let mut line = String::new();
            line.push(QOUTE);
            for ch in self.line.chars() {
                if matches!(ch, ESC | QOUTE) {
                    line.push(ESC);
                }
                line.push(ch);
            }
            line.push(QOUTE);
            Line::new(line).expect("should be valid")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_encode_1() {
            let l = L!(r#""""#).encode();
            assert_eq!(l.line, r#""\"\"""#);
            assert_eq!(l.len_in_code(), 6);
            assert_eq!(l.len_in_memory(), 2);
        }

        #[test]
        fn test_encode_2() {
            let l = L!(r#""abc""#).encode();
            assert_eq!(l.line, r#""\"abc\"""#);
            assert_eq!(l.len_in_code(), 9);
            assert_eq!(l.len_in_memory(), 5);
        }

        #[test]
        fn test_encode_3() {
            let l = L!(r#""aaa\"aaa""#).encode();
            assert_eq!(l.line, r#""\"aaa\\\"aaa\"""#);
            assert_eq!(l.len_in_code(), 16);
            assert_eq!(l.len_in_memory(), 10);
        }

        #[test]
        fn test_encode_4() {
            let l = L!(r#""\x27""#).encode();
            assert_eq!(l.line, r#""\"\\x27\"""#);
            assert_eq!(l.len_in_code(), 11);
            assert_eq!(l.len_in_memory(), 6);
        }
    }
}

pub mod line_len {
    use super::*;

    impl Line {
        pub fn new(line: String) -> Result<Self, LineValidationError> {
            let utf8_len = line.chars().count();
            let line = Self {
                line: line.trim().to_owned(),
                utf8_len,
            };
            line.validate()?;
            Ok(line)
        }

        fn validate(&self) -> Result<(), LineValidationError> {
            if self.utf8_len < 2 {
                return Err(LineValidationError::TooShort);
            }
            if !(self.line.starts_with(QOUTE) && self.line.ends_with(QOUTE)) {
                return Err(LineValidationError::ShouldBeQuoted);
            }
            // TODO: check escape chars here, I'm too tired now =__=
            Ok(())
        }

        pub fn len_in_code(&self) -> usize {
            self.utf8_len
        }

        pub fn len_in_memory(&self) -> usize {
            let inner_line = &self.line[1..self.utf8_len - 1];
            let mut iter = inner_line.chars().peekable();

            let mut len = 0;
            while let Some(c) = iter.next() {
                len += 1;
                if c == ESC {
                    if let Some(next) = iter.peek() {
                        if matches!(*next, ESC | QOUTE) {
                            iter.nth(0);
                        } else if *next == X {
                            iter.nth(2);
                        }
                    }
                }
            }
            len
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod len {
            use super::*;

            #[test]
            fn test_len_empty() {
                let l = L!(r#""""#);
                assert_eq!(l.len_in_code(), 2);
                assert_eq!(l.len_in_memory(), 0);
            }

            #[test]
            fn test_len_simple_1() {
                let l = L!(r#""abc""#);
                assert_eq!(l.len_in_code(), 5);
                assert_eq!(l.len_in_memory(), 3);
            }

            #[test]
            fn test_len_quote_1() {
                let l = L!(r#""aaa\"aaa""#);
                assert_eq!(l.len_in_code(), 10);
                assert_eq!(l.len_in_memory(), 7);
            }

            #[test]
            fn test_len_hex_1() {
                let l = L!(r#""\x27""#);
                assert_eq!(l.len_in_code(), 6);
                assert_eq!(l.len_in_memory(), 1);
            }

            #[test]
            fn test_len_hex_2() {
                let l = L!(r#""a\x27aa""#);
                assert_eq!(l.len_in_code(), 9);
                assert_eq!(l.len_in_memory(), 4);
            }
        }
    }
}

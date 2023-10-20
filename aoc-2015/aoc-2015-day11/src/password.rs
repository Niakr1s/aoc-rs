pub mod predicates;
pub mod security;

#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("empty password")]
    EmptyPassword,
    #[error("invalid character")]
    InvalidChar,
}

impl Password {
    pub fn new(pass: String) -> Result<Self, PasswordError> {
        if pass.is_empty() {
            return Err(PasswordError::EmptyPassword);
        }
        if pass.chars().all(|ch| ch.is_ascii_alphabetic()) {
            Ok(Self(pass))
        } else {
            Err(PasswordError::InvalidChar)
        }
    }

    pub fn next(&self) -> Password {
        let chars = self.0.chars().collect::<Vec<char>>();
        let ret = Self::incr(&chars);
        Password(String::from_iter(ret.into_iter()))
    }

    fn incr(s: &[char]) -> Vec<char> {
        if s.is_empty() {
            return vec![];
        }
        let (left, &[last]) = s.split_at(s.len() - 1) else {
            unreachable!()
        };
        let (mut ret, last) = match last {
            'z' => (Self::incr(left), 'a'),
            'a'..='z' => (
                Vec::from_iter(left.into_iter().copied()),
                ((last as u8) + 1) as char,
            ),
            _ => unreachable!("invalid char"),
        };
        ret.push(last);
        ret
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn next_password_iter(&self) -> Passwords {
        Passwords::new(self)
    }
}

pub struct Passwords {
    next: Password,
}

impl Passwords {
    pub fn new(password: &Password) -> Self {
        Self {
            next: password.next(),
        }
    }
}

impl Iterator for Passwords {
    type Item = Password;

    fn next(&mut self) -> Option<Self::Item> {
        let mut item = self.next.next();
        std::mem::swap(&mut self.next, &mut item);
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod passwords {
        use super::*;

        #[test]
        fn next() {
            let mut iter = Password::new("xx".to_string())
                .unwrap()
                .next_password_iter();
            assert_eq!(iter.next(), Some(Password::new("xy".to_string()).unwrap()));
            assert_eq!(iter.next(), Some(Password::new("xz".to_string()).unwrap()));
            assert_eq!(iter.next(), Some(Password::new("ya".to_string()).unwrap()));
            assert_eq!(iter.next(), Some(Password::new("yb".to_string()).unwrap()));
        }
    }

    mod password {
        use super::*;

        mod next {
            use super::*;

            #[test]
            fn xx() {
                assert_eq!(
                    Password::new("xx".to_string()).unwrap().next().as_str(),
                    "xy"
                )
            }

            #[test]
            fn xy() {
                assert_eq!(
                    Password::new("xy".to_string()).unwrap().next().as_str(),
                    "xz"
                )
            }

            #[test]
            fn xz() {
                assert_eq!(
                    Password::new("xz".to_string()).unwrap().next().as_str(),
                    "ya"
                )
            }

            #[test]
            fn ya() {
                assert_eq!(
                    Password::new("ya".to_string()).unwrap().next().as_str(),
                    "yb"
                )
            }

            #[test]
            fn zzzz() {
                assert_eq!(
                    Password::new("zzzz".to_string()).unwrap().next().as_str(),
                    "aaaa"
                );
            }

            #[test]
            fn abcd() {
                assert_eq!(
                    Password::new("abcd".to_string()).unwrap().next().as_str(),
                    "abce"
                );
            }

            #[test]
            fn xyz() {
                assert_eq!(
                    Password::new("xyz".to_string()).unwrap().next().as_str(),
                    "xza"
                );
            }

            #[test]
            fn xyzz() {
                assert_eq!(
                    Password::new("xyzz".to_string()).unwrap().next().as_str(),
                    "xzaa"
                );
            }
        }
    }
}

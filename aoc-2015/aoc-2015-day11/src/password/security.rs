use super::predicates::{contains_sequences, contains_strait, Order};

pub struct SecurityElf;

impl SecurityElf {
    pub fn is_valid(pass: &str) -> bool {
        contains_strait(pass, 3, &Order::Asc)
            && !pass.contains(['i', 'o', 'l'])
            && contains_sequences(pass, 2, 2, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod security_elf {
        use super::*;

        #[test]
        fn hijklmmn() {
            assert!(!SecurityElf::is_valid("hijklmmn"));
        }

        #[test]
        fn abbceffg() {
            assert!(!SecurityElf::is_valid("abbceffg"));
        }

        #[test]
        fn abbcegjk() {
            assert!(!SecurityElf::is_valid("abbcegjk"));
        }
    }
}

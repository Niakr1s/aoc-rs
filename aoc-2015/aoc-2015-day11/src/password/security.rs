use super::predicates::{contains_sequences, contains_strait, Order};

pub struct SecurityElf;

impl SecurityElf {
    pub fn is_valid(pass: &str) -> bool {
        !pass.contains(['i', 'o', 'l'])
            && contains_strait(pass, 3, &Order::Asc)
            && contains_sequences(pass, 2, 2, false)
    }
}

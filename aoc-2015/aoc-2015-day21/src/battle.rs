pub trait Fighter {
    fn hp(&self) -> u32;
    fn damage(&self) -> u32;
    fn armor(&self) -> u32;
    fn get_hit(&mut self, damage: u32);
}

/// Returns true if f1 won.
pub fn fight_till_death(f1: &mut impl Fighter, f2: &mut impl Fighter) -> bool {
    let mut f1_turn = true;
    while f1.hp() > 0 && f2.hp() > 0 {
        if f1_turn {
            hit(f1, f2);
        } else {
            hit(f2, f1);
        }
        f1_turn = !f1_turn;
    }
    f1.hp() > 0
}

fn hit(attacker: &mut impl Fighter, defender: &mut impl Fighter) {
    let damage = attacker
        .damage()
        .checked_sub(defender.armor())
        .max(Some(1))
        .unwrap();
    defender.get_hit(damage);
}

#[cfg(test)]
mod tests {
    use crate::boss::Boss;

    use super::*;

    #[test]
    fn fight_till_death_works() {
        let mut f1 = Boss {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let mut f2 = Boss {
            hp: 12,
            damage: 7,
            armor: 2,
        };
        assert_eq!(fight_till_death(&mut f1, &mut f2), true);
        assert_eq!(f1.hp(), 2);
        assert_eq!(f2.hp(), 0);
    }
}

use itertools::{iproduct, Itertools};

use crate::{
    items::{Armor, Ring, Weapon},
    player::Equip,
};

pub struct Shop {
    pub weapons: Vec<Weapon>,
    pub armors: Vec<Armor>,
    pub rings: Vec<Ring>,
}

impl Shop {
    pub fn player_equip_combinations(&self) -> impl Iterator<Item = Equip> + '_ {
        let weapons = self.weapons.iter();
        let armors = self.armors.iter().map(|a| Some(a)).chain(vec![None]);
        let rings = self
            .rings
            .iter()
            .map(|r| Some(r))
            .chain(vec![None])
            .combinations(2);
        iproduct!(weapons, armors, rings).map(|(w, a, r)| Equip {
            weapon: w.clone(),
            armor: a.cloned(),
            left_ring: r[0].cloned(),
            right_ring: r[1].cloned(),
        })
    }
}

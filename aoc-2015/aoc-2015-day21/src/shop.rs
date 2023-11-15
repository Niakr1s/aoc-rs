use itertools::iproduct;

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
        let left_rings = self.rings.iter().map(|r| Some(r)).chain(vec![None]);
        let right_rings = left_rings.clone();
        iproduct!(weapons, armors, left_rings, right_rings).map(|(w, a, lr, rr)| Equip {
            weapon: w.clone(),
            armor: a.cloned(),
            left_ring: lr.cloned(),
            right_ring: rr.cloned(),
        })
    }
}

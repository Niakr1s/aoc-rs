use crate::{
    battle::Fighter,
    items::{Armor, Ring, Weapon},
};

#[derive(Debug)]
pub struct Equip {
    pub weapon: Weapon,
    pub armor: Option<Armor>,
    pub left_ring: Option<Ring>,
    pub right_ring: Option<Ring>,
}

impl Equip {
    pub fn total_cost(&self) -> u32 {
        self.weapon.cost
            + self.armor.as_ref().map(|a| a.cost).unwrap_or(0)
            + self.left_ring.as_ref().map(|r| r.cost).unwrap_or(0)
            + self.right_ring.as_ref().map(|r| r.cost).unwrap_or(0)
    }
}

pub struct Player {
    pub hp: u32,
    pub equip: Equip,
}

impl Equip {
    pub fn damage(&self) -> u32 {
        self.weapon.damage
            + self
                .left_ring
                .as_ref()
                .map(|r| r.damage)
                .unwrap_or_default()
            + self
                .right_ring
                .as_ref()
                .map(|r| r.damage)
                .unwrap_or_default()
    }

    pub fn armor(&self) -> u32 {
        self.armor.as_ref().map(|a| a.armor).unwrap_or_default()
            + self.left_ring.as_ref().map(|r| r.armor).unwrap_or_default()
            + self
                .right_ring
                .as_ref()
                .map(|r| r.armor)
                .unwrap_or_default()
    }
}

impl Fighter for Player {
    fn hp(&self) -> u32 {
        self.hp
    }

    fn damage(&self) -> u32 {
        self.equip.damage()
    }

    fn armor(&self) -> u32 {
        self.equip.armor()
    }

    fn get_hit(&mut self, damage: u32) {
        self.hp = self.hp.checked_sub(damage).unwrap_or(0);
    }
}

use crate::battle::Fighter;

#[derive(Debug, Clone)]
pub struct Boss {
    pub hp: u32,
    pub damage: u32,
    pub armor: u32,
}

impl Fighter for Boss {
    fn hp(&self) -> u32 {
        self.hp
    }

    fn damage(&self) -> u32 {
        self.damage
    }

    fn armor(&self) -> u32 {
        self.armor
    }

    fn get_hit(&mut self, damage: u32) {
        self.hp = self.hp.checked_sub(damage).unwrap_or(0);
    }
}

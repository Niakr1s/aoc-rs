#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub cost: u32,
    pub damage: u32,
}

#[derive(Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub cost: u32,
    pub armor: u32,
}

#[derive(Debug, Clone)]
pub struct Ring {
    pub name: String,
    pub cost: u32,
    pub armor: u32,
    pub damage: u32,
}

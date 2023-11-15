use aoc_2015_day21::{
    battle::fight_till_death,
    boss::Boss,
    items::{Armor, Ring, Weapon},
    player::Player,
    shop::Shop,
};

fn main() {
    let shop = make_shop();
    let boss = make_boss();
    let player_hp = 100;

    let min_cost = find_min_cost_win(&shop, player_hp, &boss);
    println!("Part 1: min cost to win: {:?}", min_cost);

    let max_cost = find_max_cost_lose(&shop, player_hp, &boss);
    println!("Part 2: max cost to lose: {:?}", max_cost);
}

fn find_max_cost_lose(shop: &Shop, player_hp: u32, boss: &Boss) -> Option<u32> {
    let mut max_cost: Option<u32> = None;
    for equip in shop.player_equip_combinations() {
        let mut boss = boss.clone();
        let mut player = Player {
            hp: player_hp,
            equip,
        };
        let player_win = fight_till_death(&mut player, &mut boss);
        if !player_win {
            let equip_cost = player.equip.total_cost();
            max_cost = max_cost.map_or(Some(equip_cost), |max_cost| Some(max_cost.max(equip_cost)));
        }
    }
    max_cost
}

fn find_min_cost_win(shop: &Shop, player_hp: u32, boss: &Boss) -> Option<u32> {
    let mut min_cost: Option<u32> = None;
    for equip in shop.player_equip_combinations() {
        let mut boss = boss.clone();
        let mut player = Player {
            hp: player_hp,
            equip,
        };
        let player_win = fight_till_death(&mut player, &mut boss);
        if player_win {
            let equip_cost = player.equip.total_cost();
            min_cost = min_cost.map_or(Some(equip_cost), |min_cost| Some(min_cost.min(equip_cost)));
        }
    }
    min_cost
}

fn make_boss() -> Boss {
    Boss {
        hp: 103,
        damage: 9,
        armor: 2,
    }
}

fn make_shop() -> Shop {
    Shop {
        weapons: vec![
            Weapon {
                name: "Dagger".to_owned(),
                cost: 8,
                damage: 4,
            },
            Weapon {
                name: "Shortsword".to_owned(),
                cost: 10,
                damage: 5,
            },
            Weapon {
                name: "Warhammer".to_owned(),
                cost: 25,
                damage: 6,
            },
            Weapon {
                name: "Longsword".to_owned(),
                cost: 40,
                damage: 7,
            },
            Weapon {
                name: "Greataxe".to_owned(),
                cost: 74,
                damage: 8,
            },
        ],
        armors: vec![
            Armor {
                name: "Leather".to_owned(),
                cost: 13,
                armor: 1,
            },
            Armor {
                name: "Chainmail".to_owned(),
                cost: 31,
                armor: 2,
            },
            Armor {
                name: "Splintmail".to_owned(),
                cost: 53,
                armor: 3,
            },
            Armor {
                name: "Bandedmail".to_owned(),
                cost: 75,
                armor: 4,
            },
            Armor {
                name: "Platemail".to_owned(),
                cost: 102,
                armor: 5,
            },
        ],
        rings: vec![
            Ring {
                name: "Damage +1".to_owned(),
                cost: 25,
                damage: 1,
                armor: 0,
            },
            Ring {
                name: "Damage +2".to_owned(),
                cost: 50,
                damage: 2,
                armor: 0,
            },
            Ring {
                name: "Damage +3".to_owned(),
                cost: 100,
                damage: 3,
                armor: 0,
            },
            Ring {
                name: "Defence +1".to_owned(),
                cost: 20,
                damage: 0,
                armor: 1,
            },
            Ring {
                name: "Defence +2".to_owned(),
                cost: 40,
                damage: 0,
                armor: 2,
            },
            Ring {
                name: "Defence +3".to_owned(),
                cost: 80,
                damage: 0,
                armor: 3,
            },
        ],
    }
}

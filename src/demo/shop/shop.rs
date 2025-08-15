//! Shop system for buying weapons and upgrades

use crate::demo::shop::shop_ui::spawn_shop_ui;
use crate::{
    AppSystems, PausableSystems,
    demo::{
        level::{UpgradeShop, WeaponShop},
        player::Player,
        shooting::Money,
    },
    screens::Screen,
};
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ShopState>();
    app.init_resource::<PlayerUpgrades>();
    app.add_systems(
        Update,
        (
            detect_shop_proximity,
            handle_shop_input,
            update_shop_ui,
            handle_shop_purchases,
            handle_weapon_switching,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component)]
pub struct ShopUI;

#[derive(Component)]
pub struct ShopItemButton {
    pub item_id: ShopItem,
    pub cost: u32,
}

#[derive(Clone, Debug)]
pub enum ShopItem {
    RapidFire,
    Uzi,
    SpreadShot,
    LaserBeam,
    Sniper,
    Bazooka,
    Hammer,
    Sword,

    //The following perks are upgrades in the Upgrades-Shop:
    SpeedBoost,
    CoinMagnet,
    BufferUpgrade, // You can buy +50 per level. Level max is 10.
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Shop {
    #[default]
    None,
    Weapon,
    Upgrade,
}

#[derive(Resource, Default)]
pub struct ShopState {
    pub current_shop: Option<Shop>,
    pub is_near_shop: bool,
}

#[derive(Resource)]
pub struct PlayerUpgrades {
    // Weapons
    pub rapid_fire: bool,
    pub uzi: bool,
    pub spread_shot: bool,
    pub laser_beam: bool,
    pub sniper: bool,
    pub bazooka: bool,
    pub hammer: bool,
    pub sword: bool,

    //Upgrades
    pub speed_boost: u32,
    pub coin_magnet: bool,
    pub buffer_level: u32, // New: tracks buffer upgrade level
    pub current_weapon: WeaponType,
}

impl Default for PlayerUpgrades {
    fn default() -> Self {
        Self {
            rapid_fire: false,
            uzi: false,
            spread_shot: false,
            laser_beam: false,
            sniper: false,
            bazooka: false,
            hammer: false,
            sword: false,
            speed_boost: 0,
            coin_magnet: false,
            buffer_level: 1,
            current_weapon: WeaponType::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect)]
pub enum WeaponType {
    #[default]
    Normal,
    RapidFire,
    Uzi,
    SpreadShot,
    LaserBeam,
    Sniper,
    Bazooka,
    Hammer,
    Sword,
    //FlameThrower
    //BowAndArrow
    //Fists
    //Legs
    //GiantSword
    //UltraHammer
    //UltraSniper //these ultra types of weapons are almost funny big
}

/// Detect when player is near a shop
fn detect_shop_proximity(
    mut collision_events: EventReader<CollisionStarted>,
    mut collision_ended: EventReader<CollisionEnded>,
    player_query: Query<Entity, With<Player>>,
    weapon_shop_query: Query<Entity, With<WeaponShop>>,
    upgrade_shop_query: Query<Entity, With<UpgradeShop>>,
    mut shop_state: ResMut<ShopState>,
) {
    let player_entity = player_query.single().ok();

    // Check for shop entry
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        if let Some(player) = player_entity {
            let shop_entity = if *entity1 == player {
                Some(*entity2)
            } else if *entity2 == player {
                Some(*entity1)
            } else {
                None
            };

            if let Some(shop) = shop_entity {
                if weapon_shop_query.contains(shop) {
                    shop_state.current_shop = Some(Shop::Weapon);
                    shop_state.is_near_shop = true;
                } else if upgrade_shop_query.contains(shop) {
                    shop_state.current_shop = Some(Shop::Upgrade);
                    shop_state.is_near_shop = true;
                }
            }
        }
    }

    // Check for shop exit
    for CollisionEnded(entity1, entity2) in collision_ended.read() {
        if let Some(player) = player_entity {
            let shop_entity = if *entity1 == player {
                Some(*entity2)
            } else if *entity2 == player {
                Some(*entity1)
            } else {
                None
            };

            if let Some(shop) = shop_entity {
                if weapon_shop_query.contains(shop) || upgrade_shop_query.contains(shop) {
                    shop_state.current_shop = None;
                    shop_state.is_near_shop = false;
                }
            }
        }
    }
}

/// Handle shop input and UI spawning
fn handle_shop_input(
    shop_state: Res<ShopState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    existing_ui_query: Query<Entity, With<ShopUI>>,
    upgrades: Res<PlayerUpgrades>,
) {
    if shop_state.is_near_shop && keys.just_pressed(KeyCode::KeyE) {
        // Toggle shop UI
        if existing_ui_query.is_empty() {
            spawn_shop_ui(commands, shop_state);
        } else {
            // Close existing UI
            for entity in &existing_ui_query {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Close shop UI when leaving shop area
fn update_shop_ui(
    shop_state: Res<ShopState>,
    mut commands: Commands,
    ui_query: Query<Entity, With<ShopUI>>,
) {
    // Only close UI when leaving shop area
    if !shop_state.is_near_shop && !ui_query.is_empty() {
        for entity in &ui_query {
            commands.entity(entity).despawn();
        }
    }
}

/// Handle shop purchases with number keys
fn handle_shop_purchases(
    shop_state: Res<ShopState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    mut upgrades: ResMut<PlayerUpgrades>,
) {
    if !shop_state.is_near_shop {
        return;
    }

    let purchase_key = if keys.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keys.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else if keys.just_pressed(KeyCode::Digit5) {
        Some(4)
    } else if keys.just_pressed(KeyCode::Digit6) {
        Some(5)
    } else if keys.just_pressed(KeyCode::Digit7) {
        Some(6)
    } else {
        None
    };

    if let Some(item_index) = purchase_key {
        match shop_state.current_shop {
            Some(Shop::Weapon) => {
                let weapons = [
                    (ShopItem::RapidFire, 500),
                    (ShopItem::SpreadShot, 750),
                    (ShopItem::LaserBeam, 1000),
                    (ShopItem::Sniper, 2000),
                    (ShopItem::Hammer, 3000),
                    (ShopItem::Sword, 4000),
                    (ShopItem::Bazooka, 5000),
                ];

                if let Some((item, cost)) = weapons.get(item_index) {
                    if money.amount >= *cost {
                        let can_buy = match item {
                            ShopItem::RapidFire => !upgrades.rapid_fire,
                            ShopItem::SpreadShot => !upgrades.spread_shot,
                            ShopItem::LaserBeam => !upgrades.laser_beam,
                            ShopItem::Sniper => !upgrades.sniper,
                            ShopItem::Hammer => !upgrades.hammer,
                            ShopItem::Sword => !upgrades.sword,
                            ShopItem::Bazooka => !upgrades.bazooka,
                            _ => false,
                        };

                        if can_buy {
                            money.amount -= cost;
                            match item {
                                ShopItem::RapidFire => {
                                    upgrades.rapid_fire = true;
                                }
                                ShopItem::SpreadShot => {
                                    upgrades.spread_shot = true;
                                }
                                ShopItem::LaserBeam => {
                                    upgrades.laser_beam = true;
                                }
                                ShopItem::Sniper => {
                                    upgrades.sniper = true;
                                }
                                ShopItem::Hammer => {
                                    upgrades.hammer = true;
                                }
                                ShopItem::Sword => {
                                    upgrades.sword = true;
                                }
                                ShopItem::Bazooka => {
                                    upgrades.bazooka = true;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Some(Shop::Upgrade) => {
                let upgrades_list = [
                    (ShopItem::SpeedBoost, 300),
                    (ShopItem::CoinMagnet, 600),
                    (ShopItem::BufferUpgrade, 400),
                ];

                if let Some((item, cost)) = upgrades_list.get(item_index) {
                    // Calculate dynamic cost for upgradeable items
                    let actual_cost = match item {
                        ShopItem::BufferUpgrade => *cost + (upgrades.buffer_level * 200), // Cost increases with each level
                        _ => *cost,
                    };

                    if money.amount >= actual_cost {
                        let can_buy = match item {
                            ShopItem::SpeedBoost => upgrades.speed_boost < 3, // Max 3 levels
                            ShopItem::CoinMagnet => !upgrades.coin_magnet,
                            ShopItem::BufferUpgrade => upgrades.buffer_level < 10, // Max 10 levels
                            _ => false,
                        };

                        if can_buy {
                            money.amount -= actual_cost;
                            match item {
                                ShopItem::SpeedBoost => {
                                    upgrades.speed_boost += 1;
                                }
                                ShopItem::CoinMagnet => {
                                    upgrades.coin_magnet = true;
                                }
                                ShopItem::BufferUpgrade => {
                                    upgrades.buffer_level += 1;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Some(Shop::None) => {
                // No specific shop selected, maybe show main shop menu
            }
            None => {}
        }
    }
}

/// Handle weapon switching with Q/Tab keys
fn handle_weapon_switching(keys: Res<ButtonInput<KeyCode>>, mut upgrades: ResMut<PlayerUpgrades>) {
    if keys.just_pressed(KeyCode::KeyQ) || keys.just_pressed(KeyCode::Tab) {
        // Get available weapons
        let mut available_weapons = vec![WeaponType::Normal];

        if upgrades.rapid_fire {
            available_weapons.push(WeaponType::RapidFire);
        }
        if upgrades.uzi {
            available_weapons.push(WeaponType::Uzi);
        }
        if upgrades.spread_shot {
            available_weapons.push(WeaponType::SpreadShot);
        }
        if upgrades.laser_beam {
            available_weapons.push(WeaponType::LaserBeam);
        }
        if upgrades.sniper {
            available_weapons.push(WeaponType::Sniper);
        }
        if upgrades.bazooka {
            available_weapons.push(WeaponType::Bazooka);
        }
        if upgrades.hammer {
            available_weapons.push(WeaponType::Hammer);
        }
        if upgrades.sword {
            available_weapons.push(WeaponType::Sword);
        }

        // Find current weapon index and switch to next
        if let Some(current_index) = available_weapons
            .iter()
            .position(|&w| w == upgrades.current_weapon)
        {
            let next_index = (current_index + 1) % available_weapons.len();
            upgrades.current_weapon = available_weapons[next_index];
        }
    }
}

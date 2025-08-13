//! Shop system for buying weapons and upgrades

use bevy::prelude::*;
use avian2d::prelude::*;
use crate::{AppSystems, PausableSystems, screens::Screen, demo::{player::Player, level::{WeaponShop, UpgradeShop}, shooting::Money}};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ShopState>();
    app.init_resource::<PlayerUpgrades>();
    app.add_systems(Update, (
        detect_shop_proximity,
        handle_shop_input,
        update_shop_ui,
        handle_shop_purchases,
        handle_weapon_switching,
    ).in_set(AppSystems::Update).in_set(PausableSystems));
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
    //Upgrades
    SpeedBoost,
    CoinMagnet,
    BufferUpgrade,
}

#[derive(Clone, Debug)]
pub enum ShopType {
    Weapon,
    Upgrade,
}

#[derive(Resource, Default)]
pub struct ShopState {
    pub current_shop: Option<ShopType>,
    pub is_near_shop: bool,
}

#[derive(Resource, Default)]
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
    pub buffer_level: u32,  // New: tracks buffer upgrade level
    pub current_weapon: WeaponType,
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
                    shop_state.current_shop = Some(ShopType::Weapon);
                    shop_state.is_near_shop = true;
                } else if upgrade_shop_query.contains(shop) {
                    shop_state.current_shop = Some(ShopType::Upgrade);
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
            spawn_shop_ui(&mut commands, &shop_state, &upgrades);
        } else {
            // Close existing UI
            for entity in &existing_ui_query {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Spawn shop UI
fn spawn_shop_ui(commands: &mut Commands, shop_state: &ShopState, upgrades: &PlayerUpgrades) {
    let shop_title = match shop_state.current_shop {
        Some(ShopType::Weapon) => "WEAPON SHOP",
        Some(ShopType::Upgrade) => "UPGRADE SHOP",
        None => "SHOP",
    };

    // Create shop UI background panel
    commands.spawn((
        Name::new("Shop UI"),
        ShopUI,
        Sprite::from_color(Color::srgba(0.1, 0.1, 0.1, 0.9), Vec2::new(300.0, 450.0)),
        Transform::from_translation(Vec3::new(-200.0, 0.0, 100.0)), // Left side of screen
        StateScoped(Screen::Gameplay),
    )).with_children(|parent| {
        // Shop title
        parent.spawn((
            Text2d::new(shop_title),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(0.0, 175.0, 1.0)),
        ));

        // Instructions
        parent.spawn((
            Text2d::new("Press E to close"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
            Transform::from_translation(Vec3::new(0.0, 145.0, 1.0)),
        ));

        // Instructions for purchasing
        parent.spawn((
            Text2d::new("Press 1/2/3 to buy"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
            Transform::from_translation(Vec3::new(0.0, 115.0, 1.0)),
        ));

        // Shop items based on type
        match shop_state.current_shop {
            Some(ShopType::Weapon) => {
                let weapons = [
                    ("1. Rapid Fire", 500, ShopItem::RapidFire),
                    ("2. Spread Shot", 750, ShopItem::SpreadShot),
                    ("3. Laser Beam", 1000, ShopItem::LaserBeam),
                    ("4. Sniper", 2000, ShopItem::Sniper),
                    ("5. Hammer", 3000, ShopItem::Hammer),
                    ("6. Sword", 4000, ShopItem::Sword),
                    ("7. Bazooka", 5000, ShopItem::Bazooka),
                    //...
                ];

                for (i, (name, cost, item_id)) in weapons.iter().enumerate() {
                    let y_pos = 50.0 - (i as f32 * 60.0);

                    // Item background
                    parent.spawn((
                        Name::new(format!("Shop Item {}", name)),
                        ShopItemButton { item_id: item_id.clone(), cost: *cost },
                        Sprite::from_color(Color::srgb(0.2, 0.2, 0.3), Vec2::new(250.0, 50.0)),
                        Transform::from_translation(Vec3::new(0.0, y_pos, 1.0)),
                    )).with_children(|item_parent| {
                        // Item name
                        item_parent.spawn((
                            Text2d::new(*name),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            Transform::from_translation(Vec3::new(-60.0, 5.0, 1.0)),
                        ));

                        // Item cost
                        item_parent.spawn((
                            Text2d::new(format!("${}", cost)),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.8, 0.0)),
                            Transform::from_translation(Vec3::new(60.0, 5.0, 1.0)),
                        ));
                    });
                }
            },
            Some(ShopType::Upgrade) => {
                let upgrade_items = [
                    ("1. Speed Boost", 300, ShopItem::SpeedBoost, upgrades.speed_boost, 3),
                    ("2. Coin Magnet", 600, ShopItem::CoinMagnet, if upgrades.coin_magnet { 1 } else { 0 }, 1),
                    ("3. Buffer Upgrade", 400, ShopItem::BufferUpgrade, upgrades.buffer_level, 5),
                ];

                for (i, (name, base_cost, item_id, current_level, max_level)) in upgrade_items.iter().enumerate() {
                    let y_pos = 75.0 - (i as f32 * 70.0);

                    // Calculate dynamic cost
                    let actual_cost = match item_id {
                        ShopItem::BufferUpgrade => base_cost + (current_level * 200),
                        _ => *base_cost,
                    };

                    // Determine if maxed out or available
                    let is_maxed = *current_level >= *max_level;
                    let item_color = if is_maxed { Color::srgb(0.4, 0.4, 0.4) } else { Color::srgb(0.2, 0.3, 0.2) };

                    // Item background
                    parent.spawn((
                        Name::new(format!("Shop Item {}", name)),
                        ShopItemButton { item_id: item_id.clone(), cost: actual_cost },
                        Sprite::from_color(item_color, Vec2::new(250.0, 60.0)),
                        Transform::from_translation(Vec3::new(0.0, y_pos, 1.0)),
                    )).with_children(|item_parent| {
                        // Item name with level info
                        let display_name = if *max_level > 1 {
                            format!("{} ({}/{})", name, current_level, max_level)
                        } else if is_maxed {
                            format!("{} (Owned)", name)
                        } else {
                            name.to_string()
                        };

                        item_parent.spawn((
                            Text2d::new(display_name),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(if is_maxed { Color::srgb(0.6, 0.6, 0.6) } else { Color::WHITE }),
                            Transform::from_translation(Vec3::new(-60.0, 10.0, 1.0)),
                        ));

                        // Item cost (or "MAXED" if fully upgraded)
                        let cost_text = if is_maxed {
                            "MAXED".to_string()
                        } else {
                            format!("${}", actual_cost)
                        };

                        item_parent.spawn((
                            Text2d::new(cost_text),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(if is_maxed { Color::srgb(0.6, 0.6, 0.6) } else { Color::srgb(1.0, 0.8, 0.0) }),
                            Transform::from_translation(Vec3::new(60.0, 10.0, 1.0)),
                        ));

                        // Effect description
                        let effect_text = match item_id {
                            ShopItem::BufferUpgrade => {
                                let current_capacity = 20 + (current_level * 10);
                                let next_capacity = 20 + ((current_level + 1) * 10);
                                if is_maxed {
                                    format!("Buffer: {} coins", current_capacity)
                                } else {
                                    format!("{} -> {} coins", current_capacity, next_capacity)
                                }
                            },
                            ShopItem::SpeedBoost => format!("Speed +{}%", current_level * 25),
                            ShopItem::CoinMagnet => "Attracts coins".to_string(),
                            _ => "".to_string(),
                        };

                        item_parent.spawn((
                            Text2d::new(effect_text),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                            Transform::from_translation(Vec3::new(0.0, -10.0, 1.0)),
                        ));
                    });
                }
            },
            None => {},
        }
    });
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
            Some(ShopType::Weapon) => {
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
                                },
                                ShopItem::SpreadShot => {
                                    upgrades.spread_shot = true;
                                },
                                ShopItem::LaserBeam => {
                                    upgrades.laser_beam = true;
                                },
                                ShopItem::Sniper => {
                                    upgrades.sniper = true;
                                },
                                ShopItem::Hammer => {
                                    upgrades.hammer = true;
                                },
                                ShopItem::Sword => {
                                    upgrades.sword = true;
                                },
                                ShopItem::Bazooka => {
                                    upgrades.bazooka = true;
                                },
                                _ => {},
                            }
                        }
                    }
                }
            },
            Some(ShopType::Upgrade) => {
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
                            ShopItem::BufferUpgrade => upgrades.buffer_level < 5, // Max 5 levels
                            _ => false,
                        };

                        if can_buy {
                            money.amount -= actual_cost;
                            match item {
                                ShopItem::SpeedBoost => {
                                    upgrades.speed_boost += 1;
                                },
                                ShopItem::CoinMagnet => {
                                    upgrades.coin_magnet = true;
                                },
                                ShopItem::BufferUpgrade => {
                                    upgrades.buffer_level += 1;
                                },
                                _ => {},
                            }
                        }
                    }
                }
            },
            None => {},
        }
    }
}

/// Handle weapon switching with Q/Tab keys
fn handle_weapon_switching(
    keys: Res<ButtonInput<KeyCode>>,
    mut upgrades: ResMut<PlayerUpgrades>,
) {
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
        if let Some(current_index) = available_weapons.iter().position(|&w| w == upgrades.current_weapon) {
            let next_index = (current_index + 1) % available_weapons.len();
            upgrades.current_weapon = available_weapons[next_index];

        }
    }
}

//! Shop system for buying weapons and upgrades

use crate::{
    AppSystems, PausableSystems,
    demo::{
        level::{UpgradeShop, WeaponShop},
        player::{Money, Player},
    },
};
use avian2d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::demo::shop::shop_ui::spawn_shop_ui;
use crate::demo::shop::shop_ui::update_shop_ui;
pub const ITEM_CONFIG_PATH: &str = "assets/configurations/";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeaponData {
    pub cost: u32,
    pub damage: i32,
    pub weapon_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpgradeData {
    pub cost: u32,
    pub upgrade_type: String,
    #[serde(default)]
    pub player_speed_multiplier: Option<f32>,
    #[serde(default)]
    pub buffer: Option<HashMap<String, BufferLevel>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BufferLevel {
    pub buffer_amount: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeaponsConfig {
    pub types: HashMap<String, WeaponData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpgradesConfig {
    pub types: HashMap<String, UpgradeData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemsConfig {
    pub weapons: WeaponsConfig,
    pub upgrades: UpgradesConfig,
}

#[derive(Resource)]
pub struct ItemsData {
    pub config: ItemsConfig,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerUpgrades>();
    app.init_resource::<ShopState>();
    app.init_resource::<PlayerUpgrades>();
    app.add_systems(Startup, load_items_config);
    app.add_systems(
        Update,
        (
            handle_player_shop_collisions,
            handle_shop_input,
            update_shop_ui,
            handle_weapon_switching,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

fn load_items_config(mut commands: Commands) {
    let file_path = "assets/configurations/items.ron";

    match std::fs::read_to_string(file_path) {
        Ok(content) => match ron::from_str::<ItemsConfig>(&content) {
            Ok(config) => {
                commands.insert_resource(ItemsData { config });
                info!("Successfully loaded items configuration");
            }
            Err(e) => {
                error!("Failed to parse items.ron: {}", e);
            }
        },
        Err(e) => {
            error!("Failed to read items.ron file: {}", e);
        }
    }
}

/// Handle sensor collisions between player and shops using CollisionStarted/Ended events
fn handle_player_shop_collisions(
    mut collision_started: EventReader<CollisionStarted>,
    mut collision_ended: EventReader<CollisionEnded>,
    mut shop_state: ResMut<ShopState>,
    player_query: Query<Entity, With<Player>>,
    weapon_shop_query: Query<Entity, With<WeaponShop>>,
    upgrade_shop_query: Query<Entity, With<UpgradeShop>>,
) {
    // Handle collision started events (player enters shop sensor)
    for CollisionStarted(entity1, entity2) in collision_started.read() {
        let (player_entity, shop_entity) = if player_query.contains(*entity1) {
            (Some(*entity1), *entity2)
        } else if player_query.contains(*entity2) {
            (Some(*entity2), *entity1)
        } else {
            continue; // Neither entity is the player
        };

        if player_entity.is_some() {
            // Check which type of shop the player collided with
            if weapon_shop_query.contains(shop_entity) {
                shop_state.is_near_shop = true;
                shop_state.current_shop = Some(Shop::Weapon);
                info!("Player entered weapon shop area");
            } else if upgrade_shop_query.contains(shop_entity) {
                shop_state.is_near_shop = true;
                shop_state.current_shop = Some(Shop::Upgrade);
                info!("Player entered upgrade shop area");
            }
        }
    }

    // Handle collision ended events (player leaves shop sensor)
    for CollisionEnded(entity1, entity2) in collision_ended.read() {
        let (player_entity, shop_entity) = if player_query.contains(*entity1) {
            (Some(*entity1), *entity2)
        } else if player_query.contains(*entity2) {
            (Some(*entity2), *entity1)
        } else {
            continue; // Neither entity is the player
        };

        if player_entity.is_some() {
            // Check if the player left a shop area
            if weapon_shop_query.contains(shop_entity) || upgrade_shop_query.contains(shop_entity) {
                shop_state.is_near_shop = false;
                shop_state.current_shop = None;
                info!("Player left shop area");
            }
        }
    }
}

#[derive(Component)]
pub struct ShopUI;

#[derive(Component)]
pub struct ShopItemButton {
    pub item_name: String,
    pub item_type: ItemType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShopItem {
    pub name: &'static str,
    pub item_type: ItemType,
    pub cost: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Weapon(WeaponType),
    Upgrade(UpgradeType),
}

impl ShopItem {
    pub fn from_name(name: &str) -> ShopItem {
        match name {
            "Rapid Fire" => ShopItem {
                name: "Rapid Fire",
                item_type: ItemType::Weapon(WeaponType::RapidFire),
                cost: 500,
            },
            "Uzi" => ShopItem {
                name: "Uzi",
                item_type: ItemType::Weapon(WeaponType::Uzi),
                cost: 400,
            },
            "Spread Shot" => ShopItem {
                name: "Spread Shot",
                item_type: ItemType::Weapon(WeaponType::SpreadShot),
                cost: 750,
            },
            "Laser Beam" => ShopItem {
                name: "Laser Beam",
                item_type: ItemType::Weapon(WeaponType::LaserBeam),
                cost: 1000,
            },
            "Sniper" => ShopItem {
                name: "Sniper",
                item_type: ItemType::Weapon(WeaponType::Sniper),
                cost: 2000,
            },
            "Bazooka" => ShopItem {
                name: "Bazooka",
                item_type: ItemType::Weapon(WeaponType::Bazooka),
                cost: 5000,
            },
            "Hammer" => ShopItem {
                name: "Hammer",
                item_type: ItemType::Weapon(WeaponType::Hammer),
                cost: 3000,
            },
            "Sword" => ShopItem {
                name: "Sword",
                item_type: ItemType::Weapon(WeaponType::Sword),
                cost: 4000,
            },
            "Speed Boost" => ShopItem {
                name: "Speed Boost",
                item_type: ItemType::Upgrade(UpgradeType::SpeedBoost),
                cost: 300,
            },
            "Coin Magnet" => ShopItem {
                name: "Coin Magnet",
                item_type: ItemType::Upgrade(UpgradeType::CoinMagnet),
                cost: 600,
            },
            "Buffer Upgrade" => ShopItem {
                name: "Buffer Upgrade",
                item_type: ItemType::Upgrade(UpgradeType::BufferUpgrade),
                cost: 400,
            },
            _ => ShopItem {
                name: "Rapid Fire",
                item_type: ItemType::Weapon(WeaponType::RapidFire),
                cost: 500,
            },
        }
    }

    pub fn is_weapon(&self) -> bool {
        matches!(self.item_type, ItemType::Weapon(_))
    }

    pub fn is_upgrade(&self) -> bool {
        matches!(self.item_type, ItemType::Upgrade(_))
    }
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

#[derive(Resource, Reflect)]
#[reflect(Resource)]
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
    pub buffer_level: u32,
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

impl PlayerUpgrades {
    pub fn names() -> Vec<&'static str> {
        vec![
            "Rapid Fire",
            "Uzi",
            "Spread Shot",
            "Laser Beam",
            "Sniper",
            "Bazooka",
            "Hammer",
            "Sword",
            "Speed Boost",
            "Coin Magnet",
            "Buffer Upgrade",
        ]
    }

    pub fn weapon_names() -> Vec<&'static str> {
        vec![
            "Rapid Fire",
            "Uzi",
            "Spread Shot",
            "Laser Beam",
            "Sniper",
            "Bazooka",
            "Hammer",
            "Sword",
        ]
    }

    pub fn upgrade_names() -> Vec<&'static str> {
        vec!["Speed Boost", "Coin Magnet", "Buffer Upgrade"]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect, Serialize, Deserialize)]
pub enum UpgradeType {
    #[default]
    Normal,
    SpeedBoost,
    CoinMagnet,
    BufferUpgrade,
}

impl WeaponType {
    pub fn from_string(s: &str) -> Self {
        match s {
            "RapidFire" => WeaponType::RapidFire,
            "Uzi" => WeaponType::Uzi,
            "SpreadShot" => WeaponType::SpreadShot,
            "LaserBeam" => WeaponType::LaserBeam,
            "Sniper" => WeaponType::Sniper,
            "Bazooka" => WeaponType::Bazooka,
            "Hammer" => WeaponType::Hammer,
            "Sword" => WeaponType::Sword,
            _ => WeaponType::Normal,
        }
    }
}

impl UpgradeType {
    pub fn from_string(s: &str) -> Self {
        match s {
            "SpeedBoost" => UpgradeType::SpeedBoost,
            "CoinMagnet" => UpgradeType::CoinMagnet,
            "BufferUpgrade" => UpgradeType::BufferUpgrade,
            _ => UpgradeType::Normal,
        }
    }
}

/// Handle shop input and UI spawning
fn handle_shop_input(
    shop_state: Res<ShopState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    existing_ui_query: Query<Entity, With<ShopUI>>,
    items_data: Option<Res<ItemsData>>,
) {
    if keys.just_pressed(KeyCode::KeyE) {
        if shop_state.is_near_shop {
            // Toggle shop UI
            if existing_ui_query.is_empty() {
                spawn_shop_ui(commands, shop_state, items_data);
            } else {
                // Close existing UI
                for entity in &existing_ui_query {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

pub fn buy_item(
    trigger: Trigger<Pointer<Click>>,
    items_data: Option<Res<ItemsData>>,
    mut money: ResMut<Money>,
    mut upgrades: ResMut<PlayerUpgrades>,
    button_query: Query<(Entity, &ShopItemButton)>,
) {
    let Some(items_data) = items_data else {
        warn!("Items data not loaded yet");
        return;
    };

    // Try to get the ShopItemButton component from the triggered entity
    let Ok((_, button)) = button_query.get(trigger.target()) else {
        warn!(
            "Could not find ShopItemButton component on clicked entity: {:?}",
            trigger.target()
        );
        return;
    };

    let (cost, can_buy) = match &button.item_type {
        ItemType::Weapon(weapon_type) => {
            if let Some(weapon_data) = items_data.config.weapons.types.get(&button.item_name) {
                let can_buy = match weapon_type {
                    WeaponType::RapidFire => !upgrades.rapid_fire,
                    WeaponType::Uzi => !upgrades.uzi,
                    WeaponType::SpreadShot => !upgrades.spread_shot,
                    WeaponType::LaserBeam => !upgrades.laser_beam,
                    WeaponType::Sniper => !upgrades.sniper,
                    WeaponType::Bazooka => !upgrades.bazooka,
                    WeaponType::Hammer => !upgrades.hammer,
                    WeaponType::Sword => !upgrades.sword,
                    _ => false,
                };
                (weapon_data.cost, can_buy)
            } else {
                warn!("Weapon {} not found in config", button.item_name);
                return;
            }
        }
        ItemType::Upgrade(upgrade_type) => {
            if let Some(upgrade_data) = items_data.config.upgrades.types.get(&button.item_name) {
                let (cost, can_buy) = match upgrade_type {
                    UpgradeType::SpeedBoost => (upgrade_data.cost, upgrades.speed_boost < 3),
                    UpgradeType::CoinMagnet => (upgrade_data.cost, !upgrades.coin_magnet),
                    UpgradeType::BufferUpgrade => {
                        let additional_cost = (upgrades.buffer_level.saturating_sub(1)) * 200;
                        let total_cost = upgrade_data.cost + additional_cost;
                        (total_cost, upgrades.buffer_level < 10)
                    }
                    _ => (upgrade_data.cost, false),
                };
                (cost, can_buy)
            } else {
                warn!("Upgrade {} not found in config", button.item_name);
                return;
            }
        }
    };

    if !can_buy {
        warn!("Cannot buy item: already owned or at max level");
        return;
    }

    if money.amount < cost {
        warn!(
            "Not enough money to buy {} (need {}, have {})",
            button.item_name, cost, money.amount
        );
        return;
    }

    money.amount -= cost;

    match &button.item_type {
        ItemType::Weapon(weapon_type) => match weapon_type {
            WeaponType::RapidFire => upgrades.rapid_fire = true,
            WeaponType::Uzi => upgrades.uzi = true,
            WeaponType::SpreadShot => upgrades.spread_shot = true,
            WeaponType::LaserBeam => upgrades.laser_beam = true,
            WeaponType::Sniper => upgrades.sniper = true,
            WeaponType::Bazooka => upgrades.bazooka = true,
            WeaponType::Hammer => upgrades.hammer = true,
            WeaponType::Sword => upgrades.sword = true,
            _ => {}
        },
        ItemType::Upgrade(upgrade_type) => match upgrade_type {
            UpgradeType::SpeedBoost => upgrades.speed_boost += 1,
            UpgradeType::CoinMagnet => upgrades.coin_magnet = true,
            UpgradeType::BufferUpgrade => upgrades.buffer_level += 1,
            _ => {}
        },
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

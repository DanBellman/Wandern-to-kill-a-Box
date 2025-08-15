use crate::demo::shop::shop::ShopUI;
/// Spawn shop UI
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
use bevy::{ecs::system::IntoObserverSystem, prelude::*};

use super::{
    shop::{PlayerUpgrades, Shop, ShopItem, ShopItemButton, ShopState},
    //shop_ui::ShopUI,
};

use crate::{menus::Menu, theme::widget};

// pub fn spawn_shop_ui(
//     commands: &mut Commands,
//     shop_state: &ShopState,
//     upgrades: &PlayerUpgrades
// ) {
//     let shop_title = match shop_state.current_shop {
//         Some(ShopType::Weapon) => "WEAPON SHOP",
//         Some(ShopType::Upgrade) => "UPGRADE SHOP",
//         None => "SHOP",
//     };

//     // Create shop UI background panel
//     commands
//         .spawn((
//             Name::new("Shop UI"),
//             ShopUI,
//             Sprite::from_color(Color::srgba(0.1, 0.1, 0.1, 0.9), Vec2::new(300.0, 450.0)),
//             Transform::from_translation(Vec3::new(-200.0, 0.0, 100.0)), // Left side of screen
//             StateScoped(Screen::Gameplay),
//         ))
//         .with_children(|parent| {
//             // Shop title
//             parent.spawn((
//                 Text2d::new(shop_title),
//                 TextFont {
//                     font_size: 24.0,
//                     ..default()
//                 },
//                 TextColor(Color::WHITE),
//                 Transform::from_translation(Vec3::new(0.0, 175.0, 1.0)),
//             ));

//             // Instructions
//             parent.spawn((
//                 Text2d::new("Press E to close"),
//                 TextFont {
//                     font_size: 16.0,
//                     ..default()
//                 },
//                 TextColor(Color::srgb(0.7, 0.7, 0.7)),
//                 Transform::from_translation(Vec3::new(0.0, 145.0, 1.0)),
//             ));

//             // Instructions for purchasing
//             parent.spawn((
//                 Text2d::new("Press 1/2/3 to buy"),
//                 TextFont {
//                     font_size: 14.0,
//                     ..default()
//                 },
//                 TextColor(Color::srgb(0.6, 0.6, 0.6)),
//                 Transform::from_translation(Vec3::new(0.0, 115.0, 1.0)),
//             ));

//             // Shop items based on type
//             match shop_state.current_shop {
//                 Some(ShopType::Weapon) => {
//                     let weapons = [
//                         ("1. Rapid Fire", 500, ShopItem::RapidFire),
//                         ("2. Spread Shot", 750, ShopItem::SpreadShot),
//                         ("3. Laser Beam", 1000, ShopItem::LaserBeam),
//                         ("4. Sniper", 2000, ShopItem::Sniper),
//                         ("5. Hammer", 3000, ShopItem::Hammer),
//                         ("6. Sword", 4000, ShopItem::Sword),
//                         ("7. Bazooka", 5000, ShopItem::Bazooka),
//                         //...
//                     ];

//                     for (i, (name, cost, item_id)) in weapons.iter().enumerate() {
//                         let y_pos = 50.0 - (i as f32 * 60.0);

//                         // Item background
//                         parent
//                             .spawn((
//                                 Name::new(format!("Shop Item {}", name)),
//                                 ShopItemButton {
//                                     item_id: item_id.clone(),
//                                     cost: *cost,
//                                 },
//                                 Sprite::from_color(
//                                     Color::srgb(0.2, 0.2, 0.3),
//                                     Vec2::new(250.0, 50.0),
//                                 ),
//                                 Transform::from_translation(Vec3::new(0.0, y_pos, 1.0)),
//                             ))
//                             .with_children(|item_parent| {
//                                 // Item name
//                                 item_parent.spawn((
//                                     Text2d::new(*name),
//                                     TextFont {
//                                         font_size: 18.0,
//                                         ..default()
//                                     },
//                                     TextColor(Color::WHITE),
//                                     Transform::from_translation(Vec3::new(-60.0, 5.0, 1.0)),
//                                 ));

//                                 // Item cost
//                                 item_parent.spawn((
//                                     Text2d::new(format!("${}", cost)),
//                                     TextFont {
//                                         font_size: 16.0,
//                                         ..default()
//                                     },
//                                     TextColor(Color::srgb(1.0, 0.8, 0.0)),
//                                     Transform::from_translation(Vec3::new(60.0, 5.0, 1.0)),
//                                 ));
//                             });
//                     }
//                 }
//                 Some(ShopType::Upgrade) => {
//                     let upgrade_items = [
//                         (
//                             "1. Speed Boost",
//                             300,
//                             ShopItem::SpeedBoost,
//                             upgrades.speed_boost,
//                             3,
//                         ),
//                         (
//                             "2. Coin Magnet",
//                             600,
//                             ShopItem::CoinMagnet,
//                             if upgrades.coin_magnet { 1 } else { 0 },
//                             1,
//                         ),
//                         (
//                             "3. Buffer Upgrade",
//                             400,
//                             ShopItem::BufferUpgrade,
//                             upgrades.buffer_level,
//                             10,
//                         ),
//                     ];

//                     for (i, (name, base_cost, item_id, current_level, max_level)) in
//                         upgrade_items.iter().enumerate()
//                     {
//                         let y_pos = 75.0 - (i as f32 * 70.0);

//                         // Calculate dynamic cost
//                         let actual_cost = match item_id {
//                             ShopItem::BufferUpgrade => base_cost + (current_level * 200),
//                             _ => *base_cost,
//                         };

//                         // Determine if maxed out or available
//                         let is_maxed = *current_level >= *max_level;
//                         let item_color = if is_maxed {
//                             Color::srgb(0.4, 0.4, 0.4)
//                         } else {
//                             Color::srgb(0.2, 0.3, 0.2)
//                         };

//                         // Item background
//                         parent
//                             .spawn((
//                                 Name::new(format!("Shop Item {}", name)),
//                                 ShopItemButton {
//                                     item_id: item_id.clone(),
//                                     cost: actual_cost,
//                                 },
//                                 Sprite::from_color(item_color, Vec2::new(250.0, 60.0)),
//                                 Transform::from_translation(Vec3::new(0.0, y_pos, 1.0)),
//                             ))
//                             .with_children(|item_parent| {
//                                 // Item name with level info
//                                 let display_name = if *max_level > 1 {
//                                     format!("{} ({}/{})", name, current_level, max_level)
//                                 } else if is_maxed {
//                                     format!("{} (Owned)", name)
//                                 } else {
//                                     name.to_string()
//                                 };

//                                 item_parent.spawn((
//                                     Text2d::new(display_name),
//                                     TextFont {
//                                         font_size: 16.0,
//                                         ..default()
//                                     },
//                                     TextColor(if is_maxed {
//                                         Color::srgb(0.6, 0.6, 0.6)
//                                     } else {
//                                         Color::WHITE
//                                     }),
//                                     Transform::from_translation(Vec3::new(-60.0, 10.0, 1.0)),
//                                 ));

//                                 // Item cost (or "MAXED" if fully upgraded)
//                                 let cost_text = if is_maxed {
//                                     "MAXED".to_string()
//                                 } else {
//                                     format!("${}", actual_cost)
//                                 };

//                                 item_parent.spawn((
//                                     Text2d::new(cost_text),
//                                     TextFont {
//                                         font_size: 14.0,
//                                         ..default()
//                                     },
//                                     TextColor(if is_maxed {
//                                         Color::srgb(0.6, 0.6, 0.6)
//                                     } else {
//                                         Color::srgb(1.0, 0.8, 0.0)
//                                     }),
//                                     Transform::from_translation(Vec3::new(60.0, 10.0, 1.0)),
//                                 ));

//                                 // Effect description
//                                 let effect_text = match item_id {
//                                     ShopItem::BufferUpgrade => {
//                                         let current_capacity = 20 + (current_level * 50);
//                                         let next_capacity = 20 + ((current_level + 1) * 50);
//                                         if is_maxed {
//                                             format!("Buffer: {} coins", current_capacity)
//                                         } else {
//                                             format!(
//                                                 "{} -> {} coins",
//                                                 current_capacity, next_capacity
//                                             )
//                                         }
//                                     }
//                                     ShopItem::SpeedBoost => {
//                                         format!("Speed +{}%", current_level * 25)
//                                     }
//                                     ShopItem::CoinMagnet => "Attracts coins".to_string(),
//                                     _ => "".to_string(),
//                                 };

//                                 item_parent.spawn((
//                                     Text2d::new(effect_text),
//                                     TextFont {
//                                         font_size: 12.0,
//                                         ..default()
//                                     },
//                                     TextColor(Color::srgb(0.8, 0.8, 0.8)),
//                                     Transform::from_translation(Vec3::new(0.0, -10.0, 1.0)),
//                                 ));
//                             });
//                     }
//                 }
//                 None => {}
//             }
//         });
// }

pub fn spawn_shop_ui(mut commands: Commands, shop_state: Res<ShopState>) {
    let buttons = match shop_state.current_shop {
        Some(Shop::Weapon) => vec![widget::button("W1", mock), widget::button("W2", mock)],
        Some(Shop::Upgrade) => vec![widget::button("U1", mock), widget::button("U2", mock)],
        Some(Shop::None) => vec![
            widget::button("Weapon Shop", mock),
            widget::button("Upgrade Shop", mock),
        ],
        None => vec![
            widget::button("Weapon Shop", mock),
            widget::button("Upgrade Shop", mock),
        ],
    };

    commands
        .spawn((
            widget::ui_root("Shop"),
            GlobalZIndex(2),
            StateScoped(Menu::Main),
        ))
        .with_children(|parent| {
            for button in buttons {
                parent.spawn(button);
            }

            #[cfg(target_family = "wasm")]
            {
                parent.spawn(widget::button("Play", enter_loading_or_gameplay_screen));
                parent.spawn(widget::button("Settings", open_settings_menu));
                parent.spawn(widget::button("Credits", open_credits_menu));
            }
        });
}

fn mock(_: Trigger<Pointer<Click>>, mut next_shop: ResMut<NextState<Shop>>) {
    next_shop.set(Shop::Weapon);
}

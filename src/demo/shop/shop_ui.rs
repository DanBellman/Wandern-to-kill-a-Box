use crate::demo::shop::shop::ShopUI;
/// Spawn shop UI
use crate::{
    AppSystems, PausableSystems,
    demo::{
        level::{UpgradeShop, WeaponShop},
        player::{Player, Money},
    },
    screens::Screen,
};
use avian2d::prelude::*;
use bevy::{ecs::system::IntoObserverSystem, prelude::*};

use super::{
    shop::{PlayerUpgrades, Shop, ShopItem, ShopItemButton, ShopState, buy_item, ItemsData, ItemType, WeaponType, UpgradeType},
};

use crate::screens::Screen::Gameplay;
use crate::theme::widget;
use crate::theme::widget::shop_button;

pub(crate) fn spawn_shop_ui(mut commands: Commands, shop_state: Res<ShopState>, items_data: Option<Res<ItemsData>>) {
    commands
        .spawn((
            widget::ui_root("Shop"),
            GlobalZIndex(2),
            StateScoped(Gameplay),
            ShopUI,
        ))
        .with_children(|parent| match shop_state.current_shop {
            Some(Shop::Weapon) => {
                if let Some(items_data) = &items_data {
                    for (weapon_name, weapon_data) in &items_data.config.weapons.types {
                        let weapon_type = WeaponType::from_string(&weapon_data.weapon_type);
                        parent.spawn(widget::shop_button(
                            weapon_name, 
                            buy_item,
                            ShopItemButton {
                                item_name: weapon_name.clone(),
                                item_type: ItemType::Weapon(weapon_type),
                            }
                        ));
                    }
                }
            }
            Some(Shop::Upgrade) => {
                if let Some(items_data) = &items_data {
                    for (upgrade_name, upgrade_data) in &items_data.config.upgrades.types {
                        let upgrade_type = UpgradeType::from_string(&upgrade_data.upgrade_type);
                        parent.spawn(widget::shop_button(
                            upgrade_name, 
                            buy_item,
                            ShopItemButton {
                                item_name: upgrade_name.clone(),
                                item_type: ItemType::Upgrade(upgrade_type),
                            }
                        ));
                    }
                }
            }
            Some(Shop::None) | None => {
                parent.spawn(widget::button("Weapon Shop", buy_item));
                parent.spawn(widget::button("Upgrade Shop", buy_item));
            }
        });
}

/// Close shop UI when leaving shop area
pub(crate) fn update_shop_ui(
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

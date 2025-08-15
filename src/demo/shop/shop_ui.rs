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
    shop::{PlayerUpgrades, Shop, ShopItem, ShopItemButton, ShopState, buy_item},
    //shop_ui::ShopUI,
};

use crate::{ theme::widget};
use crate::screens::Screen::Gameplay;

pub fn spawn_shop_ui(mut commands: Commands, shop_state: Res<ShopState>) {

    commands
        .spawn((
            widget::ui_root("Shop"),
            GlobalZIndex(2),
            StateScoped(Gameplay),
        ))
        .with_children(|parent| {
            match shop_state.current_shop {
                Some(Shop::Weapon) => {
                    for name in PlayerUpgrades::weapon_names() {
                        parent.spawn(widget::button(name, buy_item));
                    }
                }
                Some(Shop::Upgrade) => {
                    for name in PlayerUpgrades::upgrade_names() {
                        parent.spawn(widget::button(name, buy_item));
                    }
                }
                Some(Shop::None) | None => {
                    parent.spawn(widget::button("Weapon Shop", buy_item));
                    parent.spawn(widget::button("Upgrade Shop", buy_item));
                }
            }

        });
}


use bevy::prelude::*;

pub(crate) mod shop;
pub(crate) mod shop_ui;

pub(super) fn plugin(app: &mut App) {
    shop::plugin(app);
}

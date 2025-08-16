use bevy::prelude::*;

mod animation;
mod hud;
pub mod level;
pub mod player;
pub mod shop;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        hud::plugin,
        level::plugin,
        player::plugin,
        shop::plugin,
    ));
}

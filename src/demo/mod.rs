use bevy::prelude::*;

mod animation;
mod hud;
pub mod level;
mod movement;
pub mod player;
mod shooting;
mod shop;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        hud::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        shooting::plugin,
        shop::plugin,
    ));
}

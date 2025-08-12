//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

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

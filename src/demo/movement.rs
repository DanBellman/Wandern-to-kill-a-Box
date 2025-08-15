//! Handle player input and translate it into movement through a character
//! controller. A character controller is the collection of systems that govern
//! the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set [`MovementController`] intent based on directional keyboard input.
//!   This is done in the `player` module, as it is specific to the player
//!   character.
//! - Apply movement based on [`MovementController`] intent and maximum speed.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is limited for demonstration
//! purposes. If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppSystems, PausableSystems, demo::shop::shop::PlayerUpgrades};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();

    app.add_systems(
        Update,
        (apply_movement, apply_screen_limits)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// The direction the character wants to move in.
    pub intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics engine.
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            // 400 pixels per second is a nice default, but we can still vary this per character.
            max_speed: 400.0,
        }
    }
}

fn apply_movement(
    upgrades: Res<PlayerUpgrades>,
    mut movement_query: Query<
        (&MovementController, &mut LinearVelocity, &Transform),
        With<RigidBody>,
    >,
) {
    for (controller, mut velocity, _transform) in &mut movement_query {
        let speed_multiplier = 1.0 + (upgrades.speed_boost as f32 * 0.3);
        velocity.x = controller.max_speed * speed_multiplier * controller.intent.x;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenLimit;

fn apply_screen_limits(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<ScreenLimit>>,
) {
    if let Ok(window) = window_query.single() {
        let window_aspect = window.width() / window.height();
        let viewport_height = 600.0; // Fixed height from camera setup
        let viewport_width = viewport_height * window_aspect;
        let half_width = viewport_width / 2.0;

        // Player boundaries at screen edges
        let left_boundary = -half_width + 16.0;
        let right_boundary = half_width - 16.0;

        for mut player_transform in &mut player_query {
            // Clamp the player's position to screen boundaries
            if player_transform.translation.x < left_boundary {
                player_transform.translation.x = left_boundary;
            }
            if player_transform.translation.x > right_boundary {
                player_transform.translation.x = right_boundary;
            }
        }
    }
}

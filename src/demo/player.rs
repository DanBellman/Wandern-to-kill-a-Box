//! Player-specific behavior.

use bevy::prelude::*;
use avian2d::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    demo::{
        movement::{
            MovementController,
            ScreenLimit
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    //app.register_type::<PlayerAssets>();
    //app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input.in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The player character.
pub fn player(
    max_speed: f32,
) -> impl Bundle {
    // Simple green box player

    (
        Name::new("Player"),
        Player,
        Sprite {
            color: Color::srgb(0.0, 0.8, 0.2), // Green box
            custom_size: Some(Vec2::new(32.0, 48.0)), // Reasonable player size
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -227.0, 0.0)), // Position a tiny bit up
        MovementController {
            max_speed,
            ..default()
        },
        ScreenLimit, // Re-enable screen limits to prevent walking past walls
        // Physics components for collision detection only
        RigidBody::Kinematic,
        LinearVelocity::ZERO, // Explicitly add LinearVelocity component
        GravityScale(0.0), // No gravity - floating movement
        LockedAxes::ROTATION_LOCKED, // Prevent rotation
        // Add collider for coin collection only
        Collider::rectangle(32.0, 48.0), // Match sprite size
        Sensor,
        CollisionEventsEnabled,
        LinearDamping(0.0), // Remove damping that's slowing movement
        AngularDamping(0.0), // Remove angular damping
    )
}


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    let horizontal_intent = Vec2::new(intent.x, 0.0);

    for mut controller in &mut controller_query {
        controller.intent = horizontal_intent;
    }
}

// #[derive(Resource, Asset, Clone, Reflect)]
// #[reflect(Resource)]
// pub struct PlayerAssets {
//     #[dependency]
//     pub steps: Vec<Handle<AudioSource>>,
// }

// impl FromWorld for PlayerAssets {
//     fn from_world(world: &mut World) -> Self {
//         let assets = world.resource::<AssetServer>();
//         Self {
//             steps: vec![
//                 assets.load("audio/sound_effects/step1.ogg"),
//                 assets.load("audio/sound_effects/step2.ogg"),
//                 assets.load("audio/sound_effects/step3.ogg"),
//                 assets.load("audio/sound_effects/step4.ogg"),
//             ],
//         }
//     }
// }

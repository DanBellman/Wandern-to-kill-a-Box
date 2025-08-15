//! Player-specific behavior.

use crate::asset_tracking::LoadResource;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{
    AppSystems, PausableSystems,
    demo::movement::{MovementController, ScreenLimit},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (
            record_player_directional_input.in_set(AppSystems::RecordInput),
            update_player_size_on_window_resize,
        )
            .in_set(PausableSystems),
    );
}

/// The player character.
pub fn player(max_speed: f32, player_assets: &PlayerAssets) -> impl Bundle {
    let player_height = 48.0;
    let player_width = 32.0;

    (
        Name::new("Player"),
        Player,
        Sprite {
            image: player_assets.player_sprite.clone(),
            custom_size: Some(Vec2::new(player_width, player_height)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -240.0, 0.0)), // Same ground level as coins
        MovementController {
            max_speed,
            ..default()
        },
        ScreenLimit,
        RigidBody::Kinematic,
        LinearVelocity::ZERO,
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(player_width, player_height),
        Sensor,
        CollisionEventsEnabled,
        LinearDamping(0.0),
        AngularDamping(0.0),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }

    let horizontal_dir = Vec2::new(dir.x, 0.0);

    for mut controller in &mut controller_query {
        controller.intent = horizontal_dir;
    }
}

/// System that updates player size when window is resized to maintain proportions
fn update_player_size_on_window_resize(
    mut player_query: Query<&mut Sprite, With<Player>>,
    window_query: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
) {
    for _window in window_query.iter() {
        let player_height = 48.0;
        let player_width = 32.0;

        for mut sprite in player_query.iter_mut() {
            sprite.custom_size = Some(Vec2::new(player_width, player_height));
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub player_sprite: Handle<Image>,
    // #[dependency]
    // pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player_sprite: assets.load("Player.exr"),
            // steps: vec![
            //     assets.load("audio/sound_effects/step1.ogg"),
            //     assets.load("audio/sound_effects/step2.ogg"),
            //     assets.load("audio/sound_effects/step3.ogg"),
            //     assets.load("audio/sound_effects/step4.ogg"),
            // ],
        }
    }
}

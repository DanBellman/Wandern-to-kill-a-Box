//! Player-specific behavior.

use crate::asset_tracking::LoadResource;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::PausableSystems;

pub mod movement;
pub mod shooting;

pub use movement::{DefaultInputContext, MovementSpeed, ScreenLimit};
pub use shooting::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, shooting::plugin));
    app.register_type::<Player>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    app.add_systems(
        Update,
        (
            update_player_size_on_window_resize,
            add_input_context_to_player,
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
        MovementSpeed { max_speed },
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

fn add_input_context_to_player(
    mut commands: Commands,
    player_query: Query<Entity, (With<Player>, Without<DefaultInputContext>)>,
) {
    for entity in &player_query {
        commands.entity(entity).insert(DefaultInputContext);
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

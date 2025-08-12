//! Spawn the main level.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use avian2d::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    demo::{
        player::{PlayerAssets, player},
        shooting::Target,
    },
    screens::Screen,
    AppSystems, PausableSystems,
};

#[derive(Component)]
pub struct Level;


#[derive(Component)]
pub struct InvisibleWall;

#[derive(Component)]
pub struct LeftWall;

#[derive(Component)]
pub struct RightWall;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WeaponShop;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UpgradeShop;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();

    // Add system to position walls at viewport edges
    app.add_systems(
        Update,
        position_invisible_walls
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("myBackground.exr"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    _player_assets: Res<PlayerAssets>,
    _texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Level,
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            background(&level_assets),
            player(400.0),
            yellow_box(),
            shop_box_upgrades(),
            shop_box_weapons(),
            // Invisible ground for coins
            //FIXME: Player and coins are not on the same ground.
            invisible_ground(),
            // Invisible walls
            left_wall(),
            right_wall(),
            // (
            //     Name::new("Gameplay Music"),
            //     music(level_assets.music.clone())
            // )
        ],
    ));
}

/// System that positions invisible walls when first spawned
fn position_invisible_walls(
    mut left_wall_query: Query<&mut Transform, (With<LeftWall>, Added<LeftWall>, Without<RightWall>)>,
    mut right_wall_query: Query<&mut Transform, (With<RightWall>, Added<RightWall>, Without<LeftWall>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.single() {
        let window_aspect = window.width() / window.height();
        let viewport_height = 600.0;
        let viewport_width = viewport_height * window_aspect;
        let half_width = viewport_width / 2.0;

        for mut transform in &mut left_wall_query {
            transform.translation.x = -half_width + 50.0;
        }

        for mut transform in &mut right_wall_query {
            transform.translation.x = half_width - 50.0;
        }
    }
}


/// Creates the left invisible wall
fn left_wall() -> impl Bundle {
    (
        Name::new("Left Wall"),
        InvisibleWall,
        LeftWall,
        Transform::default(),
    )
}

/// Creates the right invisible wall
fn right_wall() -> impl Bundle {
    (
        Name::new("Right Wall"),
        InvisibleWall,
        RightWall,
        Transform::default(),
    )
}

/// Creates the HDR background
fn background(level_assets: &LevelAssets) -> impl Bundle {
    (
        Name::new("HDR Background"),
        Sprite {
            image: level_assets.background.clone(),
            custom_size: Some(Vec2::new(1200.0, 675.0)), // 16:9 aspect ratio scaled
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)), // behind everything else
    )
}

/// Creates a yellow box in the middle of the screen
fn yellow_box() -> impl Bundle {
    (
        Name::new("Yellow Box"),
        Target::default(), // shootable with damage timers
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(35.0, 35.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -130.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(35.0, 35.0),
    )
}

/// Creates invisible ground for coins to land on
fn invisible_ground() -> impl Bundle {
    (
        Name::new("Invisible Ground"),
        // No sprite - completely invisible
        Transform::from_translation(Vec3::new(0.0, -270.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(2000.0, 20.0),
        CollisionLayers::new(LayerMask(0b0001), LayerMask(0b0010)), // On layer 0, collides with layer 1 (coins)
    )
}

fn shop_box_upgrades() -> impl Bundle {
    (
        Name::new("Shop Box Upgrades"),
        UpgradeShop,
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-190.0, -227.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0), // Shop interaction area
        Sensor,
        CollisionEventsEnabled,
    )
}

fn shop_box_weapons() -> impl Bundle {
    (
        Name::new("Shop Box Weapons"),
        WeaponShop,
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-300.0, -227.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
        Sensor,
        CollisionEventsEnabled,
    )
}


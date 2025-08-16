//! Spawn the main level.

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::PrimaryWindow;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    demo::player::{CoinBoxMaterial, PlayerAssets, Target, player},
    screens::Screen,
};

#[derive(Component)]
pub struct Level;

#[derive(Component)]
pub struct InvisibleWall;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct ShopSprite;

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

    app.add_plugins(Material2dPlugin::<CoinBoxMaterial>::default());

    app.add_systems(
        Update,
        (
            position_invisible_walls,
            update_background_size,
            update_shop_sprite_size,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    background: Handle<Image>,
    #[dependency]
    weapon_shop: Handle<Image>,
    #[dependency]
    upgrade_shop: Handle<Image>,
    #[dependency]
    coin_box: Handle<Image>,
    #[dependency]
    pub coin: Handle<Image>,
    pub ground_level: f32,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("myBackground.exr"),
            weapon_shop: assets.load("WeaponShop.exr"),
            upgrade_shop: assets.load("UpgradeShop.exr"),
            coin_box: assets.load("CoinBox.exr"),
            coin: assets.load("Coin.exr"),
            ground_level: -250.0,
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    existing_level_query: Query<(), With<Level>>,
    _texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CoinBoxMaterial>>,
) {
    // Don't spawn level if it already exists
    if !existing_level_query.is_empty() {
        return;
    }

    let Ok(window) = window_query.single() else {
        return; // Skip if window not ready yet
    };

    commands.spawn((
        Name::new("Level"),
        Level,
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            background(&level_assets, window),
            player(400.0, &player_assets),
            coin_box(&level_assets, &mut meshes, &mut materials),
            shop_box_upgrades(&level_assets),
            shop_box_weapons(&level_assets),
            // Invisible ground for both player and coins
            invisible_ground(&level_assets),
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
    mut left_wall_query: Query<
        &mut Transform,
        (With<LeftWall>, Added<LeftWall>, Without<RightWall>),
    >,
    mut right_wall_query: Query<
        &mut Transform,
        (With<RightWall>, Added<RightWall>, Without<LeftWall>),
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.single() else {
        return; // Skip if window not ready yet
    };
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

/// System that updates background size when window is resized
fn update_background_size(
    mut background_query: Query<&mut Sprite, With<Background>>,
    window_query: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
) {
    // Only run when window has changed
    for window in window_query.iter() {
        let world_height = 600.0;
        let window_aspect = window.width() / window.height();
        let world_width = world_height * window_aspect;

        // Update only the background sprite
        for mut sprite in background_query.iter_mut() {
            sprite.custom_size = Some(Vec2::new(world_width, world_height));
        }
    }
}

/// updates shop sprite sizes to scale with window
fn update_shop_sprite_size(
    mut shop_query: Query<&mut Sprite, With<ShopSprite>>,
    window_query: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
) {
    for window in window_query.iter() {
        let world_height = 600.0;
        let window_aspect = window.width() / window.height();
        let base_resolution = 1920.0 / 1080.0;
        let scale_factor = (window_aspect / base_resolution).min(1.2).max(0.8);
        let shop_size = 60.0 * scale_factor;
        for mut sprite in shop_query.iter_mut() {
            sprite.custom_size = Some(Vec2::new(shop_size, shop_size));
        }
    }
}

fn left_wall() -> impl Bundle {
    (
        Name::new("Left Wall"),
        InvisibleWall,
        LeftWall,
        Transform::default(),
    )
}

fn right_wall() -> impl Bundle {
    (
        Name::new("Right Wall"),
        InvisibleWall,
        RightWall,
        Transform::default(),
    )
}

/// Creates the HDR background
fn background(level_assets: &LevelAssets, window: &Window) -> impl Bundle {
    // Calculate the world space dimensions that will be visible
    // Camera uses FixedVertical scaling with 600px height
    let world_height = 600.0;
    let window_aspect = window.width() / window.height();
    let world_width = world_height * window_aspect;

    // Scale the background to fill the entire visible world space
    (
        Name::new("HDR Background"),
        Background,
        Sprite {
            image: level_assets.background.clone(),
            custom_size: Some(Vec2::new(world_width, world_height)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)), // behind everything else
    )
}

/// Creates a coin box in the middle of the screen with shimmer effect
fn coin_box(
    level_assets: &LevelAssets,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<CoinBoxMaterial>>,
) -> impl Bundle {
    let coinbox_width = 70.0;
    let coinbox_height = 70.0;
    (
        Name::new("Coin Box"),
        Target::default(), // shootable with damage timers
        Mesh2d(meshes.add(Rectangle::new(coinbox_width, coinbox_height))),
        MeshMaterial2d(materials.add(CoinBoxMaterial {
            base_color_texture: level_assets.coin_box.clone(),
        })),
        Transform::from_translation(Vec3::new(0.0, -130.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(coinbox_width, coinbox_height),
        Sensor,
        CollisionEventsEnabled,
    )
}

/// Creates invisible ground for both player and coins
fn invisible_ground(level_assets: &LevelAssets) -> impl Bundle {
    (
        Name::new("Invisible Ground"),
        // No sprite - completely invisible
        Transform::from_translation(Vec3::new(0.0, level_assets.ground_level, 0.0)),
        RigidBody::Static,
        Collider::rectangle(2000.0, 20.0),
        CollisionLayers::new(LayerMask(0b0001), LayerMask(0b0011)), // On layer 0, collides with layer 1 (coins) and layer 2 (player)
    )
}

fn shop_box_upgrades(level_assets: &LevelAssets) -> impl Bundle {
    (
        Name::new("Shop Box Upgrades"),
        UpgradeShop,
        ShopSprite,
        Sprite {
            image: level_assets.upgrade_shop.clone(),
            custom_size: Some(Vec2::new(60.0, 60.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-190.0, level_assets.ground_level, 0.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0), // Shop interaction area
        Sensor,
        CollisionEventsEnabled,
    )
}

fn shop_box_weapons(level_assets: &LevelAssets) -> impl Bundle {
    (
        Name::new("Shop Box Weapons"),
        WeaponShop,
        ShopSprite,
        Sprite {
            image: level_assets.weapon_shop.clone(),
            custom_size: Some(Vec2::new(60.0, 60.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-300.0, level_assets.ground_level, 0.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
        Sensor,
        CollisionEventsEnabled,
    )
}

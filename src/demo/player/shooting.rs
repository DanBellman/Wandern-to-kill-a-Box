//! Shooting system for player projectiles

use avian2d::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};
use rand::Rng;

use crate::{
    AppSystems, PausableSystems,
    demo::hud::{CoinBuffer, ScoreText},
    demo::level::LevelAssets,
    demo::shop::shop::{PlayerUpgrades, WeaponType},
    screens::Screen,
};

use super::{Player, ScreenLimit};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Projectile>();
    app.register_type::<Coin>();
    app.register_type::<CoinLanded>();
    app.register_type::<LaserBeam>();
    app.init_resource::<Money>();

    app.add_plugins(Material2dPlugin::<CoinMaterial>::default());

    app.add_systems(
        Update,
        (
            handle_shooting,
            handle_laser_beam,
            handle_projectile_collisions,
            handle_laser_continuous_damage,
            collect_coins,
            update_money_display,
            disable_coin_physics_on_ground,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub weapon_type: WeaponType,
}

impl Projectile {
    pub fn get_coin_amount(&self) -> u32 {
        match self.weapon_type {
            WeaponType::Normal => 100,
            WeaponType::RapidFire => 75,
            WeaponType::SpreadShot => 50,
            WeaponType::LaserBeam => 150,
            WeaponType::Uzi => 50,
            WeaponType::Sniper => 200,
            WeaponType::Bazooka => 400,
            WeaponType::Hammer => 300,
            WeaponType::Sword => 250,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Coin {
    pub value: u32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CoinLanded;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LaserBeam {
    pub direction: Vec2,
    pub length: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Target {
    pub last_damage_time: f32,
    pub laser_damage_timer: f32,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            last_damage_time: 0.0,
            laser_damage_timer: 0.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct Money {
    pub amount: u32,
}

fn handle_shooting(
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    upgrades: Res<PlayerUpgrades>,
    mut commands: Commands,
    existing_laser_query: Query<Entity, With<LaserBeam>>,
) {
    if upgrades.current_weapon == WeaponType::LaserBeam {
        if mouse_input.pressed(MouseButton::Right) {
            // Keep laser active, spawn if doesn't exist
            if existing_laser_query.is_empty() {
                if let (Ok((camera, camera_transform)), Ok(window), Ok(player_transform)) = (
                    camera_query.single(),
                    window_query.single(),
                    player_query.single(),
                ) {
                    if let Some(cursor_pos) = window.cursor_position() {
                        if let Ok(world_pos) =
                            camera.viewport_to_world_2d(camera_transform, cursor_pos)
                        {
                            let player_pos = player_transform.translation.truncate();
                            let direction = (world_pos - player_pos).normalize();
                            spawn_continuous_laser(&mut commands, player_pos, direction);
                        }
                    }
                }
            }
        } else {
            // Mouse released, remove laser
            for laser_entity in &existing_laser_query {
                commands.entity(laser_entity).despawn();
            }
        }
        return;
    }

    if mouse_input.just_pressed(MouseButton::Right) {
        if let (Ok((camera, camera_transform)), Ok(window), Ok(player_transform)) = (
            camera_query.single(),
            window_query.single(),
            player_query.single(),
        ) {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    let player_pos = player_transform.translation.truncate();
                    let direction = (world_pos - player_pos).normalize();

                    match upgrades.current_weapon {
                        WeaponType::Normal => {
                            spawn_projectile(
                                &mut commands,
                                player_pos,
                                direction,
                                WeaponType::Normal,
                            );
                        }
                        WeaponType::RapidFire => {
                            spawn_projectile(
                                &mut commands,
                                player_pos,
                                direction,
                                WeaponType::RapidFire,
                            );
                        }
                        WeaponType::SpreadShot => {
                            spawn_projectile(
                                &mut commands,
                                player_pos,
                                direction,
                                WeaponType::SpreadShot,
                            );
                            spawn_projectile(
                                &mut commands,
                                player_pos,
                                direction.rotate(Vec2::from_angle(0.2)),
                                WeaponType::SpreadShot,
                            );
                            spawn_projectile(
                                &mut commands,
                                player_pos,
                                direction.rotate(Vec2::from_angle(-0.2)),
                                WeaponType::SpreadShot,
                            );
                        }
                        WeaponType::LaserBeam => {
                            //FIXME still buggy
                            spawn_laser_projectile(
                                &mut commands,
                                player_pos,
                                direction,
                                WeaponType::LaserBeam,
                            );
                        }
                        WeaponType::Uzi => {
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::Uzi);
                        }
                        WeaponType::Sniper => {
                            spawn_sniper_projectile(
                                &mut commands,
                                player_pos,
                                direction,
                                WeaponType::Sniper,
                            );
                        }
                        WeaponType::Bazooka => {
                            spawn_projectile(
                                &mut commands,
                                player_pos,
                                direction,
                                WeaponType::Bazooka,
                            );
                        }
                        WeaponType::Hammer => {
                            spawn_projectile(
                                &mut commands,
                                player_pos,
                                direction,
                                WeaponType::Hammer,
                            );
                        }
                        WeaponType::Sword => {
                            spawn_projectile(
                                &mut commands,
                                player_pos,
                                direction,
                                WeaponType::Sword,
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Spawn a laser projectile (faster, different appearance)
fn spawn_laser_projectile(
    commands: &mut Commands,
    start_pos: Vec2,
    direction: Vec2,
    weapon_type: WeaponType,
) {
    commands.spawn((
        Name::new("Laser Projectile"),
        Projectile {
            velocity: direction * 1200.0,
            lifetime: 5.0,
            weapon_type,
        },
        Sprite {
            color: Color::srgb(0.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(3.0, 20.0)),
            ..default()
        },
        Transform::from_translation(start_pos.extend(5.0)),
        RigidBody::Dynamic,
        Collider::rectangle(1.5, 10.0),
        Sensor,
        LinearVelocity(direction * 1200.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
    ));
}

/// Spawn a sniper projectile (very fast, high damage, distinctive appearance)
fn spawn_sniper_projectile(
    commands: &mut Commands,
    start_pos: Vec2,
    direction: Vec2,
    weapon_type: WeaponType,
) {
    commands.spawn((
        Name::new("Sniper Bullet"),
        Projectile {
            velocity: direction * 2000.0, // Much faster than normal bullets
            lifetime: 6.0,                // Longer range
            weapon_type,
        },
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.0),       // Golden bullet color
            custom_size: Some(Vec2::new(2.0, 12.0)), // Thin, long bullet
            ..default()
        },
        Transform::from_translation(start_pos.extend(5.0)),
        RigidBody::Dynamic,
        Collider::rectangle(1.0, 6.0),
        Sensor,
        LinearVelocity(direction * 2000.0), // Very high speed
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
    ));
}

/// Spawn a projectile from player toward target
fn spawn_projectile(
    commands: &mut Commands,
    start_pos: Vec2,
    direction: Vec2,
    weapon_type: WeaponType,
) {
    commands.spawn((
        Name::new("Projectile"),
        Projectile {
            velocity: direction * 800.0,
            lifetime: 3.0,
            weapon_type,
        },
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(8.0, 8.0)),
            ..default()
        },
        Transform::from_translation(start_pos.extend(5.0)),
        RigidBody::Dynamic,
        Collider::circle(4.0),
        Sensor, // Donesnt physically push other objects
        LinearVelocity(direction * 800.0),
        GravityScale(0.0), // no gravity on projectiles
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
    ));
}

/// Handle collisions between projectiles and targets using Avian2D collision events
fn handle_projectile_collisions(
    mut collision_events: EventReader<CollisionStarted>,
    projectile_query: Query<&Projectile>,
    mut target_query: Query<(&Transform, &mut Target)>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut coin_materials: ResMut<Assets<CoinMaterial>>,
    time: Res<Time>,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        // Check if one entity is a projectile and the other is a target
        let (projectile_entity, target_entity) = if projectile_query.contains(*entity1) {
            (*entity1, *entity2)
        } else if projectile_query.contains(*entity2) {
            (*entity2, *entity1)
        } else {
            continue; // Neither entity is a projectile
        };

        // Get the projectile and target components
        if let (Ok(projectile), Ok((target_transform, mut target))) = (
            projectile_query.get(projectile_entity),
            target_query.get_mut(target_entity),
        ) {
            let current_time = time.elapsed_secs();

            // Prevent rapid-fire damage from same weapon type
            if current_time - target.last_damage_time > 0.1 {
                target.last_damage_time = current_time;
                commands.entity(projectile_entity).despawn();
                let coin_amount = projectile.get_coin_amount();

                spawn_weapon_coins(
                    &mut commands,
                    target_transform.translation.truncate(),
                    coin_amount,
                    projectile.weapon_type,
                    &level_assets,
                    &mut meshes,
                    &mut materials,
                    &mut coin_materials,
                );
            }
        }
    }
}

/// Spawn coins based on weapon type
fn spawn_weapon_coins(
    commands: &mut Commands,
    position: Vec2,
    base_value: u32,
    weapon_type: WeaponType,
    level_assets: &Res<LevelAssets>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    coin_materials: &mut ResMut<Assets<CoinMaterial>>,
) {
    use std::f32::consts::TAU;

    let coin_count = match weapon_type {
        WeaponType::Bazooka => 40,
        WeaponType::Sniper => 20,
        WeaponType::Hammer => 7,
        WeaponType::Sword => 6,
        WeaponType::LaserBeam => 5,
        WeaponType::Normal => 4,
        WeaponType::RapidFire => 3,
        WeaponType::Uzi => 2,
        WeaponType::SpreadShot => 2,
    };

    for i in 0..coin_count {
        let mut rng = rand::thread_rng();

        // Completely random angle
        let angle: f32 = rng.gen_range(0.0..TAU);

        // Random distance from center
        let distance = rng.gen_range(20.0..80.0);

        let random_offset = Vec2::new(angle.cos() * distance, angle.sin() * distance);

        commands.spawn((
            Name::new("Coin"),
            Coin {
                value: base_value / coin_count,
            },
            // Shimmer shader version
            Mesh2d(meshes.add(Circle::new(8.0))),
            MeshMaterial2d(coin_materials.add(CoinMaterial {
                base_color_texture: level_assets.coin.clone(),
            })),
            // Yellow procedural version (uncomment to test performance)
            // This works: start
            // Mesh2d(meshes.add(Circle::new(8.0))),
            // MeshMaterial2d(materials.add(Color::srgb(1.0, 0.8, 0.0))),
            // This works: end
            Transform::from_translation((position + random_offset).extend(1.0)),
            RigidBody::Dynamic,
            Collider::circle(8.0),
            CollisionLayers::new(LayerMask(0b0010), LayerMask(0b0001)), // On layer 1, collides with layer 0 (ground)
            CollisionEventsEnabled,
            LinearVelocity::ZERO, // Like player - no velocity
            GravityScale(10.0),   // Like player - no gravity
            AngularVelocity(2.0 + i as f32),
            LockedAxes::ROTATION_LOCKED, // Like player - no rotation
            ScreenLimit,
        ));
    }
}

/// Collect coins when player touches them using collision events
fn collect_coins(
    mut collision_events: EventReader<CollisionStarted>,
    player_query: Query<Entity, With<Player>>,
    coin_query: Query<&Coin>,
    mut money: ResMut<Money>,
    mut buffer: ResMut<CoinBuffer>,
    mut commands: Commands,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        let (player_entity, coin_entity) =
            if player_query.contains(*entity1) && coin_query.contains(*entity2) {
                (Some(*entity1), Some(*entity2))
            } else if player_query.contains(*entity2) && coin_query.contains(*entity1) {
                (Some(*entity2), Some(*entity1))
            } else {
                (None, None)
            };

        if let (Some(_player), Some(coin)) = (player_entity, coin_entity) {
            if let Ok(coin_component) = coin_query.get(coin) {
                // Only collect if buffer has space
                if buffer.current < buffer.max {
                    money.amount += coin_component.value;
                    buffer.add_coin(); // Add to buffer
                    commands.entity(coin).despawn();
                }
                // If buffer is full, coin stays and doesn't disappear
            }
        }
    }
}

/// Update money display text
fn update_money_display(money: Res<Money>, mut text_query: Query<&mut Text2d, With<ScoreText>>) {
    if money.is_changed() {
        for mut text in &mut text_query {
            **text = format!("Money: ${}", money.amount);
        }
    }
}

/// Spawn a continuous laser beam
fn spawn_continuous_laser(commands: &mut Commands, start_pos: Vec2, direction: Vec2) {
    let laser_length = 600.0;

    commands.spawn((
        Name::new("Laser Beam"),
        LaserBeam {
            direction,
            length: laser_length,
        },
        Sprite {
            color: Color::srgb(1.0, 0.3, 0.3),
            custom_size: Some(Vec2::new(laser_length, 3.0)),
            ..default()
        },
        Transform::from_translation(start_pos.extend(6.0))
            .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x))),
        RigidBody::Kinematic, // Use kinematic instead of static for better updates
        Collider::rectangle(laser_length / 2.0, 1.5),
        Sensor,
        CollisionEventsEnabled,
    ));
}

/// Update laser beam position and direction
fn handle_laser_beam(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    mut laser_query: Query<(&mut LaserBeam, &mut Transform), (With<LaserBeam>, Without<Player>)>,
) {
    if let (Ok((camera, camera_transform)), Ok(window), Ok(player_transform)) = (
        camera_query.single(),
        window_query.single(),
        player_query.single(),
    ) {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let player_pos = player_transform.translation.truncate();
                let direction = (world_pos - player_pos).normalize();

                for (mut laser_beam, mut transform) in &mut laser_query {
                    laser_beam.direction = direction;

                    let laser_center = player_pos + direction * (laser_beam.length / 2.0);
                    transform.translation = laser_center.extend(6.0);

                    let angle = direction.y.atan2(direction.x);
                    transform.rotation = Quat::from_rotation_z(angle);
                }
            }
        }
    }
}

/// Handle continuous laser beam damage to targets
/// This should be like the melee weapons, the only difference is that
/// the damage is not one time, but continuous.
fn handle_laser_continuous_damage(
    laser_query: Query<&Transform, With<LaserBeam>>,
    mut target_query: Query<(&Transform, &mut Target)>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut coin_materials: ResMut<Assets<CoinMaterial>>,
    time: Res<Time>,
) {
    if laser_query.is_empty() {
        return;
    }

    for (target_transform, mut target) in &mut target_query {
        let mut is_being_hit = false;

        for laser_transform in &laser_query {
            let distance = laser_transform
                .translation
                .distance(target_transform.translation);
            if distance < 300.0 {
                is_being_hit = true;
                break;
            }
        }

        if is_being_hit {
            target.laser_damage_timer += time.delta_secs();
            if target.laser_damage_timer >= 0.3 {
                target.laser_damage_timer = 0.0;

                spawn_weapon_coins(
                    &mut commands,
                    target_transform.translation.truncate(),
                    120,
                    WeaponType::LaserBeam,
                    &level_assets,
                    &mut meshes,
                    &mut materials,
                    &mut coin_materials,
                );
            }
        } else {
            target.laser_damage_timer = 0.0;
        }
    }
}

const SHADER_ASSET_PATH: &str = "shaders/animate_shader.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CoinBoxMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Handle<Image>,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CoinMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Handle<Image>,
}

impl Material2d for CoinBoxMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

impl Material2d for CoinMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

/// Disable physics for coins that have landed on the ground
fn disable_coin_physics_on_ground(
    mut collision_events: EventReader<CollisionStarted>,
    coin_query: Query<Entity, (With<Coin>, Without<CoinLanded>)>,
    ground_query: Query<Entity, With<Name>>,
    name_query: Query<&Name>,
    mut commands: Commands,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        // Check if a coin hit the invisible ground
        let (coin_entity, ground_entity) = if coin_query.contains(*entity1) {
            (*entity1, *entity2)
        } else if coin_query.contains(*entity2) {
            (*entity2, *entity1)
        } else {
            continue;
        };

        // Check if the other entity is the invisible ground
        if let Ok(name) = name_query.get(ground_entity) {
            if name.as_str() == "Invisible Ground" {
                // Only modify the coin if it still exists and hasn't been collected
                if coin_query.get(coin_entity).is_ok() {
                    commands
                        .entity(coin_entity)
                        .insert(CoinLanded)
                        .remove::<RigidBody>()
                        .remove::<LinearVelocity>()
                        .remove::<AngularVelocity>()
                        .remove::<GravityScale>()
                        .remove::<LockedAxes>();
                    // Keep CollisionLayers and Collider so player can still collect the coin
                }
            }
        }
    }
}

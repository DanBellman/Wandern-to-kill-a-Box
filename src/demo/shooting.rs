//! Shooting system for player projectiles

use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;
use avian2d::prelude::*;

use crate::{AppSystems, PausableSystems, demo::player::Player, demo::hud::{ScoreText, CoinBuffer}, demo::movement::ScreenLimit, demo::shop::{PlayerUpgrades, WeaponType}};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Projectile>();
    app.register_type::<Coin>();
    app.register_type::<LaserBeam>();
    app.init_resource::<Money>();

    app.add_systems(
        Update,
        (
            handle_shooting,
            handle_laser_beam,
            move_projectiles,
            handle_projectile_collisions,
            handle_laser_continuous_damage,
            collect_coins,
            update_money_display,
            coin_magnet_effect,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
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
            WeaponType::RapidFire => 75,  // Lower per shot but fires faster
            WeaponType::SpreadShot => 50, // Lower per shot but 3 shots
            WeaponType::LaserBeam => 150, // Higher damage per projectile
            WeaponType::Uzi => 50 , // Fast firing, lower damage
            WeaponType::Sniper => 200, // High damage, single shot
            WeaponType::Bazooka => 400, // High damage
            WeaponType::Hammer => 300, // High damage melee
            WeaponType::Sword => 250, // Medium-high damage melee
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

/// Handle right-click shooting
fn handle_shooting(
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    upgrades: Res<PlayerUpgrades>,
    mut commands: Commands,
    existing_laser_query: Query<Entity, With<LaserBeam>>,
) {
    // Handle laser beam differently - continuous while held
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
                        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
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

    // Handle other weapons - single shot
    if mouse_input.just_pressed(MouseButton::Right) {
        if let (Ok((camera, camera_transform)), Ok(window), Ok(player_transform)) = (
            camera_query.single(),
            window_query.single(),
            player_query.single(),
        ) {
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert cursor position to world coordinates
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    let player_pos = player_transform.translation.truncate();
                    let direction = (world_pos - player_pos).normalize();

                    // Spawn projectiles based on current weapon
                    match upgrades.current_weapon {
                        WeaponType::Normal => {
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::Normal);
                        },
                        WeaponType::RapidFire => {
                            // Rapid fire - just normal projectile for now (could add rate of fire later)
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::RapidFire);
                        },
                        WeaponType::SpreadShot => {
                            // Spread shot - 3 projectiles
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::SpreadShot);
                            spawn_projectile(&mut commands, player_pos, direction.rotate(Vec2::from_angle(0.2)), WeaponType::SpreadShot);
                            spawn_projectile(&mut commands, player_pos, direction.rotate(Vec2::from_angle(-0.2)), WeaponType::SpreadShot);
                        },
                        WeaponType::LaserBeam => {
                            // Laser beam - faster, longer projectile
                            //FIXME still buggy
                            spawn_laser_projectile(&mut commands, player_pos, direction, WeaponType::LaserBeam);
                        },
                        WeaponType::Uzi => {
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::Uzi);
                        },
                        WeaponType::Sniper => {
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::Sniper);
                        },
                        WeaponType::Bazooka => {
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::Bazooka);
                        },
                        WeaponType::Hammer => {
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::Hammer);
                        },
                        WeaponType::Sword => {
                            spawn_projectile(&mut commands, player_pos, direction, WeaponType::Sword);
                        },
                    }
                }
            }
        }
    }
}

/// Spawn a laser projectile (faster, different appearance)
fn spawn_laser_projectile(commands: &mut Commands, start_pos: Vec2, direction: Vec2, weapon_type: WeaponType) {
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

/// Spawn a projectile from player toward target
fn spawn_projectile(commands: &mut Commands, start_pos: Vec2, direction: Vec2, weapon_type: WeaponType) {
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
    ));
}

/// Move projectiles and handle lifetime
fn move_projectiles(
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut projectile, _transform) in &mut projectile_query {
        // Updates lifetime
        projectile.lifetime -= time.delta_secs();

        // Remove projectile if lifetime expired
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Handle collisions between projectiles and targets using spatial query
fn handle_projectile_collisions(
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    mut target_query: Query<(&Transform, &mut Target)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (projectile_entity, projectile_transform, projectile) in &projectile_query {
        for (target_transform, mut target) in &mut target_query {
            let distance = projectile_transform.translation.distance(target_transform.translation);

            //FIXME
            // Check if projectile is close enough to target (collision detection)
            if distance < 20.0 { // Collision threshold
                let current_time = time.elapsed_secs();

                // Prevent rapid-fire damage from same weapon type
                if current_time - target.last_damage_time > 0.1 {
                    target.last_damage_time = current_time;
                    commands.entity(projectile_entity).despawn();
                    let coin_amount = projectile.get_coin_amount();
                    spawn_weapon_coins(&mut commands, target_transform.translation.truncate(),
                    coin_amount, &mut meshes, &mut materials);
                }
            }
        }
    }
}

/// Spawn coins based on weapon damage
fn spawn_weapon_coins(
    commands: &mut Commands,
    position: Vec2,
    base_value: u32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    use std::f32::consts::TAU;

    //FIXME
    let coin_count = if base_value >= 120 {
        5 // Laser beam
    } else if base_value >= 100 {
        4 // Normal weapon
    } else if base_value >= 75 {
        3 // Rapid fire
    } else {
        2 // Spread shot
    };

    for i in 0..coin_count {
        let angle = (i as f32 / coin_count as f32) * TAU;
        let distance = 40.0;
        let random_offset = Vec2::new(
            angle.cos() * distance + (i as f32 * 10.0 - 15.0), // Some variation
            angle.sin() * distance + (i as f32 * 5.0 - 10.0),
        );

        commands.spawn((
            Name::new("Coin"),
            Coin { value: base_value / coin_count },
            Mesh2d(meshes.add(Circle::new(8.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.8, 0.0))),
            Transform::from_translation((position + random_offset).extend(1.0)),
            RigidBody::Dynamic,
            Collider::circle(8.0),
            CollisionLayers::new(LayerMask(0b0010), LayerMask(0b0001)), // On layer 1, collides with layer 0 (ground)
            CollisionEventsEnabled,
            LinearVelocity(random_offset.normalize() * 100.0), // Initial spread velocity
            AngularVelocity(2.0 + i as f32), // Rotation based on coin number
            GravityScale(10.0),
            AngularDamping(1.0),
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
        let (player_entity, coin_entity) = if player_query.contains(*entity1) && coin_query.contains(*entity2) {
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
fn update_money_display(
    money: Res<Money>,
    mut text_query: Query<&mut Text2d, With<ScoreText>>,
) {
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
            color: Color::srgb(1.0, 0.3, 0.3), // Bright red laser beam
            custom_size: Some(Vec2::new(laser_length, 3.0)), // Thin laser
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
                    // Update laser direction
                    laser_beam.direction = direction;

                    // Position laser starting from player, extending in direction
                    let laser_center = player_pos + direction * (laser_beam.length / 2.0);
                    transform.translation = laser_center.extend(6.0);

                    // Rotate laser to point toward cursor
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    if laser_query.is_empty() {
        return;
    }

    for (target_transform, mut target) in &mut target_query {
        // Check if any laser beam is hitting this target
        let mut is_being_hit = false;

        for laser_transform in &laser_query {
            let distance = laser_transform.translation.distance(target_transform.translation);
            if distance < 300.0 { // Laser hit range (half laser length)
                is_being_hit = true;
                break;
            }
        }

        if is_being_hit {
            // Continuous laser damage - spawn coins every 0.3 seconds
            target.laser_damage_timer += time.delta_secs();
            if target.laser_damage_timer >= 0.3 {
                target.laser_damage_timer = 0.0;

                // Laser beam coins - higher value for continuous damage
                spawn_weapon_coins(&mut commands, target_transform.translation.truncate(), 120, &mut meshes, &mut materials);
            }
        } else {
            // Reset laser timer when not being hit
            target.laser_damage_timer = 0.0;
        }
    }
}

/// Coin magnet effect - attracts coins to player
fn coin_magnet_effect(
    upgrades: Res<PlayerUpgrades>,
    player_query: Query<&Transform, With<Player>>,
    mut coin_query: Query<(&Transform, &mut LinearVelocity), (With<Coin>, Without<Player>)>,
) {
    if !upgrades.coin_magnet {
        return;
    }

    if let Ok(player_transform) = player_query.single() {
        for (coin_transform, mut coin_velocity) in &mut coin_query {
            let distance = player_transform.translation.distance(coin_transform.translation);

            if distance < 100.0 { // Magnet range
                let direction = (player_transform.translation - coin_transform.translation).normalize();
                let magnet_force = 300.0 / (distance * 0.1 + 1.0); // Stronger when closer

                coin_velocity.x += direction.x * magnet_force * 0.016; // Approximate delta time
                coin_velocity.y += direction.y * magnet_force * 0.016;
            }
        }
    }
}

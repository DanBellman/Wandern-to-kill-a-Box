//! Handle player input and translate it into movement using bevy_enhanced_input.
//! This module provides a character controller system that governs player movement.
//!
//! The movement system has the following logic:
//! - Use bevy_enhanced_input actions to capture directional input (WASD/gamepad).
//! - Apply movement based on input values and maximum speed with upgrade multipliers.
//! - Constrain the player within screen boundaries.
//!
//! This implementation is designed for 2D horizontal movement in a side-scrolling game.
//! The system supports both keyboard (A/D keys) and gamepad (left stick) input.

use crate::demo::player::Player;
use avian2d::prelude::*;
use bevy::{platform::collections::HashSet, prelude::*};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_enhanced_input::prelude::*;
use std::any::TypeId;

use crate::{AppSystems, PausableSystems, demo::shop::shop::PlayerUpgrades};

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<DefaultInputContext>();
    app.add_observer(bind_default_inputs);

    app.init_resource::<BlocksInput>();
    app.register_type::<BlocksInput>();
    app.add_systems(
        PreUpdate,
        update_player_input_binding.run_if(resource_changed::<BlocksInput>),
    );

    app.add_systems(
        Update,
        (apply_enhanced_movement, apply_screen_limits)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementSpeed {
    pub max_speed: f32,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        Self { max_speed: 400.0 }
    }
}

fn apply_enhanced_movement(
    upgrades: Res<PlayerUpgrades>,
    mut movement_query: Query<(&MovementSpeed, &mut LinearVelocity), With<Player>>,
    move_action: Single<&Action<Move>>,
) {
    for (movement_speed, mut velocity) in &mut movement_query {
        let move_input = **move_action;
        let speed_multiplier = 1.0 + (upgrades.speed_boost as f32 * 0.3);

        velocity.x = movement_speed.max_speed * speed_multiplier * move_input.x;
        velocity.y = 0.0;
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

        let left_boundary = -half_width + 16.0;
        let right_boundary = half_width - 16.0;

        for mut player_transform in &mut player_query {
            if player_transform.translation.x < left_boundary {
                player_transform.translation.x = left_boundary;
            }
            if player_transform.translation.x > right_boundary {
                player_transform.translation.x = right_boundary;
            }
        }
    }
}

//new
#[derive(Debug, InputAction)]
#[action_output(Vec3)]
pub(crate) struct Move;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub(crate) struct Jump;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub(crate) struct Interact;

#[derive(Debug, InputAction)]
#[action_output(Vec2)]
pub(crate) struct Rotate;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub(crate) struct PickupProp;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub(crate) struct DropProp;

#[derive(Debug, Component, Default)]
pub struct DefaultInputContext;

#[derive(Resource, Default, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct BlocksInput(HashSet<TypeId>);

fn bind_default_inputs(trigger: Trigger<OnAdd, DefaultInputContext>, mut commands: Commands) {
    const DEFAULT_SENSITIVITY: f32 = 0.002;
    commands
        .entity(trigger.target())
        .insert(actions!(DefaultInputContext[
            (
                Action::<Move>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Bindings::spawn((
                    Cardinal::wasd_keys(),
                    Axial::left_stick()
                ))
            ),
            (Action::<Jump>::new(), bindings![KeyCode::Space, GamepadButton::South]),
            (Action::<Interact>::new(), bindings![KeyCode::KeyE, GamepadButton::South]),
            (Action::<Rotate>::new(),Negate::all(), Scale::splat(DEFAULT_SENSITIVITY),
                Bindings::spawn((Spawn(Binding::mouse_motion()), Axial::right_stick()))),
            (Action::<PickupProp>::new(), bindings![MouseButton::Left, GamepadButton::East]),
            (Action::<DropProp>::new(), bindings![MouseButton::Right, GamepadButton::East]),
        ]));
}

fn update_player_input_binding(
    player: Single<Entity, With<Player>>,
    blocks_input: Res<BlocksInput>,
    mut commands: Commands,
) {
    if blocks_input.is_empty() {
        commands.entity(*player).insert(DefaultInputContext);
    } else {
        commands
            .entity(*player)
            .remove_with_requires::<DefaultInputContext>()
            .despawn_related::<Actions<DefaultInputContext>>();
    }
}

// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]
// Increase recursion limit for Bevy async compilation
#![recursion_limit = "256"]

mod asset_tracking;
mod audio;
mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod screens;
mod theme;

use bevy::{
    asset::AssetMetaCheck, 
    prelude::*, 
    render::camera::ScalingMode,
    diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin, SystemInformationDiagnosticsPlugin},
    core_pipeline::tonemapping::Tonemapping,
};
use avian2d::prelude::*;
use iyes_perf_ui::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "2d Box Slayer".to_string(),
                        fit_canvas_to_parent: true,
                        present_mode: bevy::window::PresentMode::Immediate, // No VSync, no frame rate cap
                        ..default()
                    }),
                    ..default()
                }),
            PhysicsPlugins::default(),
            FrameTimeDiagnosticsPlugin { max_history_length: 1, smoothing_factor: 0.0 },
            EntityCountDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin::default(),
        ));

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            //audio::plugin,
            demo::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
            PerfUiPlugin::default(),
        ));

        // Add perf UI toggle system
        app.add_systems(Update, toggle_perf_ui);

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, (spawn_camera, spawn_perf_ui));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("HDR Camera"),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // Fixed scale - window resize crops/letterboxes instead of showing more world
            scaling_mode: ScalingMode::FixedVertical { viewport_height: 600.0 },
            ..OrthographicProjection::default_2d()
        }),
        // Enable HDR rendering
        Camera {
            hdr: true,
            ..default()
        },
        // Configure tonemapping for HDR
        Tonemapping::AcesFitted,
    ));
}

fn spawn_perf_ui(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}

fn toggle_perf_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut perf_ui_query: Query<&mut Visibility, With<PerfUiRoot>>,
) {
    if keys.just_pressed(KeyCode::F12) {
        for mut visibility in perf_ui_query.iter_mut() {
            *visibility = match *visibility {
                Visibility::Visible => Visibility::Hidden,
                _ => Visibility::Visible,
            };
        }
    }
}

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

use avian2d::prelude::*;
use bevy::{
    asset::AssetMetaCheck,
    core_pipeline::tonemapping::Tonemapping,
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    render::camera::ScalingMode,
};
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
                    mode: bevy::asset::AssetMode::Unprocessed,
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
            FrameTimeDiagnosticsPlugin {
                max_history_length: 1,
                smoothing_factor: 0.0,
            },
            EntityCountDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin::default(),
        ));

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
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
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 600.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        // Enable HDR rendering
        Camera {
            hdr: true,
            ..default()
        },
        // Configure tonemapping for HDR
        Tonemapping::None,
    ));
}

// bitflags! {
//     struct RenderLayer: u32 {
//         /// Used implicitly by all entities without a `RenderLayers` component.
//         /// Our world model camera and all objects other than the player are on this layer.
//         /// The light source belongs to both layers.
//         const DEFAULT = 0b00000001;
//         /// Used by the view model camera and the player's arm.
//         /// The light source belongs to both layers.
//         const VIEW_MODEL = 0b00000010;
//         /// Since we use multiple cameras, we need to be explicit about
//         /// which one is allowed to render particles.
//         const PARTICLES = 0b00000100;
//         /// 3D gizmos. These need to be rendered only by a 3D camera, otherwise the UI camera will render them in a buggy way.
//         /// Specifically, the UI camera is a 2D camera, which by default is placed at a far away Z position,
//         /// so it will effectively render a very zoomed out view of the scene in the center of the screen.
//         const GIZMO3 = 0b0001000;
//     }
// }

// impl From<RenderLayer> for RenderLayers {
//     fn from(layer: RenderLayer) -> Self {
//         // Render layers are just vectors of ints, so we convert each active bit to an int.
//         RenderLayers::from_iter(layer.iter().map(|l| (l.bits() >> 1) as usize))
//     }
// }

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

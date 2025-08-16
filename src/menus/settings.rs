//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*, window::PresentMode};

use crate::{menus::Menu, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.register_type::<GlobalVolumeLabel>();
    app.register_type::<FramerateLimitLabel>();
    app.init_resource::<FramerateLimitSettings>();
    app.add_systems(
        Update,
        (
            update_global_volume_label,
            update_framerate_limit_label,
            apply_framerate_limit_changes,
        ).run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            global_volume_widget(),
            (
                widget::label("Framerate Limit"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            framerate_limit_widget(),
        ],
    )
}

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_global_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_global_volume),
        ],
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_global_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_global_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn go_back_on_click(
    _: Trigger<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FramerateOption {
    Unlimited,
    Fps30,
    Fps60,
    Fps120,
    Fps140,
}

impl FramerateOption {
    fn display_name(&self) -> &'static str {
        match self {
            FramerateOption::Unlimited => "Unlimited",
            FramerateOption::Fps30 => "30 FPS",
            FramerateOption::Fps60 => "60 FPS",
            FramerateOption::Fps120 => "120 FPS",
            FramerateOption::Fps140 => "140 FPS",
        }
    }

    fn to_present_mode(&self) -> PresentMode {
        match self {
            FramerateOption::Unlimited => PresentMode::Immediate,
            FramerateOption::Fps30 => PresentMode::Fifo,
            FramerateOption::Fps60 => PresentMode::Fifo,
            FramerateOption::Fps120 => PresentMode::Mailbox,
            FramerateOption::Fps140 => PresentMode::Mailbox,
        }
    }

    fn all_options() -> [FramerateOption; 5] {
        [
            FramerateOption::Unlimited,
            FramerateOption::Fps30,
            FramerateOption::Fps60,
            FramerateOption::Fps120,
            FramerateOption::Fps140,
        ]
    }

    fn next(&self) -> FramerateOption {
        let options = Self::all_options();
        let current_index = options.iter().position(|&x| x == *self).unwrap_or(0);
        let next_index = (current_index + 1) % options.len();
        options[next_index]
    }

    fn previous(&self) -> FramerateOption {
        let options = Self::all_options();
        let current_index = options.iter().position(|&x| x == *self).unwrap_or(0);
        let prev_index = if current_index == 0 {
            options.len() - 1
        } else {
            current_index - 1
        };
        options[prev_index]
    }
}

#[derive(Resource)]
struct FramerateLimitSettings {
    current: FramerateOption,
    needs_update: bool,
}

impl Default for FramerateLimitSettings {
    fn default() -> Self {
        Self {
            current: FramerateOption::Unlimited,
            needs_update: false,
        }
    }
}

fn framerate_limit_widget() -> impl Bundle {
    (
        Name::new("Framerate Limit Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("<", decrease_framerate_limit),
            (
                Name::new("Current Framerate Limit"),
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    min_width: Px(80.0),
                    ..default()
                },
                children![(widget::label(""), FramerateLimitLabel)],
            ),
            widget::button_small(">", increase_framerate_limit),
        ],
    )
}

fn decrease_framerate_limit(
    _: Trigger<Pointer<Click>>,
    mut framerate_settings: ResMut<FramerateLimitSettings>,
) {
    framerate_settings.current = framerate_settings.current.previous();
    framerate_settings.needs_update = true;
}

fn increase_framerate_limit(
    _: Trigger<Pointer<Click>>,
    mut framerate_settings: ResMut<FramerateLimitSettings>,
) {
    framerate_settings.current = framerate_settings.current.next();
    framerate_settings.needs_update = true;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FramerateLimitLabel;

fn update_framerate_limit_label(
    framerate_settings: Res<FramerateLimitSettings>,
    mut label: Single<&mut Text, With<FramerateLimitLabel>>,
) {
    if framerate_settings.is_changed() {
        label.0 = framerate_settings.current.display_name().to_string();
    }
}

fn apply_framerate_limit_changes(
    mut framerate_settings: ResMut<FramerateLimitSettings>,
    mut windows: Query<&mut Window>,
) {
    if framerate_settings.needs_update {
        framerate_settings.needs_update = false;

        for mut window in &mut windows {
            window.present_mode = framerate_settings.current.to_present_mode();
        }
    }
}

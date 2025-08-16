//! The title screen that appears after the splash screen.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Title),
        (spawn_title_background, open_main_menu),
    );
    app.add_systems(OnExit(Screen::Title), close_menu);
    app.add_systems(
        Update,
        update_title_background_size.run_if(in_state(Screen::Title)),
    );
}

#[derive(Component)]
struct TitleBackground;

fn spawn_title_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    // Calculate the world space dimensions that will be visible
    // Camera uses FixedVertical scaling with 600px height
    let world_height = 600.0;
    let window_aspect = window.width() / window.height();
    let world_width = world_height * window_aspect;

    commands.spawn((
        Name::new("Title Background"),
        TitleBackground,
        Sprite {
            image: asset_server.load("Title.exr"),
            custom_size: Some(Vec2::new(world_width, world_height)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)), // Behind everything else
        StateScoped(Screen::Title),
    ));
}

fn update_title_background_size(
    mut background_query: Query<&mut Sprite, With<TitleBackground>>,
    window_query: Query<&Window, (With<PrimaryWindow>, Changed<Window>)>,
) {
    // Only run when window has changed
    for window in window_query.iter() {
        let world_height = 600.0;
        let window_aspect = window.width() / window.height();
        let world_width = world_height * window_aspect;

        // Update the background sprite
        for mut sprite in background_query.iter_mut() {
            sprite.custom_size = Some(Vec2::new(world_width, world_height));
        }
    }
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

//! Simple HUD system

use bevy::prelude::*;
use bevy::text::FontSmoothing;
use crate::{screens::Screen, AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameTimer>();
    app.insert_resource(CoinBuffer::new());
    app.add_systems(OnEnter(Screen::Gameplay), (spawn_hud, start_game_timer));
    app.add_systems(Update, (
        update_timer_display,
        update_coin_buffer,
        update_buffer_display,
    ).in_set(AppSystems::Update).in_set(PausableSystems));
}

#[derive(Component)]
pub struct GameHud;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct BufferBar;

#[derive(Component)]
pub struct BufferBarFill;

#[derive(Component)]
pub struct TimeText;

#[derive(Resource, Default)]
pub struct GameTimer {
    pub elapsed: f32,
}

#[derive(Resource, Default)]
pub struct CoinBuffer {
    pub current: f32,
    pub max: f32,
}

impl CoinBuffer {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            max: 20.0,
        }
    }

    pub fn add_coin(&mut self) {
        self.current = (self.current + 1.0).min(self.max);
    }

    pub fn drain(&mut self, delta_time: f32) {
        let drain_rate = 2.0; // Coins per second recovery rate
        self.current = (self.current - drain_rate * delta_time).max(0.0);
    }

    pub fn get_percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

/// Spawns the HUD when entering gameplay
fn spawn_hud(mut commands: Commands) {
    // Left text
    commands.spawn((
        Name::new("Coin Text"),
        GameHud,
        ScoreText,
        Text2d::new("Money: 0"),
        TextFont {
            font_size: 32.0,
            ..default()
        }
        .with_font_smoothing(FontSmoothing::None),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(-200.0, -280.0, 10.0)),
        StateScoped(Screen::Gameplay),
    ));

    // Center text
    commands.spawn((
        Name::new("Time Text"),
        GameHud,
        TimeText,
        Text2d::new("00:00"),
        TextFont {
            font_size: 28.0,
            ..default()
        }
        .with_font_smoothing(FontSmoothing::None),
        TextColor(Color::srgb(0.8, 0.2, 0.2)),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(0.0, -280.0, 10.0)),
        StateScoped(Screen::Gameplay),
    ));

    // Buffer Bar (right side)
    commands.spawn((
        Name::new("Buffer Bar Container"),
        GameHud,
        BufferBar,
        Sprite::from_color(Color::srgb(0.3, 0.3, 0.3), Vec2::new(120.0, 20.0)),
        Transform::from_translation(Vec3::new(200.0, -280.0, 10.0)),
        StateScoped(Screen::Gameplay),
    )).with_children(|parent| {
        // Buffer bar fill
        parent.spawn((
            Name::new("Buffer Bar Fill"),
            BufferBarFill,
            Sprite::from_color(Color::srgb(0.0, 0.8, 0.2), Vec2::new(0.0, 18.0)),
            Transform::from_translation(Vec3::new(-60.0, 0.0, 1.0)),
        ));
    });

    commands.spawn((
        Name::new("Buffer Text Label"),
        GameHud,
        Text2d::new("Buffer"),
        TextFont {
            font_size: 18.0,
            ..default()
        }
        .with_font_smoothing(FontSmoothing::None),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(200.0, -250.0, 10.0)),
        StateScoped(Screen::Gameplay),
    ));
}

/// Start the game timer when entering gameplay
fn start_game_timer(mut timer: ResMut<GameTimer>) {
    timer.elapsed = 0.0;
}

/// Update the timer display
fn update_timer_display(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut text_query: Query<&mut Text2d, With<TimeText>>,
) {
    timer.elapsed += time.delta_secs();

    let minutes = (timer.elapsed / 60.0) as u32;
    let seconds = (timer.elapsed % 60.0) as u32;

    for mut text in &mut text_query {
        **text = format!("{:02}:{:02}", minutes, seconds);
    }
}

/// Update the coin buffer (drain over time)
fn update_coin_buffer(
    time: Res<Time>,
    mut buffer: ResMut<CoinBuffer>,
) {
    buffer.drain(time.delta_secs());
}

/// Update the buffer bar fill
fn update_buffer_display(
    buffer: Res<CoinBuffer>,
    mut bar_query: Query<(&mut Sprite, &mut Transform), With<BufferBarFill>>,
) {
    if buffer.is_changed() {
        let percentage = buffer.get_percentage();
        let max_width = 120.0;
        let fill_width = max_width * percentage;

        for (mut sprite, mut transform) in &mut bar_query {
            sprite.custom_size = Some(Vec2::new(fill_width, 18.0));
            let offset_x = -60.0 + (fill_width / 2.0);
            transform.translation.x = offset_x;
        }
    }
}

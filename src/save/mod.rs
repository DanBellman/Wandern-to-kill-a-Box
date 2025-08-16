use bevy::prelude::*;
use moonshine_save::prelude::*;

pub mod ui;

const SAVES_DIR: &str = "saves";

/// Plugin that handles save/load functionality using moonshine-save
pub fn plugin(app: &mut App) {
    app.add_plugins((SavePlugin, LoadPlugin))
        .init_resource::<SaveSettings>()
        .add_event::<DeleteSaveEvent>()
        .register_type::<crate::demo::player::shooting::Money>()
        .register_type::<crate::demo::shop::shop::PlayerUpgrades>()
        .register_type::<Transform>()
        .add_systems(Startup, setup_save_system)
        .add_systems(
            Update,
            (
                handle_delete_events,
                show_save_notifications,
                ui::save_ui_system,
            )
                .run_if(in_state(crate::screens::Screen::Gameplay)),
        )
        .add_systems(Update, (handle_save_requests, handle_load_requests));
}

/// Marker component for entities that should be saved
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GameSave;

/// Save system configuration
#[derive(Resource, Default)]
pub struct SaveSettings {
    pub quick_save_enabled: bool,
}

/// Resource-based save request (moonshine-save style)
#[derive(Resource)]
pub struct SaveRequest {
    pub path: String,
    pub slot_name: String,
}

/// Resource-based load request (moonshine-save style)
#[derive(Resource)]
pub struct LoadRequest {
    pub path: String,
}

fn get_save_path(filename: &str) -> String {
    if let Err(e) = std::fs::create_dir_all(SAVES_DIR) {
        warn!("Failed to create saves directory: {}", e);
    }

    format!("{}/{}", SAVES_DIR, filename)
}

#[derive(Event)]
pub struct DeleteSaveEvent {
    pub slot_id: u32,
}

// Simplified - just handle file deletion directly
pub fn handle_delete_events(mut delete_events: EventReader<DeleteSaveEvent>) {
    for event in delete_events.read() {
        let filename = format!("save_{:03}.ron", event.slot_id);
        let full_path = get_save_path(&filename);
        if let Err(e) = std::fs::remove_file(&full_path) {
            warn!("Failed to delete save file {}: {}", full_path, e);
        } else {
            info!("Deleted save file: {}", full_path);
        }
    }
}

/// Quick save system for Box Slayer
pub fn quick_save(mut commands: Commands) {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    commands.insert_resource(SaveRequest {
        path: get_save_path("quicksave.ron"),
        slot_name: format!("Quick Save - {}", timestamp),
    });

    info!("Quick save created");
}

/// Helper function to mark entities as saveable
pub fn mark_as_saveable(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).insert(Save);
    }
}

/// Helper function to initialize save system
pub fn setup_save_system(mut commands: Commands) {
    commands.insert_resource(SaveSettings {
        quick_save_enabled: true,
    });
    commands.insert_resource(SaveNotifications::default());

    // Spawn a dummy entity marked for saving to prevent moonshine-save panics
    // This ensures there's always at least one entity to save
    commands.spawn((GameSave, Save, Name::new("SaveSystemMarker")));
}

/// Resource for tracking save notifications
#[derive(Resource, Default)]
pub struct SaveNotifications {
    pub notifications: Vec<SaveNotification>,
}

#[derive(Clone)]
pub struct SaveNotification {
    pub message: String,
    pub timestamp: f64,
    pub duration: f64,
}

/// Simple notification system - shows feedback when save/load resources are detected
pub fn show_save_notifications(
    mut save_notifications: ResMut<SaveNotifications>,
    save_request: Option<Res<SaveRequest>>,
    load_request: Option<Res<LoadRequest>>,
    time: Res<Time>,
) {
    // Add notifications when save/load requests are detected
    if let Some(request) = save_request {
        if request.is_added() {
            save_notifications.notifications.push(SaveNotification {
                message: format!("Saving: {}", request.slot_name),
                timestamp: time.elapsed_secs_f64(),
                duration: 2.0,
            });
        }
    }

    if let Some(_request) = load_request {
        if _request.is_added() {
            save_notifications.notifications.push(SaveNotification {
                message: "Loading game...".to_string(),
                timestamp: time.elapsed_secs_f64(),
                duration: 2.0,
            });
        }
    }

    // Remove expired notifications
    let current_time = time.elapsed_secs_f64();
    save_notifications
        .notifications
        .retain(|notification| current_time - notification.timestamp < notification.duration);
}

#[derive(Clone, Debug)]
pub struct SaveFileInfo {
    pub path: String,
    pub display_name: String,
    pub timestamp: String,
    pub turn: Option<u32>,
}

/// Get list of all save files with metadata
pub fn get_save_files() -> Vec<SaveFileInfo> {
    let mut save_files = Vec::new();

    // Create saves directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(SAVES_DIR) {
        warn!("Failed to create saves directory: {}", e);
        return save_files;
    }

    if let Ok(entries) = std::fs::read_dir(SAVES_DIR) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".ron")
                    && (filename.starts_with("save_")
                        || filename.starts_with("autosave_")
                        || filename == "quicksave.ron")
                {
                    // Only include files that actually exist and have content
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.len() > 0 {
                            let modified_time = metadata
                                .modified()
                                .ok()
                                .map(|time| {
                                    let datetime = std::time::SystemTime::now()
                                        .duration_since(time)
                                        .map(|duration| {
                                            let seconds_ago = duration.as_secs();
                                            if seconds_ago < 60 {
                                                "Just now".to_string()
                                            } else if seconds_ago < 3600 {
                                                format!("{} minutes ago", seconds_ago / 60)
                                            } else if seconds_ago < 86400 {
                                                format!("{} hours ago", seconds_ago / 3600)
                                            } else {
                                                format!("{} days ago", seconds_ago / 86400)
                                            }
                                        })
                                        .unwrap_or_else(|_| "Recently".to_string());
                                    datetime
                                })
                                .unwrap_or_else(|| "Unknown time".to_string());

                            let display_name = if filename == "quicksave.ron" {
                                "Quick Save".to_string()
                            } else if filename.starts_with("autosave_") {
                                // Extract turn number from autosave filename
                                let parts: Vec<&str> = filename.split('_').collect();
                                if parts.len() >= 3 {
                                    format!("Autosave Turn {}", parts[2].replace(".ron", ""))
                                } else {
                                    filename.replace(".ron", "")
                                }
                            } else {
                                filename.replace(".ron", "").replace("save_", "Slot ")
                            };

                            save_files.push(SaveFileInfo {
                                path: get_save_path(filename),
                                display_name,
                                timestamp: modified_time,
                                turn: extract_turn_from_filename(filename),
                            });
                        }
                    }
                }
            }
        }
    }

    save_files.sort_by(|a, b| {
        if a.path == "quicksave.ron" {
            return std::cmp::Ordering::Less;
        }
        if b.path == "quicksave.ron" {
            return std::cmp::Ordering::Greater;
        }
        if a.path.starts_with("autosave_") && !b.path.starts_with("autosave_") {
            return std::cmp::Ordering::Less;
        }
        if b.path.starts_with("autosave_") && !a.path.starts_with("autosave_") {
            return std::cmp::Ordering::Greater;
        }

        a.path.cmp(&b.path)
    });
    save_files
}

fn extract_turn_from_filename(_filename: &str) -> Option<u32> {
    // Box Slayer doesn't have turns, but keep the function for compatibility
    None
}

/// Handle save requests
fn handle_save_requests(
    mut commands: Commands,
    save_request: Option<Res<SaveRequest>>,
    money: Res<crate::demo::player::shooting::Money>,
    upgrades: Res<crate::demo::shop::shop::PlayerUpgrades>,
) {
    if let Some(request) = save_request {
        if request.is_added() {
            // Simple save implementation - just save the money and upgrades as JSON
            let save_data = serde_json::json!({
                "money": money.amount,
                "upgrades": {
                    "rapid_fire": upgrades.rapid_fire,
                    "uzi": upgrades.uzi,
                    "spread_shot": upgrades.spread_shot,
                    "laser_beam": upgrades.laser_beam,
                    "sniper": upgrades.sniper,
                    "bazooka": upgrades.bazooka,
                    "hammer": upgrades.hammer,
                    "sword": upgrades.sword,
                    "speed_boost": upgrades.speed_boost,
                    "coin_magnet": upgrades.coin_magnet,
                    "current_weapon": format!("{:?}", upgrades.current_weapon),
                    "buffer_level": upgrades.buffer_level,
                }
            });

            if let Err(e) = std::fs::write(&request.path, save_data.to_string()) {
                error!("Failed to save game to {}: {}", request.path, e);
            } else {
                info!("Game saved to: {}", request.path);
            }

            commands.remove_resource::<SaveRequest>();
        }
    }
}

/// Handle load requests
fn handle_load_requests(
    mut commands: Commands,
    load_request: Option<Res<LoadRequest>>,
    mut money: ResMut<crate::demo::player::shooting::Money>,
    mut upgrades: ResMut<crate::demo::shop::shop::PlayerUpgrades>,
) {
    if let Some(request) = load_request {
        if request.is_added() {
            if let Ok(data) = std::fs::read_to_string(&request.path) {
                if let Ok(save_data) = serde_json::from_str::<serde_json::Value>(&data) {
                    // Load money
                    if let Some(saved_money) = save_data["money"].as_u64() {
                        money.amount = saved_money as u32;
                    }

                    // Load upgrades
                    if let Some(saved_upgrades) = save_data["upgrades"].as_object() {
                        upgrades.rapid_fire =
                            saved_upgrades["rapid_fire"].as_bool().unwrap_or(false);
                        upgrades.uzi = saved_upgrades["uzi"].as_bool().unwrap_or(false);
                        upgrades.spread_shot =
                            saved_upgrades["spread_shot"].as_bool().unwrap_or(false);
                        upgrades.laser_beam =
                            saved_upgrades["laser_beam"].as_bool().unwrap_or(false);
                        upgrades.sniper = saved_upgrades["sniper"].as_bool().unwrap_or(false);
                        upgrades.bazooka = saved_upgrades["bazooka"].as_bool().unwrap_or(false);
                        upgrades.hammer = saved_upgrades["hammer"].as_bool().unwrap_or(false);
                        upgrades.sword = saved_upgrades["sword"].as_bool().unwrap_or(false);
                        upgrades.speed_boost =
                            saved_upgrades["speed_boost"].as_u64().unwrap_or(0) as u32;
                        upgrades.coin_magnet =
                            saved_upgrades["coin_magnet"].as_bool().unwrap_or(false);
                        upgrades.buffer_level =
                            saved_upgrades["buffer_level"].as_u64().unwrap_or(1) as u32;

                        // Parse weapon type
                        if let Some(weapon_str) = saved_upgrades["current_weapon"].as_str() {
                            upgrades.current_weapon = match weapon_str {
                                "RapidFire" => crate::demo::shop::shop::WeaponType::RapidFire,
                                "Uzi" => crate::demo::shop::shop::WeaponType::Uzi,
                                "SpreadShot" => crate::demo::shop::shop::WeaponType::SpreadShot,
                                "LaserBeam" => crate::demo::shop::shop::WeaponType::LaserBeam,
                                "Sniper" => crate::demo::shop::shop::WeaponType::Sniper,
                                "Bazooka" => crate::demo::shop::shop::WeaponType::Bazooka,
                                "Hammer" => crate::demo::shop::shop::WeaponType::Hammer,
                                "Sword" => crate::demo::shop::shop::WeaponType::Sword,
                                _ => crate::demo::shop::shop::WeaponType::Normal,
                            };
                        }
                    }

                    info!("Game loaded from: {}", request.path);
                } else {
                    error!("Failed to parse save file: {}", request.path);
                }
            } else {
                error!("Failed to read save file: {}", request.path);
            }

            commands.remove_resource::<LoadRequest>();
        }
    }
}

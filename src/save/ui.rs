use super::{
    LoadRequest, SaveNotification, SaveNotifications, SaveRequest, get_save_files, get_save_path,
};
use crate::{menus::Menu, theme::widget};
use bevy::prelude::*;

/// Simple UI system for save/load functionality
pub fn save_ui_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut save_notifications: ResMut<SaveNotifications>,
    time: Res<Time>,
) {
    // Quick save with F5
    if keys.just_pressed(KeyCode::F5) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        commands.insert_resource(SaveRequest {
            path: get_save_path("quicksave.ron"),
            slot_name: format!("Quick Save - {}", timestamp),
        });

        save_notifications.notifications.push(SaveNotification {
            message: "Quick Save Created (F5)".to_string(),
            timestamp: time.elapsed_secs_f64(),
            duration: 2.0,
        });
    }

    // Quick load with F9
    if keys.just_pressed(KeyCode::F9) {
        let quicksave_path = get_save_path("quicksave.ron");
        if std::path::Path::new(&quicksave_path).exists() {
            commands.insert_resource(LoadRequest {
                path: quicksave_path,
            });

            save_notifications.notifications.push(SaveNotification {
                message: "Quick Load (F9)".to_string(),
                timestamp: time.elapsed_secs_f64(),
                duration: 2.0,
            });
        } else {
            save_notifications.notifications.push(SaveNotification {
                message: "No Quick Save Found".to_string(),
                timestamp: time.elapsed_secs_f64(),
                duration: 2.0,
            });
        }
    }
}

/// Create a save game slot button
pub fn save_slot_button(slot_id: u32, label: &str) -> impl Bundle {
    widget::button(
        label,
        move |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
            let filename = format!("save_{:03}.ron", slot_id);
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            commands.insert_resource(SaveRequest {
                path: get_save_path(&filename),
                slot_name: format!("Save Slot {} - {}", slot_id, timestamp),
            });
        },
    )
}

/// Create a load game slot button
pub fn load_slot_button(save_path: String, label: &str) -> impl Bundle {
    widget::button(
        label,
        move |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.insert_resource(LoadRequest {
                path: save_path.clone(),
            });
        },
    )
}

/// Save/Load menu component
#[derive(Component)]
pub struct SaveLoadMenu;

/// Spawn save/load menu UI
pub fn spawn_save_load_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Save/Load Menu"),
        GlobalZIndex(3),
        StateScoped(Menu::SaveLoad),
        SaveLoadMenu,
        children![
            widget::header("Save / Load Game"),
            // Save section
            widget::label("Save Game:"),
            save_slot_button(1, "Save Slot 1"),
            save_slot_button(2, "Save Slot 2"),
            save_slot_button(3, "Save Slot 3"),
            // Quick save button
            widget::button(
                "Quick Save (F5)",
                |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    commands.insert_resource(SaveRequest {
                        path: get_save_path("quicksave.ron"),
                        slot_name: format!("Quick Save - {}", timestamp),
                    });
                }
            ),
            // Load section header
            widget::label("Load Game:"),
            // Dynamic load buttons will be added here based on existing saves
            load_buttons_container(),
            // Back button
            widget::button(
                "Back",
                |_trigger: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>| {
                    next_menu.set(Menu::Pause);
                }
            ),
        ],
    ));
}

/// Container for dynamically generated load buttons
fn load_buttons_container() -> impl Bundle {
    let save_files = get_save_files();

    (
        Name::new("Load Buttons Container"),
        Node::default(),
        children![
            // Quick load button if quicksave exists
            widget::button(
                "Quick Load (F9)",
                |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.insert_resource(LoadRequest {
                        path: get_save_path("quicksave.ron"),
                    });
                }
            ),
            // Show message if no saves
            widget::label(if save_files.is_empty() {
                "No saved games found"
            } else {
                "Save files available"
            }),
        ],
    )
}

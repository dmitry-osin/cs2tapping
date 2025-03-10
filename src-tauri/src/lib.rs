// CS2 Tapping Helper — Application entry point and Tauri commands
// Author: Dmitry Osin <d@osin.pro>

pub mod keyboard_hook;
pub mod settings;
pub mod tray;

use keyboard_hook::{set_active, start_keyboard_hook, HookState};
use settings::load_settings;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

pub struct AppState {
    pub hook_state: Arc<Mutex<HookState>>,
}

mod commands {
    use super::{settings, AppState};
    use settings::Settings;
    use tauri::{AppHandle, Manager, State};

    #[tauri::command]
    pub fn get_settings(state: State<'_, AppState>) -> Settings {
        state.hook_state.lock().unwrap().settings.clone()
    }

    #[tauri::command]
    pub fn update_settings(
        new_settings: Settings,
        state: State<'_, AppState>,
    ) -> Result<(), String> {
        settings::save_settings(&new_settings)?;
        state.hook_state.lock().unwrap().settings = new_settings;
        Ok(())
    }

    #[tauri::command]
    pub fn get_active(state: State<'_, AppState>) -> bool {
        state.hook_state.lock().unwrap().is_active
    }

    #[tauri::command]
    pub fn set_hook_active(
        active: bool,
        app_handle: AppHandle,
        state: State<'_, AppState>,
    ) -> Result<(), String> {
        crate::keyboard_hook::set_active(&app_handle, &state.hook_state, active);
        Ok(())
    }

    #[tauri::command]
    pub fn hide_to_tray(app_handle: AppHandle) {
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.hide();
        }
    }
    #[tauri::command]
    pub fn get_autostart_state(app_handle: AppHandle) -> bool {
        use tauri_plugin_autostart::ManagerExt;
        app_handle.autolaunch().is_enabled().unwrap_or(false)
    }

    #[tauri::command]
    pub fn set_autostart_state(enabled: bool, app_handle: AppHandle) -> Result<(), String> {
        use tauri_plugin_autostart::ManagerExt;
        if enabled {
            app_handle.autolaunch().enable().map_err(|e| e.to_string())
        } else {
            app_handle.autolaunch().disable().map_err(|e| e.to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let loaded = load_settings();
    let start_active = loaded.start_active;
    let start_minimized = loaded.start_minimized;

    let hook_state = Arc::new(Mutex::new(HookState::new(loaded)));

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            hook_state: hook_state.clone(),
        })
        .setup(move |app| {
            tray::build_tray(&app.handle())?;

            start_keyboard_hook(app.handle().clone(), hook_state.clone());

            if start_active {
                set_active(&app.handle(), &hook_state, true);
            }

            if start_minimized {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.hide();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_settings,
            commands::update_settings,
            commands::get_active,
            commands::set_hook_active,
            commands::hide_to_tray,
            commands::get_autostart_state,
            commands::set_autostart_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// CS2 Tapping Helper — System tray module
// Author: Dmitry Osin <d@osin.pro>

use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

fn app_icon() -> Image<'static> {
    Image::from_bytes(include_bytes!("../icons/icon.png"))
        .expect("failed to load tray icon")
}

pub fn update_tray(app: &AppHandle, active: bool) {
    if let Some(tray) = app.tray_by_id("ct-tray") {
        let tooltip = if active {
            "CS2 Tapping — Active"
        } else {
            "CS2 Tapping — Inactive"
        };
        let _ = tray.set_tooltip(Some(tooltip));
    }

    if let Some(toggle_item) = app.try_state::<TrayToggleItem>() {
        let label = if active { "Disable" } else { "Enable" };
        let _ = toggle_item.0.set_text(label);
    }
}

pub struct TrayToggleItem(pub MenuItem<tauri::Wry>);

pub fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let show   = MenuItem::with_id(app, "show",   "Show",   true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "Enable", true, None::<&str>)?;
    let exit   = MenuItem::with_id(app, "exit",   "Exit",   true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &toggle, &exit])?;

    app.manage(TrayToggleItem(toggle));

    TrayIconBuilder::with_id("ct-tray")
        .icon(app_icon())
        .tooltip("CS2 Tapping — Inactive")
        .menu(&menu)
        .on_menu_event(|app, event| handle_menu_event(app, event.id.as_ref()))
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

fn handle_menu_event(app: &AppHandle, id: &str) {
    match id {
        "show" => show_main_window(app),
        "toggle" => {
            use crate::keyboard_hook::set_active;
            if let Some(state) = app.try_state::<crate::AppState>() {
                let new_active = !state.hook_state.lock().unwrap().is_active;
                set_active(app, &state.hook_state, new_active);
            }
        }
        "exit" => app.exit(0),
        _ => {}
    }
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

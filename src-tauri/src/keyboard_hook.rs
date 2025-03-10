// CS2 Tapping Helper — Keyboard hook module
// Author: Dmitry Osin <d@osin.pro>

use rand::Rng;
use rdev::{listen, simulate, Event, EventType, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use crate::settings::Settings;
use crate::tray::update_tray;

pub struct HookState {
    pub is_active: bool,
    pub settings: Settings,
    pub ignore_left_release:    Arc<AtomicBool>,
    pub ignore_right_release:   Arc<AtomicBool>,
    pub ignore_forward_release: Arc<AtomicBool>,
    pub ignore_back_release:    Arc<AtomicBool>,
}

impl HookState {
    pub fn new(settings: Settings) -> Self {
        HookState {
            is_active: false,
            settings,
            ignore_left_release:    Arc::new(AtomicBool::new(false)),
            ignore_right_release:   Arc::new(AtomicBool::new(false)),
            ignore_forward_release: Arc::new(AtomicBool::new(false)),
            ignore_back_release:    Arc::new(AtomicBool::new(false)),
        }
    }
}

pub fn start_keyboard_hook(app_handle: AppHandle, state: Arc<Mutex<HookState>>) {
    thread::spawn(move || {
        if let Err(e) = listen(move |event| {
            handle_event(event, &app_handle, &state);
        }) {
            eprintln!("Keyboard hook error: {:?}", e);
        }
    });
}

pub fn key_to_string(key: &Key) -> String {
    format!("{:?}", key)
}

pub fn string_to_key(s: &str) -> Option<Key> {
    match s {
        "Alt" => Some(Key::Alt),
        "AltGr" => Some(Key::AltGr),
        "Backspace" => Some(Key::Backspace),
        "CapsLock" => Some(Key::CapsLock),
        "ControlLeft" => Some(Key::ControlLeft),
        "ControlRight" => Some(Key::ControlRight),
        "Delete" => Some(Key::Delete),
        "DownArrow" => Some(Key::DownArrow),
        "End" => Some(Key::End),
        "Escape" => Some(Key::Escape),
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "Home" => Some(Key::Home),
        "Insert" => Some(Key::Insert),
        "LeftArrow" => Some(Key::LeftArrow),
        "MetaLeft" => Some(Key::MetaLeft),
        "MetaRight" => Some(Key::MetaRight),
        "NumLock" => Some(Key::NumLock),
        "PageDown" => Some(Key::PageDown),
        "PageUp" => Some(Key::PageUp),
        "Pause" => Some(Key::Pause),
        "PrintScreen" => Some(Key::PrintScreen),
        "Return" => Some(Key::Return),
        "RightArrow" => Some(Key::RightArrow),
        "ScrollLock" => Some(Key::ScrollLock),
        "ShiftLeft" => Some(Key::ShiftLeft),
        "ShiftRight" => Some(Key::ShiftRight),
        "Space" => Some(Key::Space),
        "Tab" => Some(Key::Tab),
        "UpArrow" => Some(Key::UpArrow),
        "BackQuote" => Some(Key::BackQuote),
        "Num1" => Some(Key::Num1),
        "Num2" => Some(Key::Num2),
        "Num3" => Some(Key::Num3),
        "Num4" => Some(Key::Num4),
        "Num5" => Some(Key::Num5),
        "Num6" => Some(Key::Num6),
        "Num7" => Some(Key::Num7),
        "Num8" => Some(Key::Num8),
        "Num9" => Some(Key::Num9),
        "Num0" => Some(Key::Num0),
        "Minus" => Some(Key::Minus),
        "Equal" => Some(Key::Equal),
        "KeyQ" => Some(Key::KeyQ),
        "KeyW" => Some(Key::KeyW),
        "KeyE" => Some(Key::KeyE),
        "KeyR" => Some(Key::KeyR),
        "KeyT" => Some(Key::KeyT),
        "KeyY" => Some(Key::KeyY),
        "KeyU" => Some(Key::KeyU),
        "KeyI" => Some(Key::KeyI),
        "KeyO" => Some(Key::KeyO),
        "KeyP" => Some(Key::KeyP),
        "LeftBracket" => Some(Key::LeftBracket),
        "RightBracket" => Some(Key::RightBracket),
        "KeyA" => Some(Key::KeyA),
        "KeyS" => Some(Key::KeyS),
        "KeyD" => Some(Key::KeyD),
        "KeyF" => Some(Key::KeyF),
        "KeyG" => Some(Key::KeyG),
        "KeyH" => Some(Key::KeyH),
        "KeyJ" => Some(Key::KeyJ),
        "KeyK" => Some(Key::KeyK),
        "KeyL" => Some(Key::KeyL),
        "SemiColon" => Some(Key::SemiColon),
        "Quote" => Some(Key::Quote),
        "BackSlash" => Some(Key::BackSlash),
        "IntlBackslash" => Some(Key::IntlBackslash),
        "KeyZ" => Some(Key::KeyZ),
        "KeyX" => Some(Key::KeyX),
        "KeyC" => Some(Key::KeyC),
        "KeyV" => Some(Key::KeyV),
        "KeyB" => Some(Key::KeyB),
        "KeyN" => Some(Key::KeyN),
        "KeyM" => Some(Key::KeyM),
        "Comma" => Some(Key::Comma),
        "Dot" => Some(Key::Dot),
        "Slash" => Some(Key::Slash),
        "KpReturn" => Some(Key::KpReturn),
        "KpMinus" => Some(Key::KpMinus),
        "KpPlus" => Some(Key::KpPlus),
        "KpMultiply" => Some(Key::KpMultiply),
        "KpDivide" => Some(Key::KpDivide),
        "Kp0" => Some(Key::Kp0),
        "Kp1" => Some(Key::Kp1),
        "Kp2" => Some(Key::Kp2),
        "Kp3" => Some(Key::Kp3),
        "Kp4" => Some(Key::Kp4),
        "Kp5" => Some(Key::Kp5),
        "Kp6" => Some(Key::Kp6),
        "Kp7" => Some(Key::Kp7),
        "Kp8" => Some(Key::Kp8),
        "Kp9" => Some(Key::Kp9),
        "KpDelete" => Some(Key::KpDelete),
        _ => None,
    }
}

fn random_delay_ms(min_ms: u64, max_ms: u64) -> u64 {
    rand::rng().random_range(min_ms..=max_ms)
}

fn fire_counter_strafe(key: Key, delay_ms: u64, ignore_flag: Arc<AtomicBool>) {
    thread::spawn(move || {
        let _ = simulate(&EventType::KeyPress(key));
        thread::sleep(Duration::from_millis(delay_ms));
        ignore_flag.store(true, Ordering::SeqCst);
        let _ = simulate(&EventType::KeyRelease(key));
    });
}

fn handle_event(event: Event, app_handle: &AppHandle, state: &Arc<Mutex<HookState>>) {
    let EventType::KeyRelease(key) = event.event_type else {
        return;
    };

    let key_str = key_to_string(&key);

    let (is_active, settings) = {
        let s = state.lock().unwrap();
        (s.is_active, s.settings.clone())
    };

    if !is_active {
        return;
    }

    let check_ignore = |flag_name: &str| -> bool {
        let s = state.lock().unwrap();
        match flag_name {
            "left"    => s.ignore_left_release.swap(false, Ordering::SeqCst),
            "right"   => s.ignore_right_release.swap(false, Ordering::SeqCst),
            "forward" => s.ignore_forward_release.swap(false, Ordering::SeqCst),
            "back"    => s.ignore_back_release.swap(false, Ordering::SeqCst),
            _         => false,
        }
    };

    let get_flag = |flag_name: &str| -> Arc<AtomicBool> {
        let s = state.lock().unwrap();
        match flag_name {
            "left"    => Arc::clone(&s.ignore_left_release),
            "right"   => Arc::clone(&s.ignore_right_release),
            "forward" => Arc::clone(&s.ignore_forward_release),
            _         => Arc::clone(&s.ignore_back_release),
        }
    };

    let delay = random_delay_ms(settings.delay_min_ms, settings.delay_max_ms);

    if key_str == settings.key_left {
        if check_ignore("left") { return; }
        if let Some(opposite) = string_to_key(&settings.key_right) {
            fire_counter_strafe(opposite, delay, get_flag("right"));
            let _ = app_handle.emit("counter-strafe", &settings.key_right);
        }
    } else if key_str == settings.key_right {
        if check_ignore("right") { return; }
        if let Some(opposite) = string_to_key(&settings.key_left) {
            fire_counter_strafe(opposite, delay, get_flag("left"));
            let _ = app_handle.emit("counter-strafe", &settings.key_left);
        }
    } else if settings.strafe_forward_back && key_str == settings.key_forward {
        if check_ignore("forward") { return; }
        if let Some(opposite) = string_to_key(&settings.key_back) {
            fire_counter_strafe(opposite, delay, get_flag("back"));
            let _ = app_handle.emit("counter-strafe", &settings.key_back);
        }
    } else if settings.strafe_forward_back && key_str == settings.key_back {
        if check_ignore("back") { return; }
        if let Some(opposite) = string_to_key(&settings.key_forward) {
            fire_counter_strafe(opposite, delay, get_flag("forward"));
            let _ = app_handle.emit("counter-strafe", &settings.key_forward);
        }
    }
}

pub fn set_active(app_handle: &AppHandle, state: &Arc<Mutex<HookState>>, active: bool) {
    state.lock().unwrap().is_active = active;
    update_tray(app_handle, active);
    let _ = app_handle.emit("active-changed", active);
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nosleep::{NoSleep, NoSleepType};
use std::sync::{Arc, Mutex};
use tauri::State;

use tauri::{Manager, SystemTray, SystemTrayEvent};

pub struct NoSleepState {
    enable: bool,
    handle: NoSleep,
}

impl Default for NoSleepState {
    fn default() -> Self {
        NoSleepState {
            enable: false,
            handle: NoSleep::new().unwrap(),
        }
    }
}

fn main() {
    let state = Arc::new(Mutex::new(NoSleepState::default()));

    let tray = SystemTray::new();
    tauri::Builder::default()
        .system_tray(tray)
        .manage(state)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let state: State<'_, Arc<Mutex<NoSleepState>>> = app.try_state().unwrap();
                let mut no_sleep = state.lock().unwrap();
                no_sleep.enable = !no_sleep.enable;
                if no_sleep.enable {
                    println!("sleeping enabled");
                    no_sleep.handle.stop().unwrap();
                } else {
                    println!("sleeping disabled");
                    no_sleep
                    .handle
                    .start(NoSleepType::PreventUserIdleDisplaySleep)
                    .unwrap();
                }
            }
            _ => {}
        })
        // .plugin(tauri_plugin_nosleep::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

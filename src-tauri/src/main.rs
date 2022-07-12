#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nosleep::{NoSleep, NoSleepType};
use std::sync::{Arc, Mutex};
use tauri::State;

use tauri::{Manager, SystemTray, SystemTrayEvent};
use tauri_plugin_autostart::AutoLaunchManager;
use tauri_plugin_autostart::MacosLauncher;

pub struct NoSleepState {
    prevent: bool,
    handle: NoSleep,
}

impl Default for NoSleepState {
    fn default() -> Self {
        NoSleepState {
            prevent: false,
            handle: NoSleep::new().unwrap(),
        }
    }
}

fn main() {
    let state = Arc::new(Mutex::new(NoSleepState::default()));
    let mate_waiting_icon =
        tauri::Icon::Raw(include_bytes!("../icons/mate-waiting/icon.ico").to_vec());
    let tray = SystemTray::new().with_icon(mate_waiting_icon.try_into().unwrap());

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::AppleScript,
            false,
        ))
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let manager: State<'_, AutoLaunchManager> = app.try_state().unwrap();
            manager.enable()?;
            Ok(())
        })
        .system_tray(tray)
        .manage(state)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let mate_waiting_icon =
                    tauri::Icon::Raw(include_bytes!("../icons/mate-waiting/icon.ico").to_vec());
                let mate_drinking_icon =
                    tauri::Icon::Raw(include_bytes!("../icons/icon.ico").to_vec());
                let state: State<'_, Arc<Mutex<NoSleepState>>> = app.try_state().unwrap();
                let mut no_sleep = state.lock().unwrap();
                no_sleep.prevent = !no_sleep.prevent;
                if !no_sleep.prevent {
                    no_sleep.handle.stop().unwrap();
                    app.tray_handle()
                        .set_icon(mate_waiting_icon.clone())
                        .unwrap();
                } else {
                    no_sleep
                        .handle
                        .start(NoSleepType::PreventUserIdleDisplaySleep)
                        .unwrap();
                    app.tray_handle()
                        .set_icon(mate_drinking_icon.clone())
                        .unwrap();
                }
            }
            _ => {}
        })
        // .plugin(tauri_plugin_nosleep::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

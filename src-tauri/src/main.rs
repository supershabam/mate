#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nosleep::{NoSleep, NoSleepType};
use std::sync::{Arc, Mutex};
use tauri::State;

use tauri::{Manager, SystemTray, SystemTrayEvent, CustomMenuItem, SystemTrayMenu};
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
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let tray = SystemTray::new().with_icon(mate_waiting_icon.try_into().unwrap()).with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::AppleScript,
            false,
        ))
        .setup(|app| {
            // do not allow cmd+tab to display application;
            // also do not allow focus to program name in menu bar
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            // force-set auto-launch; TODO make configurable
            let manager: State<'_, AutoLaunchManager> = app.try_state().unwrap();
            manager.enable()?;
            Ok(())
        })
        .system_tray(tray)
        .manage(state)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                  "quit" => {
                    std::process::exit(0);
                  },
                  _ => {}
                }
            },
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

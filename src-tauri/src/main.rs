#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api::shell::open,
    AppHandle,
    CustomMenuItem,
    GlobalShortcutManager,
    Manager,
    SystemTray,
    SystemTrayEvent,
    SystemTrayMenu,
    SystemTrayMenuItem,
    SystemTraySubmenu,
    Window,
};
use copypasta::{ClipboardContext, ClipboardProvider};
use tauri::regex::Regex;
use serde::{Deserialize, Serialize};

// Define the settings struct
#[derive(Serialize, Deserialize)]
struct Settings {
    hotkey: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            hotkey: "Alt+V".to_string(),
        }
    }
}

const LINKS: [(&str, &str, &str); 2] = [
    // GitHub links
    ("open-github-infinitel8p", "InfiniteL8p", "https://github.com/infinitel8p"),
    ("open-github-zaplink", "ZapLink", "https://github.com/infinitel8p/zaplink"),
];

#[tauri::command]
async fn close_splashscreen(window: Window) {
    window
        .get_window("splashscreen")
        .expect("No window labeled 'splashscreen' found")
        .close()
        .unwrap();
}

#[tauri::command]
async fn unhide_window(app: AppHandle) {
    print!("Unhiding window");
    if let Some(window) = app.get_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        println!("Main window not found.");
    }
}

#[tauri::command]
async fn get_hotkey() -> Vec<String> {
    // Get the settings file path
    let config_dir = tauri::api::path::config_dir().expect("Failed to get config directory");
    let settings_dir = config_dir.join("ZapLink");
    let settings_path = settings_dir.join("settings.json");

    // Read the settings
    let settings: Settings = if settings_path.exists() {
        std::fs::read_to_string(&settings_path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    } else {
        Settings::default()
    };

    // Split the hotkey string on '+'
    settings.hotkey.split('+').map(|s| s.to_string()).collect()
}

#[tauri::command]
async fn update_hotkey(
    app_handle: tauri::AppHandle,
    new_hotkey: Vec<String>,
) -> Result<(), String> {
    let hotkey = new_hotkey.join("+");

    // Get the settings file path
    let config_dir =
        tauri::api::path::config_dir().ok_or("Failed to get config directory")?;
    let settings_dir = config_dir.join("ZapLink");
    let settings_path = settings_dir.join("settings.json");

    // Read the current settings
    let mut settings: Settings = if settings_path.exists() {
        std::fs::read_to_string(&settings_path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    } else {
        Settings::default()
    };

    // Unregister the old hotkey
    let mut shortcut_manager = app_handle.global_shortcut_manager();
    if let Err(e) = shortcut_manager.unregister_all() {
        println!("Failed to unregister shortcuts: {}", e);
    }

    // Update the hotkey in settings
    settings.hotkey = hotkey.clone();

    // Write the updated settings back to the file
    std::fs::write(
        &settings_path,
        serde_json::to_string_pretty(&settings).unwrap(),
    )
    .map_err(|e| e.to_string())?;

    // Register the new hotkey
    register_hotkey(&hotkey, &app_handle)?;

    Ok(())
}

fn main() {
    let sub_menu_github = {
        let mut menu = SystemTrayMenu::new();
        for (id, label, _url) in LINKS.iter().filter(|(id, _label, _url)| id.starts_with("open-github")) {
            menu = menu.add_item(CustomMenuItem::new(id.to_string(), label.to_string()));
        }

        SystemTraySubmenu::new("GitHub", menu)
    };

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("visibility-toggle".to_string(), "Show"))
        .add_submenu(sub_menu_github)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![close_splashscreen, unhide_window, get_hotkey, update_hotkey])
        .system_tray(tray)
        .on_system_tray_event(on_system_tray_event)
        .setup(|app| {
            let app_handle = app.handle();
            let main_window = app.get_window("main").unwrap();

            // Clone main_window and app_handle to use inside the closure
            let main_window_clone = main_window.clone();
            let app_handle_clone = app_handle.clone();

            // Intercept the window close event
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    // Prevent the window from closing and hide it instead
                    api.prevent_close();
                    main_window_clone.hide().unwrap();

                    // Update the system tray menu item to "Show"
                    let item_handle = app_handle_clone.tray_handle().get_item("visibility-toggle");
                    item_handle.set_title("Show").unwrap();
                }
            });

            // Get the settings file path
            let config_dir = tauri::api::path::config_dir().expect("Failed to get config directory");
            let settings_dir = config_dir.join("ZapLink");
            std::fs::create_dir_all(&settings_dir).expect("Failed to create settings directory");
            let settings_path = settings_dir.join("settings.json");

            // Read the settings
            let settings: Settings = if settings_path.exists() {
                std::fs::read_to_string(&settings_path)
                    .ok()
                    .and_then(|content| serde_json::from_str(&content).ok())
                    .unwrap_or_default()
            } else {
                let default_settings = Settings::default();
                std::fs::write(&settings_path, serde_json::to_string_pretty(&default_settings).unwrap())
                    .expect("Failed to write default settings");
                default_settings
            };

            // Register the shortcut
            let hotkey = settings.hotkey.clone();
            
            register_hotkey(&hotkey, &app_handle)?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "visibility-toggle" => {
                    let window = app.get_window("main").unwrap();
                    match window.is_visible() {
                        Ok(true) => {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        }
                        Ok(false) => {
                            window.show().unwrap();
                            item_handle.set_title("Hide").unwrap();
                        }
                        Err(_e) => unimplemented!("what kind of errors happen here?"),
                    }
                }
                "quit" => app.exit(0),
                s if s.starts_with("open-") => {
                    if let Some(link) = LINKS.iter().find(|(id, ..)| id == &s) {
                        open(&app.shell_scope(), link.2, None).unwrap();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn register_hotkey(
    hotkey: &str,
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    let mut shortcut_manager = app_handle.global_shortcut_manager();

    let url_pattern = Regex::new(
        r"[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
    )
    .unwrap();

    // Clone the app handle for use inside the closure
    let app_handle_clone = app_handle.clone();

    shortcut_manager
        .register(hotkey, move || {
            let mut ctx = ClipboardContext::new().unwrap();
            let clipboard_content = ctx.get_contents().unwrap_or_default();

            if url_pattern.is_match(&clipboard_content) {
                let url = if clipboard_content.starts_with("http://")
                    || clipboard_content.starts_with("https://")
                {
                    clipboard_content.to_string()
                } else {
                    format!("http://{}", clipboard_content)
                };

                // Open the URL from the clipboard
                open(&app_handle_clone.shell_scope(), &url, None).unwrap();
            } else {
                println!("Clipboard content is not a valid URL.");
            }
        })
        .map_err(|e| e.to_string())
}

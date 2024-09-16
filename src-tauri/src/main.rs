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

const LINKS: [(&str, &str, &str); 2] = [
    // GitHub links
    ("open-github-infinitel8p", "InfiniteL8p", "https://github.com/infinitel8p"),
    ("open-github-zaplink", "ZapLink", "https://github.com/infinitel8p/zaplink"),
];

#[tauri::command]
async fn close_splashscreen(window: Window) {
    window
        .get_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .close()
        .unwrap();
}

#[tauri::command]
async fn unhide_window(window: Window) {
    window.show().unwrap();
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
        .invoke_handler(tauri::generate_handler![close_splashscreen, unhide_window])
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
                    // Prevent the window from closing
                    api.prevent_close();
                    // Hide the window instead
                    main_window_clone.hide().unwrap();

                    // Update the system tray menu item to "Show"
                    let item_handle = app_handle_clone.tray_handle().get_item("visibility-toggle");
                    item_handle.set_title("Show").unwrap();
                }
            });

            let mut shortcut_manager = app.global_shortcut_manager();

            // Register the Alt + V shortcut
            shortcut_manager
                .register("Alt+V", move || {
                    let url_pattern = Regex::new(
                        r"[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
                    )
                    .unwrap();

                    let mut ctx = ClipboardContext::new().unwrap();
                    let clipboard_content = ctx.get_contents().unwrap_or_default();

                    if url_pattern.is_match(&clipboard_content) {
                        let url = if clipboard_content.starts_with("http://") || clipboard_content.starts_with("https://") {
                            clipboard_content.to_string()
                        } else {
                            format!("http://{}", clipboard_content)
                        };

                        // Open the URL from the clipboard
                        open(&app_handle.shell_scope(), &url, None).unwrap();
                    } else {
                        println!("Clipboard content is not a valid URL.");
                    }
                })
                .unwrap();

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
            dbg!(&id);
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

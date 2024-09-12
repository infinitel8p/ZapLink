#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api::shell::open, AppHandle, CustomMenuItem, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

const LINKS: [(&str, &str, &str); 2] = [
    // github links
    ("open-github-infinitel8p", "InfiniteL8p","https://github.com/infinitel8p"),
    ("open-github-zaplink", "ZapLink","https://github.com/infinitel8p/zaplink"),
];

fn main() {
    let sub_menu_github = {
        let mut menu = SystemTrayMenu::new();
        for (id, label, _url) in
            LINKS.iter().filter(|(id, label, _url)| {
                id.starts_with("open-github")
            })
        {
            menu = menu.add_item(CustomMenuItem::new(
                id.to_string(),
                label.to_string(),
            ));
        }

        SystemTraySubmenu::new("GitHub", menu)
    };
    
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(
            "visibility-toggle".to_string(),
            "Show",
        ))
        .add_submenu(sub_menu_github)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new(
            "quit".to_string(),
            "Quit",
        ));

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(on_system_tray_event)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested {
                api, ..
            } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn on_system_tray_event(
    app: &AppHandle,
    event: SystemTrayEvent,
) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle =
                app.tray_handle().get_item(&id);
            dbg!(&id);
            match id.as_str() {
                "visibility-toggle" => {
                    let window =
                        app.get_window("main").unwrap();
                    match window.is_visible() {
                        Ok(true) => {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        },
                        Ok(false) => {
                            window.show();
                            item_handle.set_title("Hide").unwrap();
                        },
                        Err(e) => unimplemented!("what kind of errors happen here?"),
                    }
                }
                "quit" => app.exit(0),
                s if s.starts_with("open-") => {
                    if let Some(link) = LINKS
                        .iter()
                        .find(|(id, ..)| id == &s)
                    {
                        open(
                            &app.shell_scope(),
                            link.2,
                            None,
                        )
                        .unwrap();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}
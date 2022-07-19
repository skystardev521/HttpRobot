// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{self, Manager};

pub mod cmds;
pub mod events;
pub mod structs;
pub mod storage;

fn splashscreen<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_window("main").unwrap();

    let splash_window = app.get_window("splashscreen").unwrap();

    tauri::async_runtime::spawn(async move {
        println!("Show splashscreen...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Close splashscreen...");

        main_window.show().unwrap();

        splash_window.close().unwrap();
    });
    Ok(())
}

fn main() {
    let builder = tauri::Builder::default();
    cmds::invoke_handler(builder)
        .manage(structs::MyState {
            value: 0,
            label: "Tauri!".into(),
        })
        .setup(|app| {
           
            splashscreen(app)?;
            events::listen_event(app)?;

            let main_window = app.get_window("main");
            
            #[cfg(debug_assertions)]
            if let Some(window) = main_window{
                window.open_devtools();
            }
            Ok(())
        })
        .menu(events::make_menu())
        .on_menu_event(|event| {
            events::menu_event(event);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



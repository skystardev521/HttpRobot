// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{self, CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

pub mod commands;
pub mod events;
pub mod structs;

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
    tauri::Builder::default()
        .manage(structs::MyState {
            value: 0,
            label: "Tauri!".into(),
        })
        .on_menu_event(|event| {
            events::menu_event(event);
        })
        .menu(events::make_menu())
        .setup(|app| {
            splashscreen(app)?;
            events::listen_event(app)
        })
        .invoke_handler(tauri::generate_handler![
            commands::window_label,
            commands::simple_command,
            commands::stateful_command,
            commands::async_simple_command,
            commands::future_simple_command,
            commands::async_stateful_command,
            commands::command_arguments_wild,
            commands::command_arguments_struct,
            commands::stateful_command_with_result,
            commands::command_arguments_tuple_struct,
            commands::future_simple_command_with_return,
            commands::future_simple_command_with_result,
            commands::async_stateful_command_with_result,
            commands::open_remote_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

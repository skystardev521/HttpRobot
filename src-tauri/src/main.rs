// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use tauri;
pub mod commands;


#[derive(Debug)]
pub struct MyState {
    #[allow(dead_code)]
    value: u64,
    #[allow(dead_code)]
    label: String,
}

#[derive(Debug, serde::Serialize)]
pub enum MyError {
    FooError,
}

fn main() {
    tauri::Builder::default()
        .manage(MyState {
            value: 0,
            label: "Tauri!".into(),
        })
        .invoke_handler(tauri::generate_handler![
          
            commands::borrow_cmd,
            commands::borrow_cmd_async,
            commands::window_label,
            commands::force_async,
            commands::force_async_with_result,
            commands::simple_command,
            commands::stateful_command,
            commands::cmd,
            commands::invoke,
            commands::message,
            commands::resolver,
            commands::async_simple_command,
            commands::future_simple_command,
            commands::async_stateful_command,
            commands::command_arguments_wild,
            commands::command_arguments_struct,
            commands::simple_command_with_result,
            commands::stateful_command_with_result,
            commands::command_arguments_tuple_struct,
            commands::async_simple_command_with_result,
            commands::future_simple_command_with_return,
            commands::future_simple_command_with_result,
            commands::async_stateful_command_with_result,

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

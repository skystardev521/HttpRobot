use tauri::{self, command, State, Window};
use super::structs::{MyState,Person,InlinePerson,Payload};

#[command]
pub fn simple_command(argument: String) {
    println!("{}", argument);
}

#[command]
pub fn stateful_command(argument: Option<String>, state: State<'_, MyState>) {
    println!("{:?} {:?}", argument, state.inner());
}

// ------------------------ Commands using Window ------------------------
#[command]
pub fn window_label(window: Window) {
    println!("window label: {}", window.label());

    window
        .emit(
            "event-name",
            Payload {
                message: "Tauri is awesome!".into(),
            },
        )
        .unwrap();
}

#[command]
pub async fn async_simple_command(argument: String) {
    println!("{}", argument);

    use std::collections::HashMap;
    let resp = reqwest::get("https://httpbin.org/ip")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await;
    println!("{:#?}", resp);
}

#[command]
pub async fn async_stateful_command(
    argument: Option<String>,
    state: State<'_, MyState>,
) -> Result<(), ()> {
    println!("{:?} {:?}", argument, state.inner());
    Ok(())
}

// Raw future commands
#[command(async)]
pub fn future_simple_command(argument: String) -> impl std::future::Future<Output = ()> {
    println!("{}", argument);
    std::future::ready(())
}

#[command(async)]
pub fn future_simple_command_with_return(
    argument: String,
) -> impl std::future::Future<Output = String> {
    println!("{}", argument);
    std::future::ready(argument)
}

#[command(async)]
pub fn future_simple_command_with_result(
    argument: String,
) -> impl std::future::Future<Output = Result<String, ()>> {
    println!("{}", argument);
    std::future::ready(Ok(argument))
}

// ------------------------ Commands returning Result ------------------------


#[command]
pub fn stateful_command_with_result(
    argument: Option<String>,
    state: State<'_, MyState>,
) -> Result<String, String> {
    println!("{:?} {:?}", argument, state.inner());
    dbg!(argument.ok_or("Error".to_string()))
}

#[command]
pub async fn async_stateful_command_with_result(
    argument: Option<String>,
    state: State<'_, MyState>,
) -> Result<String, String> {
    println!("argument：{:?}", argument);
    println!("state.inner：{:?}", state.inner());
    Ok(argument.unwrap_or_else(|| "".to_string()))
}

// Non-Ident command function arguments

#[command]
pub fn command_arguments_wild(win: Window) {
    println!("Window label:{}", win.label())
}


#[command]
pub fn command_arguments_struct(Person { name, age }: Person<'_>) {
    println!("received person struct with name: {} | age: {}", name, age)
}

#[command]
pub fn command_arguments_tuple_struct(InlinePerson(name, age): InlinePerson<'_>) {
    println!("received person tuple with name: {} | age: {}", name, age)
}


#[tauri::command]
pub async fn open_remote_window(handle: tauri::AppHandle) {

  let remote_window = tauri::WindowBuilder::new(
    &handle,
    "remote_window", /* the unique window label */
    tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
  ).build().unwrap();
  remote_window.show().unwrap();
  println!("open_remote_window");
}
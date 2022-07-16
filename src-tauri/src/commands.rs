// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT


use serde::Deserialize;
use tauri::{command, State, Window};

use super::{MyState, MyError};


#[command]
pub fn cmd(_argument: String) {}

#[command]
pub fn invoke(_argument: String) {}

#[command]
pub fn message(_argument: String) {}

#[command]
pub fn resolver(_argument: String) {}

#[command]
pub fn simple_command(argument: String) {
  println!("{}", argument);
}

#[command]
pub fn stateful_command(argument: Option<String>, state: State<'_, super::MyState>) {
  println!("{:?} {:?}", argument, state.inner());
}

// ------------------------ Commands using Window ------------------------
#[command]
pub fn window_label(window: Window) {
  println!("window label: {}", window.label());
}

// Async commands

#[command]
pub async fn async_simple_command(argument: String) {
  println!("{}", argument);
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

#[command(async)]
pub fn force_async(argument: String) -> String {
  argument
}

#[command(async)]
pub fn force_async_with_result(argument: &str) -> Result<&str, MyError> {
  (!argument.is_empty())
    .then(|| argument)
    .ok_or(MyError::FooError)
}

// ------------------------ Commands returning Result ------------------------

#[command]
pub fn simple_command_with_result(argument: String) -> Result<String, MyError> {
  println!("{}", argument);
  (!argument.is_empty())
    .then(|| argument)
    .ok_or(MyError::FooError)
}

#[command]
pub fn stateful_command_with_result(
  argument: Option<String>,
  state: State<'_, MyState>,
) -> Result<String, MyError> {
  println!("{:?} {:?}", argument, state.inner());
  dbg!(argument.ok_or(MyError::FooError))
}

// Async commands

#[command]
pub async fn async_simple_command_with_result(argument: String) -> Result<String, MyError> {
  println!("{}", argument);
  Ok(argument)
}

#[command]
pub async fn async_stateful_command_with_result(
  argument: Option<String>,
  state: State<'_, MyState>,
) -> Result<String, MyError> {
  println!("argument：{:?}", argument);
  println!("state.inner：{:?}", state.inner().value);
  Ok(argument.unwrap_or_else(|| "".to_string()))
}

// Non-Ident command function arguments

#[command]
pub fn command_arguments_wild(_: Window) {
  println!("we saw the wildcard!")
}

#[derive(Deserialize)]
pub struct Person<'a> {
  name: &'a str,
  age: u8,
}

#[command]
pub fn command_arguments_struct(Person { name, age }: Person<'_>) {
  println!("received person struct with name: {} | age: {}", name, age)
}

#[derive(Deserialize)]
pub struct InlinePerson<'a>(&'a str, u8);

#[command]
pub fn command_arguments_tuple_struct(InlinePerson(name, age): InlinePerson<'_>) {
  println!("received person tuple with name: {} | age: {}", name, age)
}

#[command]
pub fn borrow_cmd(argument: &str) -> &str {
  argument
}

#[command]
pub fn borrow_cmd_async(argument: &str) -> &str {
  argument
}

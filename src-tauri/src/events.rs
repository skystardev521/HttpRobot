use tauri::{self, CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use super::structs::Payload;

pub fn make_menu() -> Menu {
    let load_menu = CustomMenuItem::new("load_menu", "Load");

    let file_menu = Submenu::new("File", Menu::new().add_item(load_menu));

    Menu::new()
        .add_submenu(file_menu)
        .add_item(CustomMenuItem::new("close_menu", "Close"))
}

pub fn menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "load_menu" => {
            println!("load_menu");
        }
        "close_menu" => {
            println!("close_menu");
            event.window().close().unwrap();
        }
        _ => {}
    }
}

//-------有问题发送事件收不到-------
pub fn listen_event<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _global_id = app.listen_global("event-name", |event| {
        println!("got event-name with payload {:?}", event.payload());
    });

    app.emit_all(
        "event-name",
        Payload {
            message: "Tauri is awesome!".into(),
        },
    )
    .unwrap();
    //app.unlisten(global_id);

    let main_window = app.get_window("main").unwrap();

    let _event_id = main_window.listen("event-name", |event| {
        println!("got window event-name with payload {:?}", event.payload());
    });

    main_window
        .emit(
            "event-name",
            Payload {
                message: "Tauri is awesome!".into(),
            },
        )
        .unwrap();
    //main_window.unlisten(event_id);

    Ok(())
}

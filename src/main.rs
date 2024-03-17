use data::ToDoState;
use druid::{AppLauncher, WindowDesc};
use im::Vector;
use saver::read_stored;
use ui::ui_builder;

mod ui;
mod data;
mod saver;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Rusty To-Do's")
        .window_size((400., 400.));

    let stored = read_stored();
    let default_state = ToDoState{
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .launch(default_state )
        .expect("Who knows why your here?")
}

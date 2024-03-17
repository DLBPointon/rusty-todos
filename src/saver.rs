use std::{fs::{self, read_to_string}, path::Path};

use directories::BaseDirs;
use druid::Widget;
use serde::{Deserialize, Serialize};
use crate::data::{ToDoItem, ToDoState};

pub struct Saver;

impl Widget<ToDoState> for Saver {
    fn event(&mut self, ctx: &mut druid::widget::prelude::EventCtx, event: &druid::widget::prelude::Event, data: &mut ToDoState, env: &druid::widget::prelude::Env) {
        //todo!()
    }

    fn lifecycle(&mut self, ctx: &mut druid::widget::prelude::LifeCycleCtx, event: &druid::widget::prelude::LifeCycle, data: &ToDoState, env: &druid::widget::prelude::Env) {
        //todo!()
    }

    fn update(&mut self, ctx: &mut druid::widget::prelude::UpdateCtx, old_data: &ToDoState, data: &ToDoState, env: &druid::widget::prelude::Env) {
        if data.todos != old_data.todos {
            if let Some(base_dirs) = BaseDirs::new() {
                let config = format!("{}/{}", base_dirs.config_dir().to_str().unwrap(), "rusty.json");
                let config_path = Path::new(&config);
                let tasks = TaskData {tasks: data.todos.clone().into_iter().collect() };
                fs::write(config_path, serde_json::to_string(&tasks).unwrap()).expect("Path not exist");
            }
        }
    }

    fn layout(&mut self, ctx: &mut druid::widget::prelude::LayoutCtx, bc: &druid::widget::prelude::BoxConstraints, data: &ToDoState, env: &druid::widget::prelude::Env) -> druid::Size {
        return druid::Size::new(0.1, 0.1)
    }

    fn paint(&mut self, ctx: &mut druid::widget::prelude::PaintCtx, data: &ToDoState, env: &druid::widget::prelude::Env) {
        //todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskData {
    pub tasks: Vec<ToDoItem>
}

pub fn read_stored() -> TaskData {
    if let Some(base_dirs) = BaseDirs::new() {
        let config = format!("{}/{}", base_dirs.config_dir().to_str().unwrap(), "rusty.json");
        let config_path = Path::new(&config);
        let data = match fs::read_to_string(config_path) {
            Ok(a) => a,
            Err(_) => return TaskData {tasks: Vec::new()}
        };
        match serde_json::from_str(&data) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Oops. Save data is corupted! Data is {}", e);
                return TaskData {tasks: Vec::new()}
            },
        }
    } else {
        return TaskData{ tasks: Vec::new()}
    }
}
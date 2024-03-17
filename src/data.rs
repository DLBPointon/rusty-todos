use druid::{Data, Lens};
use im::Vector;
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Lens, Default)]
pub struct ToDoState {
    pub todos: Vector<ToDoItem>,
    pub new_text: String,
}

#[derive(Clone, Data, Lens, Default, PartialEq, Serialize, Deserialize)]
pub struct ToDoItem {
    pub checked: bool,
    pub text: String
}
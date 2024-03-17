use druid::{Env, EventCtx, Menu, MenuItem, Point, UnitPoint, Widget, WidgetExt};
use druid::widget::{Button, Checkbox, Flex, Label, List, Padding, TextBox, ZStack};

use crate::data::{ToDoItem, ToDoState};
use crate::saver::Saver;

pub fn ui_builder() -> impl Widget<ToDoState> {
    let header = Flex::row()
                                    .with_flex_child(TextBox::new()
                                        .lens(ToDoState::new_text)
                                        .expand_width(), 1.)
                                    .with_child(
                                        Button::new("->")
                                        .on_click(|
                                             _ctx, data: &mut ToDoState, _env| {
                                                if data.new_text.trim() != "" {
                                                    let text = data.new_text.clone();
                                                    data.new_text = "".to_string();
                                                    data.todos.push_front(ToDoItem { checked: false, text })
                                                }
                                             }))
                                    .with_child(Saver {})
                                    ;

    let todos = List::new(|| {

        // Check - todo - clear
        Flex::row()
            .with_child(Checkbox::new("")
                .lens(ToDoItem::checked))
            .with_child(Label::new( |data: &ToDoItem, _: &Env | data.text.clone() ))
            .with_flex_spacer(0.01)
            .with_child(Button::new("...").on_click(|ctx: &mut EventCtx, data: &mut ToDoItem, _env | {
                let data_clone = data.clone();
                let menu: Menu<ToDoState> = Menu::empty()
                    .entry(MenuItem::new("Remove").on_activate(move |_, main_data: &mut ToDoState, _| {
                        let location = main_data.todos.index_of(&data_clone).unwrap();
                        main_data.todos.remove(location);
                    }));

                ctx.show_context_menu(menu, Point::new(0., 0.))
            }))
    }).lens(ToDoState::todos)
        .scroll().vertical();

    let clear_complete = Button::new("Clear Completed").on_click( | _, data: &mut ToDoState, _| {
        data.todos.retain( |item| !item.checked)
    });

    ZStack::new(
        Flex::column()
            .with_child(header)
            .with_flex_child(todos, 1.))
        .with_aligned_child(
            Padding::new(5.,clear_complete), UnitPoint::BOTTOM_RIGHT
    )
}
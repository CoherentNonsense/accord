use druid::{
    widget::{Button, Flex, Label, TextBox},
    Env, EventCtx, Selector, Widget, WidgetExt,
};

use crate::{data::AppState, matrix::login};

pub fn ui_app() -> impl Widget<AppState> {
    let name = Label::raw().lens(AppState::name);
    let username_input = TextBox::new()
        .with_placeholder("Username")
        .lens(AppState::username_text);
    let password_input = TextBox::new()
        .with_placeholder("Password")
        .lens(AppState::password_text);
    let button =
        Button::new("Login").on_click(|ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
            // let login_result = login(&data.password_text);
            let event_sink = ctx.get_external_handle();
            login(
                event_sink,
                String::from("https://matrix.org"),
                data.username_text.clone(),
                data.password_text.clone(),
            );
        });

    Flex::column()
        .with_child(name)
        .with_child(username_input)
        .with_child(password_input)
        .with_child(button)
}

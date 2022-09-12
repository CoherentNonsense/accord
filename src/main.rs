use druid::widget::{Align, Flex, Label, Padding, TextBox};
use druid::{
    AppLauncher, Data, Env, Lens, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc,
};

fn build_ui() -> impl Widget<()> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left"), 1.0)
                    .with_flex_child(Label::new("bottom left"), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Label::new("top right"), 1.0),
                1.0,
            ),
    )
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}

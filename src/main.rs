mod data;
mod delegate;
mod gui;
mod matrix;

use data::AppState;
use delegate::Delegate;
use druid::{AppLauncher, WindowDesc};
use gui::ui_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState::default();

    let window = WindowDesc::new(ui_app)
        .title("Accord")
        .window_size((600.0, 400.0));

    let launcher = AppLauncher::with_window(window).delegate(Delegate {});
    launcher.launch(state).expect("Failed to launch app");

    Ok(())
}

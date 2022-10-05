use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};

use crate::data::AppState;

pub const LOGIN: Selector = Selector::new("user.login");
pub const LOGIN_SUCCESS: Selector = Selector::new("user.login.success");
pub const LOGIN_ERROR: Selector = Selector::new("user.login.error");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(LOGIN) {
            println!("Logging in...");
            data.name = String::from("Logging in...");
            Handled::Yes
        } else if cmd.is(LOGIN_SUCCESS) {
            data.name = String::from("Logged in");
            println!("Logged in");
            Handled::Yes
        } else if cmd.is(LOGIN_ERROR) {
            println!("Could not login");
            data.name = String::from("Could not log in");
            Handled::Yes
        } else {
            println!("Command Forwarded: {:?}", cmd);
            Handled::No
        }
    }
}

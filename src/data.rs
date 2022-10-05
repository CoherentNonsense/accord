use druid::{Data, Lens};

#[derive(Data, Clone, Lens)]
pub struct AppState {
    pub name: String,
    pub username_text: String,
    pub password_text: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            username_text: "".to_owned(),
            password_text: "".to_owned(),
        }
    }
}

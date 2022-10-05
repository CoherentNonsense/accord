use druid::{ExtEventSink, Target};
use matrix_sdk::{reqwest::Url, Client};

use crate::delegate::{LOGIN, LOGIN_ERROR, LOGIN_SUCCESS};

pub fn login(event_sink: ExtEventSink, homeserver: String, username: String, password: String) {
    tokio::spawn(async move {
        let homeserver = Url::parse(&homeserver).expect("Could not parse homeserver");
        let client = Client::new(homeserver).await.unwrap();

        event_sink
            .submit_command(LOGIN, (), Target::Auto)
            .expect("Failed to send command");

        let result = client
            .login_username(&username, &password)
            .initial_device_display_name("rust-sdk")
            .send()
            .await;

        match result {
            Ok(_) => {
                event_sink
                    .submit_command(LOGIN_SUCCESS, (), Target::Auto)
                    .expect("Failed to send command");
            }
            Err(_) => {
                event_sink
                    .submit_command(LOGIN_ERROR, (), Target::Auto)
                    .expect("Failed to send command");
            }
        }
    });
}

use std::borrow::Borrow;

use crate::command::{self, KccliCommand};

pub struct PsCommand {}

impl KccliCommand for PsCommand {
    async fn execute(host: String, _: String, format: command::Format) {
        let client = reqwest::Client::new();
        let response = client  
            .get(format!("{host}/connectors"))
            .send()
            .await
            .unwrap();
        let data = response.text().await.unwrap();
        if format == command::Format::Json {
            println!("{data}");
            return
        }
        let v: Vec<String> = serde_json::from_str(data.borrow()).expect("Cannot read JSON response.");
        for i in v {
            println!("{i}")
        }
    }
}

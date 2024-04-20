use std::borrow::Borrow;

use crate::{command::{self, KccliCommand}, connector::Connector, print::KccliPrint};

pub struct GetCommand {}

impl KccliCommand for GetCommand {
    async fn execute(host: String, connector: String, format: command::Format) {
        let client = reqwest::Client::new();
        let response = client  
            .get(format!("{host}/connectors/{connector}"))
            .send()
            .await
            .unwrap();
        let data = response.text().await.unwrap();
        if format == command::Format::Json {
            println!("{data}");
            return
        }
        let c: Connector = serde_json::from_str(data.borrow()).expect("Cannot read JSON response.");
        c.print()
    }
}

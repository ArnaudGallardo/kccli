use std::{borrow::Borrow, io::Read};
use std::io;

use crate::{command::{self, KccliCommand}, connector::Connector, print::KccliPrint};

pub struct CreateCommand {}

impl KccliCommand for CreateCommand {
    async fn execute(host: String, _: String, format: command::Format) {
        let mut buffer = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut buffer).expect("Cannot read input");
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{host}/connectors"))
            .body(buffer)
            .header("Content-Type", "application/json")
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

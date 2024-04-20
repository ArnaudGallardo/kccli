use console::style;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

use crate::print::KccliPrint;

#[derive(Serialize, Deserialize)]
struct Task {
    connector: String,
    task: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Connector {
    name: String,
    config: Map<String, Value>,
    tasks: Vec<Task>,
}

impl KccliPrint for Connector {
    fn print(&self) {
        println!("{}:", style(&self.name).bold());
        println!("\tconfig:");
        for (k, v) in &self.config {
            println!("\t\t{k}: {v}");
        }
        println!("\ttasks:");
        for t in &self.tasks {
            println!("\t\t{}: {}", t.task, t.connector);
        }
    }
}

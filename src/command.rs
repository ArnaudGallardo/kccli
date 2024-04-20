use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    Json,
    Properties,
}

pub trait KccliCommand {
    async fn execute(host: String, target: String, format: Format);
}

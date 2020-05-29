mod cli;
mod client;

use anyhow::Result;
use clap::Clap;

const BASE_URL: &str = "https://xkcd.com";
const LATEST_COMIC: usize = 0;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let client = client::XkcdClient::new(args);
    client.run()
}

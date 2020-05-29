mod cli;

use clap::Clap;

const BASE_URL: &str = "https://xkcd.com";
const LATEST_COMIC: usize = 0;

fn main() {
    let args = cli::Args::parse();
    // TODO
}

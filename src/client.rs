use anyhow::Result;
use serde_derive::Deserialize;

use crate::cli::Args;

#[allow(dead_code)]
pub struct XkcdClient {
    args: Args,
}

impl XkcdClient {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn run(&self) -> Result<()> {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ComicResponse {
    month: String,
    num: usize,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

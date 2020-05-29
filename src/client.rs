use anyhow::Result;

use crate::cli::Args;

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

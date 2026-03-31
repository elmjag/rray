use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    #[arg(short, long)]
    pub scene: Option<PathBuf>,
}

pub fn parse() -> Args {
    Args::parse()
}

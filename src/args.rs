use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    #[arg(short, long)]
    pub scene: Option<PathBuf>,

    #[arg(long, default_value_t = 1)]
    pub scale: u32,

    #[arg(short, long, default_value_t = false)]
    pub fps_stats: bool,
}

pub fn parse() -> Args {
    Args::parse()
}

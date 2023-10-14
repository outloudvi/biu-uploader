mod constants;
mod interactive;
mod types;
mod upload;
mod util;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{debug, info};

use crate::types::FfprobeResponse;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    uid: u64,
    #[arg(long)]
    api_key: String,
    #[arg(long)]
    file_path: PathBuf,
    #[arg(long, default_value = "ffprobe")]
    ffprobe_path: String,
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Args::parse();

    let metadata = util::extract_metadata(&args.ffprobe_path, &args.file_path)?;

    interactive::guard_bitrate(metadata.format.bit_rate)?;
    let meta = interactive::guard_metadata(&metadata.format.tags)?;
    let category = interactive::guard_category()?;
    let typ = interactive::guard_type()?;

    info!("Submitting...");
    let file = fs::read(args.file_path)?;
    let md5_value = md5::compute(file);
    debug!("{:?}", md5_value);
    let dup_check_result = upload::check_dupe(
        args.uid,
        args.api_key,
        meta,
        category,
        typ,
        format!("{:x}", md5_value),
    );

    Ok(())
}

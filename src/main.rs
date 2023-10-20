mod constants;
mod interactive;
mod types;
mod upload;
mod util;

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;
use dialoguer::Confirm;
use log::{debug, error, info};

use crate::types::{DupeCheckError, FfprobeResponse};

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
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let args = Args::parse();

    let metadata = util::extract_metadata(&args.ffprobe_path, &args.file_path)?;

    interactive::guard_bitrate(metadata.format.bit_rate)?;
    let meta = interactive::guard_metadata(&metadata.format.tags)?;
    let category = interactive::guard_category()?;
    let typ = interactive::guard_type()?;

    info!("Submitting...");
    let file = fs::read(args.file_path)?;
    let md5_value = md5::compute(&file);
    let md5_str = format!("{:x}", md5_value);
    debug!("{:?}", md5_value);
    let dup_check_result = upload::check_dupe(
        args.uid,
        &args.api_key,
        &meta,
        category,
        typ,
        &md5_str,
        false,
    )?;
    if !dup_check_result.success {
        let error_code = dup_check_result.error_code.unwrap();
        error!("错误：{}", error_code);
        if error_code == DupeCheckError::PotentialDupe {
            util::display_dupes(dup_check_result.result.unwrap());
            let confirmation = Confirm::new().with_prompt("是否确认上传？").interact()?;
            if !confirmation {
                info!("上传被取消。");
                return Ok(());
            }
        } else {
            return Err(anyhow!("{}", error_code));
        }
    } else {
        info!("Dupcheck finished. Uploading.");
    }
    let upload_token = match dup_check_result.token {
        Some(x) => x,
        None => upload::check_dupe(
            args.uid,
            &args.api_key,
            &meta,
            category,
            typ,
            &md5_str,
            true,
        )?
        .token
        .unwrap(),
    };
    upload::upload(&upload_token, &md5_str, file)?;
    info!("上传成功。");
    Ok(())
}

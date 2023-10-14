use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};

use crate::FfprobeResponse;

pub(crate) fn extract_metadata(ffprobe_path: &str, file_path: &PathBuf) -> Result<FfprobeResponse> {
    let ffprobe_json = String::from_utf8(
        Command::new(ffprobe_path)
            .arg(file_path)
            .arg("-show_format")
            .arg("-print_format")
            .arg("json")
            .output()?
            .stdout,
    )?;

    serde_json::from_str::<FfprobeResponse>(&ffprobe_json).map_err(|r| anyhow!(r))
}

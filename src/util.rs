use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Result};
use term_table::row::Row;

use crate::types::DupeItem;
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

pub(crate) fn display_dupes(items: Vec<DupeItem>) {
    let mut tbl = term_table::Table::new();

    tbl.add_row(Row::new([
        "曲名",
        "歌手",
        "专辑",
        "音质",
        "撞车评分",
        "聆听",
    ]));
    for (index, item) in items.iter().enumerate() {
        let mut row = Row::new([
            item.title.to_owned(),
            item.singer.to_owned(),
            item.album.to_owned(),
            item.level.to_string(),
            item.score.to_string(),
            format!("https://biu.moe/#/s{}", item.sid),
        ]);
        if index == 0 {
            row.has_separator = true;
        }
        tbl.add_row(row);
    }

    println!("{}", tbl.render());
}

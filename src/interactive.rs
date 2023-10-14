use anyhow::{anyhow, Result};
use dialoguer::{Input, Select};
use log::debug;

use crate::constants::MIN_BITRATE;
use crate::types::{FfmpegTags, MusicCategory, MusicType, SubmittedMusicMetadata};

pub(crate) fn guard_bitrate(bit_rate: u64) -> Result<()> {
    if bit_rate < MIN_BITRATE {
        return Err(anyhow!(
            "Bitrate {} is too low - {}kbps ({}) is required",
            bit_rate,
            MIN_BITRATE / 1000,
            MIN_BITRATE
        ));
    } else {
        debug!("Good bitrate: {}", bit_rate);
    }
    Ok(())
}

pub(crate) fn guard_metadata(tags: &FfmpegTags) -> Result<SubmittedMusicMetadata> {
    let meta_title = tags.title.clone().unwrap_or_default();
    let title: String = Input::new()
        .with_prompt("Title")
        .default(meta_title)
        .interact_text()?;

    let meta_artist = tags
        .artist
        .clone()
        .unwrap_or(tags.album_artist.clone().unwrap_or_default());
    let artist: String = Input::new()
        .with_prompt("Artist")
        .default(meta_artist)
        .interact_text()?;

    let meta_title = tags.album.clone().unwrap_or_default();
    let album: String = Input::new()
        .with_prompt("Album")
        .default(meta_title)
        .interact_text()?;

    Ok(SubmittedMusicMetadata {
        title,
        artist,
        album,
    })
}

pub(crate) fn guard_category() -> Result<MusicCategory> {
    let selections = [
        MusicCategory::Anime,
        MusicCategory::Game,
        MusicCategory::Idol,
        MusicCategory::Touhou,
        MusicCategory::Vocaloid,
        MusicCategory::Doujin,
    ];
    let value = Select::new()
        .with_prompt("Category")
        .items(&selections)
        .interact()?;

    Ok(selections[value])
}

pub(crate) fn guard_type() -> Result<MusicType> {
    let selections = [
        MusicType::Original,
        MusicType::Instrumental,
        MusicType::PureMusic,
        MusicType::Feat,
        MusicType::Others,
    ];
    let value = Select::new()
        .with_prompt("Type")
        .items(&selections)
        .interact()?;

    Ok(selections[value])
}

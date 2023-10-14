use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Copy, Clone, Serialize, Debug)]
pub(crate) enum MusicCategory {
    /// 动画
    Anime = 1,
    /// 游戏
    Game = 2,
    /// 偶像
    Idol = 3,
    /// 东方 Project
    Touhou = 4,
    /// VOCALOID
    Vocaloid = 5,
    /// 同人（MAD 用曲和其它）
    Doujin = 6,
}

#[derive(Copy, Clone, Serialize, Debug)]
pub(crate) enum MusicType {
    /// 原唱
    Original = 1,
    /// 伴奏
    Instrumental = 2,
    /// 纯音乐
    PureMusic = 3,
    /// 翻唱
    Feat = 4,
    /// 其它（Freetalk、广播剧等）
    Others = 5,
}

impl Display for MusicCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicCategory::Anime => f.write_str("动画"),
            MusicCategory::Game => f.write_str("游戏"),
            MusicCategory::Idol => f.write_str("偶像"),
            MusicCategory::Touhou => f.write_str("东方 Project"),
            MusicCategory::Vocaloid => f.write_str("VOCALOID"),
            MusicCategory::Doujin => f.write_str("同人（MAD 用曲和其它）"),
        }
    }
}

impl Display for MusicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicType::Original => f.write_str("原唱"),
            MusicType::Instrumental => f.write_str("伴奏"),
            MusicType::PureMusic => f.write_str("纯音乐"),
            MusicType::Feat => f.write_str("翻唱"),
            MusicType::Others => f.write_str("其它（Freetalk、广播剧等）"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct FfprobeResponse {
    pub(crate) format: FfprobeFormat,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub(crate) struct FfprobeFormat {
    pub(crate) format_name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) duration: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) size: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) bit_rate: u64,
    pub(crate) tags: FfmpegTags,
}

#[derive(Deserialize, Debug)]
pub(crate) struct FfmpegTags {
    #[serde(deserialize_with = "chk_nonempty_str_or_none")]
    pub(crate) album: Option<String>,
    #[serde(deserialize_with = "chk_nonempty_str_or_none")]
    pub(crate) album_artist: Option<String>,
    #[serde(deserialize_with = "chk_nonempty_str_or_none")]
    pub(crate) artist: Option<String>,
    #[serde(deserialize_with = "chk_nonempty_str_or_none")]
    pub(crate) title: Option<String>,
}

fn chk_nonempty_str_or_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    if buf.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(buf.trim().to_string()))
    }
}

pub(crate) struct SubmittedMusicMetadata {
    pub(crate) album: String,
    pub(crate) artist: String,
    pub(crate) title: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct DupeCheckRequestBody {
    pub(crate) uid: u64,
    pub(crate) filemd5: String,
    pub(crate) title: String,
    pub(crate) singer: String,
    pub(crate) album: String,
    pub(crate) remark: String,
    // 0 or 1
    pub(crate) force: u8,
    pub(crate) type1: MusicCategory,
    pub(crate) type2: MusicType,
    pub(crate) sign: String,
}

use std::fmt::Display;

use num_derive::ToPrimitive;
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Copy, Clone, Debug, ToPrimitive)]
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

#[derive(Copy, Clone, Debug, ToPrimitive)]
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

#[derive(Copy, Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum MusicQualityLevel {
    Lossless = 1,
    HighQualityAac = 2,
    HighQualityMp3 = 3,
    LowQualityMp3 = 4,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub(crate) enum DupeCheckError {
    BadSignature = 1,
    PotentialDupe = 2,
    QueueIsFull = 3,
    BadParameters = 4,
    Md5Dupe = 5,
    DatabaseError = 6,
    UploadDisabled = 7,
    CategoryError = 8,
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

impl ToString for MusicQualityLevel {
    fn to_string(&self) -> String {
        match self {
            MusicQualityLevel::Lossless => "无损".to_string(),
            MusicQualityLevel::HighQualityAac => "高音质 AAC".to_string(),
            MusicQualityLevel::HighQualityMp3 => "高音质 MP3".to_string(),
            MusicQualityLevel::LowQualityMp3 => "低音质 MP#".to_string(),
        }
    }
}

impl Display for DupeCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DupeCheckError::BadSignature => f.write_str("签名错误。请检查 UID 及 API token。"),
            DupeCheckError::PotentialDupe => f.write_str("疑似撞车。请确认以下是否有撞车项。"),
            DupeCheckError::QueueIsFull => {
                f.write_str("待审队列超过 100 首。请删除未通过文件后重试。")
            },
            DupeCheckError::BadParameters => f.write_str("参数不齐。歌曲名不能为空。"),
            DupeCheckError::Md5Dupe => f.write_str("存在 MD5 重复的文件。"),
            DupeCheckError::DatabaseError => f.write_str("数据库错误。"),
            DupeCheckError::UploadDisabled => f.write_str("站点暂时关闭上传。"),
            DupeCheckError::CategoryError => f.write_str("分类/类型错误。"),
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

#[derive(Debug)]
pub(crate) struct DupeCheckRequestBody {
    pub(crate) uid: String,
    pub(crate) filemd5: String,
    pub(crate) title: String,
    pub(crate) singer: String,
    pub(crate) album: String,
    pub(crate) remark: String,
    // 0 or non-0
    pub(crate) force: String,

    pub(crate) type1: String,
    pub(crate) type2: String,
    pub(crate) sign: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DupeItem {
    pub(crate) sid: String,
    pub(crate) title: String,
    pub(crate) singer: String,
    pub(crate) album: String,
    pub(crate) level: MusicQualityLevel,
    pub(crate) score: f32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DupeCheckResponseBody {
    pub(crate) success: bool,
    pub(crate) token: Option<String>,
    pub(crate) error_code: Option<DupeCheckError>,
    pub(crate) result: Option<Vec<DupeItem>>,
}

use anyhow::Result;
use log::debug;

use crate::constants::{BIU_DUPCHK_URL, USER_AGENT};
use crate::types::{DupeCheckRequestBody, MusicCategory, MusicType, SubmittedMusicMetadata};

pub(crate) fn check_dupe(
    uid: u64,
    api_key: String,
    meta: SubmittedMusicMetadata,
    music_category: MusicCategory,
    music_type: MusicType,
    md5: String,
) -> Result<()> {
    let mut sig_body = String::new();
    sig_body.push_str(&uid.to_string());
    sig_body.push_str(&md5);
    sig_body.push_str(&meta.title);
    sig_body.push_str(&meta.artist);
    sig_body.push_str(&meta.album);
    sig_body.push_str("");
    sig_body.push_str(&api_key);
    let md5_signature_value = md5::compute(sig_body);
    let body = DupeCheckRequestBody {
        uid,
        title: meta.title,
        singer: meta.artist,
        album: meta.album,
        filemd5: md5,
        remark: "".to_owned(), // TODO
        force: 0,
        type1: music_category,
        type2: music_type,
        sign: format!("{:x}", md5_signature_value),
    };
    debug!("{:?}", body);
    let response: String = ureq::post(BIU_DUPCHK_URL)
        .set("User-Agent", USER_AGENT)
        .send_json(body)?
        .into_string()?;
    debug!("{}", response);
    Ok(())
}

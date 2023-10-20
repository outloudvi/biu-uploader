use std::borrow::Cow;
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use log::{debug, error};
use num_traits::ToPrimitive;
use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;
use reqwest::StatusCode;

use crate::constants::{BIU_DUPCHK_URL, QINIU_UPLOAD_URL, USER_AGENT};
use crate::types::{DupeCheckResponseBody, MusicCategory, MusicType, SubmittedMusicMetadata};

pub(crate) fn check_dupe(
    uid: u64,
    api_key: &str,
    meta: &SubmittedMusicMetadata,
    music_category: MusicCategory,
    music_type: MusicType,
    file_md5: &str,
    forced: bool,
) -> Result<DupeCheckResponseBody> {
    let mut sig_body = String::new();
    sig_body.push_str(&uid.to_string());
    sig_body.push_str(file_md5);
    sig_body.push_str(&meta.title);
    sig_body.push_str(&meta.artist);
    sig_body.push_str(&meta.album);
    sig_body.push_str("");
    sig_body.push_str(api_key);
    let md5_signature_value = md5::compute(sig_body);

    let mut form = HashMap::<&str, String>::new();
    form.insert("uid", uid.to_string());
    form.insert("filemd5", file_md5.to_string());
    form.insert("title", meta.title.to_string());
    form.insert("singer", meta.artist.to_string());
    form.insert("album", meta.album.to_string());
    form.insert("remark", "".to_string());
    form.insert(
        "type1",
        ToPrimitive::to_u8(&music_category).unwrap().to_string(),
    );
    form.insert(
        "type2",
        ToPrimitive::to_u8(&music_type).unwrap().to_string(),
    );
    form.insert("sign", format!("{:x}", md5_signature_value));
    form.insert("force", (if forced { "1" } else { "0" }).to_string());
    debug!("Request body: {:?}", form);

    let client = Client::new();
    let response = client
        .post(BIU_DUPCHK_URL)
        .form(&form)
        .send()?
        .json::<DupeCheckResponseBody>()?;
    debug!("Response body: {:?}", response);
    Ok(response)
}

pub(crate) fn upload(token: &str, md5: &str, file: Vec<u8>) -> Result<()> {
    let form = Form::new()
        .text("key", md5.to_owned())
        .text("x:md5", md5.to_owned())
        .text("token", token.to_owned())
        .part("file", Part::bytes(Cow::Owned(file)));

    let client = Client::new();
    let response = client.post(QINIU_UPLOAD_URL).multipart(form).send()?;
    if response.status() == StatusCode::OK {
        Ok(())
    } else {
        Err(anyhow!(response.text()?))
    }
}

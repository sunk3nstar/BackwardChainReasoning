//! 从JSON文件/字符串中获取知识库

use super::KB;
use std::fs;

pub fn make_kb_from_str<T: AsRef<str>>(src: T) -> Result<KB, serde_json::Error> {
    let kb: KB = serde_json::from_str(src.as_ref())?;
    Ok(kb)
}

pub fn make_kb_from_file<T: AsRef<std::path::Path>>(
    src: T,
) -> Result<KB, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(src.as_ref())?;
    Ok(make_kb_from_str(&data)?)
}

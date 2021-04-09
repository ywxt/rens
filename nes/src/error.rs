use thiserror::Error;
#[derive(Error, Debug)]
pub enum RomError{
    #[error("无效的INES文件: {0}")]
    InvalidInes(String),
}

pub type Result<T> = anyhow::Result<T>;
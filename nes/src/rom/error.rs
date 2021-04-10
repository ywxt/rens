use thiserror::Error;
#[derive(Error, Debug)]
pub enum NesError {
    #[error("无效的INES文件: {0}")]
    InvalidInes(String),
}


use thiserror::Error;
#[derive(Error, Debug)]
pub enum CpuError {
    #[error("无法读取地址: {0}")]
    ReadMemoryAddressError(u16),

    #[error("无法写入地址: {0}")]
    WriteMemoryAddressError(u16),
}

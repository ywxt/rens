use thiserror::Error;
#[derive(Error, Debug)]
pub enum CpuError {
    #[error("无法读取地址: {0:#010X}")]
    ReadMemoryAddressError(u16),

    #[error("无法写入地址: {0:#010X}")]
    WriteMemoryAddressError(u16),

    #[error("无效的指令: {0:#04X}")]
    UnknownInstructionError(u8),

    #[error("执行`{0:#04X}`指令发生错误")]
    InvokeInstructionError(u8),
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum CpuError {
    #[error("无法读取地址: 0x{0:X}")]
    ReadMemoryAddressError(u16),

    #[error("无法写入地址: 0x{0:X}")]
    WriteMemoryAddressError(u16),

    #[error("无效的指令: 0x{0:02X}")]
    UnknownInstructionError(u8),

    #[error("执行`0x{0:02X}`指令发生错误")]
    InvokeInstructionError(u8),
}

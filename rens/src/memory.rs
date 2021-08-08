#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("无法读取地址: {0:#010X}")]
    ReadMemory(u16),

    #[error("无法写入地址: {0:#010X}")]
    WriteMemory(u16),
}

pub type Result<T> = std::result::Result<T, MemoryError>;
pub trait Memory {
    fn read(&self, address: u16) -> Result<u8>;
    /// 默认小端
    fn read_word(&self, address: u16) -> Result<u16> {
        let low = self.read(address)?;
        let high = self.read(address + 1)?;
        Ok(((high as u16) << 8) | (low as u16))
    }
    fn write(&mut self, address: u16, data: u8) -> Result<()>;
    /// 默认小端
    fn write_word(&mut self, address: u16, data: u16) -> Result<()> {
        let low = (data & 0x00FF) as u8;
        let high = (data >> 8) as u8;
        self.write(address, low)
            .and_then(|_| self.write(address + 1, high))
    }
}

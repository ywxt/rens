pub trait Memory {
    fn read(&self, address: u16) -> Option<u8>;
    /// 默认小端
    fn read_word(&self, address: u16) -> Option<u16> {
        let low = self.read(address)?;
        let high = self.read(address + 1)?;
        Some(((high as u16) << 8) | (low as u16))
    }
    fn write(&mut self, address: u16, data: u8) -> bool;
    fn write_word(&mut self, address: u16, data: u16) -> bool {
        let low = (data & 0x00FF) as u8;
        let high = (data >> 8) as u8;
        self.write(address, low) && self.write(address + 1, high)
    }
}

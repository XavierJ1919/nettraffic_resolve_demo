pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    pub fn read_one_byte(&mut self) -> Result<u8, Err> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }
    pub fn read_u16(&mut self) -> Result<u16, Err> {
        let res = ((self.read_one_byte()? as u16) << 8) | (self.read_one_byte()? as u16);
        Ok(res)
    }
}
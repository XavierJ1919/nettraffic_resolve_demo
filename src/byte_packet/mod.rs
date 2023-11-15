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
    pub fn get(&mut self, pos: usize) -> Result<u8, Err> {
        Ok(self.buf[pos])
    }
    pub fn set_pos(&mut self, pos: usize) -> Result<(), Err> {
        self.pos = pos;
        Ok(())
    }
    pub fn read_u16(&mut self) -> Result<u16, Err> {
        let res = ((self.read_one_byte()? as u16) << 8) | (self.read_one_byte()? as u16);
        Ok(res)
    }
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], Err> {
        Ok(&self.buf[start..(start + len) as usize])
    }

    pub fn read_qname(&mut self, qname: &mut String) -> Result<(), Err> {
        let mut pos = self.pos;
        let len = self.get(pos)?;
        let mut delim = "";
        while len != 0 {
            if (len & 0xC0) == 0xC0 {
                self.set_pos(pos+2)?;
                let len2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | len2;
                pos = offset as usize;
                continue;
            }
            pos += 1;
            qname.push_str(delim);

            let qname_piece = self.get_range(pos, len as usize)?;
            qname.push_str(&String::from_utf8_lossy(qname_piece).to_lowercase());
            delim = ".";
            pos += len as usize;
        }
        Ok(())
    }
}
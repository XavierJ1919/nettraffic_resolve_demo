use crate::*;

pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0;512],
            pos: 0,
        }
    }
    pub fn read_one_byte(&mut self) -> Result<u8, Error> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }
    pub fn get(&mut self, pos: usize) -> Result<u8, Error> {
        Ok(self.buf[pos])
    }
    pub fn set_pos(&mut self, pos: usize) -> Result<(), Error> {
        self.pos = pos;
        Ok(())
    }
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let res = ((self.read_one_byte()? as u16) << 8) | (self.read_one_byte()? as u16);
        Ok(res)
    }
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let res = ((self.read_u16()? as u32) << 16) | (self.read_u16()? as u32);
        Ok(res)
    }
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], Error> {
        if start + len >= 512 {
            return Err("Error: end of buffer".into());
        }
        Ok(&self.buf[start..start + len])
    }

    pub fn read_qname(&mut self, qname: &mut String) -> Result<(), Error> {
        let mut pos = self.pos;
        let mut delim = "";
        let mut jumped = false;
        let mut len = 1;
        while len != 0 {
            let len = self.get(pos)?;
            if (len & 0xC0) == 0xC0 {
                self.set_pos(pos+2)?;
                let len2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | len2;
                pos = offset as usize;
                jumped = true;
                continue;
            } else {
                qname.push_str(delim);

                let qname_label = self.get_range(pos, len as usize)?;
                qname.push_str(&String::from_utf8_lossy(qname_label).to_lowercase());
                delim = ".";
                pos += len as usize;
                pos += 1;
            }
        }
        if !jumped {
            self.set_pos(pos)?;
        }
        Ok(())
    }
}
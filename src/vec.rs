use std::cmp::min;
use std::io::{Result, Error, ErrorKind};

use super::{ReadAt, WriteAt, Size};

impl ReadAt for Vec<u8> {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        self.as_slice().read_at(pos, buf)
    }
}

impl WriteAt for Vec<u8> {
    fn write_at(&mut self, pos: u64, buf: &[u8]) -> Result<usize> {
        // Ensure no overflow.
        if pos > usize::max_value() as u64 {
            return Err(Error::new(ErrorKind::InvalidInput, "vector size too big"));
        }
        let pos = pos as usize;

        // Resize the vector so pos <= self.len().
        if pos >= self.len() {
            self.resize(pos as usize, 0);
        }

        // Copy anything that fits into existing space.
        let avail = min(self.len() - pos, buf.len());
        if avail > 0 {
            self[pos..(pos + avail)].copy_from_slice(&buf[..avail]);
        }

        // Extend with anything leftover.
        if avail < buf.len() {
            self.extend_from_slice(&buf[avail..]);
        }

        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Size for Vec<u8> {
    fn size(&self) -> Result<Option<u64>> {
        Ok(Some(self.len() as u64))
    }
}

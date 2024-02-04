use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Error)]
#[error("")]
pub struct InvalidAddress;

pub trait MemoryCard {
    fn size(&self) -> u16;

    fn read_byte(&self, addr: u16) -> Result<u8, InvalidAddress>;

    fn read_word(&self, addr: u16) -> Result<u16, InvalidAddress>;

    fn write_byte(&mut self, addr: u16, byte: u8)
        -> Result<(), InvalidAddress>;

    fn write_word(
        &mut self,
        addr: u16,
        word: u16,
    ) -> Result<(), InvalidAddress>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualMemoryCard {
    data: Box<[u8]>,
}

impl VirtualMemoryCard {
    pub fn new(size: u16) -> Self {
        Self { data: vec![0; usize::from(size)].into() }
    }
}

impl MemoryCard for VirtualMemoryCard {
    fn size(&self) -> u16 {
        self.data.len() as u16
    }

    fn read_byte(&self, addr: u16) -> Result<u8, InvalidAddress> {
        self.data.get(usize::from(addr)).copied().ok_or(InvalidAddress)
    }

    fn read_word(&self, addr: u16) -> Result<u16, InvalidAddress> {
        let low = self.read_byte(addr)?;
        let high =
            self.read_byte(addr.checked_add(1).ok_or(InvalidAddress)?)?;
        Ok(u16::from_le_bytes([low, high]))
    }

    fn write_byte(
        &mut self,
        addr: u16,
        byte: u8,
    ) -> Result<(), InvalidAddress> {
        let dest =
            self.data.get_mut(usize::from(addr)).ok_or(InvalidAddress)?;
        *dest = byte;
        Ok(())
    }

    fn write_word(
        &mut self,
        addr: u16,
        word: u16,
    ) -> Result<(), InvalidAddress> {
        let low_addr = usize::from(addr);
        let high_addr = usize::from(addr.checked_add(1).ok_or(InvalidAddress)?);
        let [low, high] = word.to_le_bytes();
        let Some([low_dest, high_dest]) =
            self.data.get_mut(low_addr ..= high_addr)
        else {
            Err(InvalidAddress)?
        };
        *low_dest = low;
        *high_dest = high;
        Ok(())
    }
}

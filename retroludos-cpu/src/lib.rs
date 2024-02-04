use std::{collections::BTreeMap, fmt};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Error)]
#[error("invalid memory address")]
pub struct InvalidAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Error)]
#[error("invalid memory device")]
pub struct InvalidMemoryDevice;

pub type DynMemoryDevice<'a> = Box<dyn MemoryDevice + Send + Sync + 'a>;

pub trait MemoryDevice {
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

impl MemoryDevice for VirtualMemoryCard {
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

#[derive(Default)]
pub struct MemoryMultiplexer<'a> {
    devices: BTreeMap<u16, DynMemoryDevice<'a>>,
}

impl<'a> fmt::Debug for MemoryMultiplexer<'a> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debugger = fmtr.debug_list();
        for (start, device) in &self.devices {
            debugger.entry(&(*start .. *start + device.size()));
        }
        debugger.finish()
    }
}

impl<'a> MemoryMultiplexer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    fn infimum_device_range(&self, addr: u16) -> Option<(u16, u16)> {
        let (start, device) = self.devices.range(..= addr).last()?;
        Some((*start, *start + device.size()))
    }

    fn supremum_device_range(&self, addr: u16) -> Option<(u16, u16)> {
        let (start, device) = self.devices.range(addr ..).next()?;
        Some((*start, *start + device.size()))
    }

    fn select_device(
        &self,
        addr: u16,
    ) -> Result<&DynMemoryDevice<'a>, InvalidAddress> {
        let (_, device) =
            self.devices.range(..= addr).last().ok_or(InvalidAddress)?;
        Ok(device)
    }

    fn select_device_mut(
        &mut self,
        addr: u16,
    ) -> Result<&mut DynMemoryDevice<'a>, InvalidAddress> {
        let (_, device) =
            self.devices.range_mut(..= addr).last().ok_or(InvalidAddress)?;
        Ok(device)
    }

    pub fn add_device<M>(
        &mut self,
        addr: u16,
        device: M,
    ) -> Result<(), InvalidMemoryDevice>
    where
        M: MemoryDevice + Send + Sync + 'a,
    {
        self.add_dyn_device(addr, Box::new(device))
    }

    pub fn add_dyn_device(
        &mut self,
        addr: u16,
        device: DynMemoryDevice<'a>,
    ) -> Result<(), InvalidMemoryDevice> {
        let end_addr =
            addr.checked_add(device.size()).ok_or(InvalidMemoryDevice)?;
        if let Some((_, end)) = self.infimum_device_range(addr) {
            if end > addr {
                Err(InvalidMemoryDevice)?;
            }
        }
        if let Some((start, _)) = self.supremum_device_range(addr) {
            if end_addr > start {
                Err(InvalidMemoryDevice)?;
            }
        }
        self.devices.insert(addr, device);
        Ok(())
    }

    pub fn read_byte(&self, addr: u16) -> Result<u8, InvalidAddress> {
        self.select_device(addr)?.read_byte(addr)
    }

    pub fn read_word(&self, addr: u16) -> Result<u16, InvalidAddress> {
        self.select_device(addr)?.read_word(addr)
    }

    pub fn write_byte(
        &mut self,
        addr: u16,
        byte: u8,
    ) -> Result<(), InvalidAddress> {
        self.select_device_mut(addr)?.write_byte(addr, byte)
    }

    pub fn write_word(
        &mut self,
        addr: u16,
        word: u16,
    ) -> Result<(), InvalidAddress> {
        self.select_device_mut(addr)?.write_word(addr, word)
    }
}

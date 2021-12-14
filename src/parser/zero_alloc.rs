use crate::{
    as_c_struct::{ELFHeader as ELFHeaderAsC, ELF_CLASS_OFFSET},
    ELFHeader,
};
use async_trait::async_trait;
use std::mem::size_of;

enum RawELFHeader<'a> {
    U32(&'a ELFHeaderAsC<u32>),
    U64(&'a ELFHeaderAsC<u64>),
}

impl RawELFHeader<'_> {
    fn magic(&self) -> &[u8; 4] {
        match self {
            RawELFHeader::U32(h) => &h.e_ident.magic,
            RawELFHeader::U64(h) => &h.e_ident.magic,
        }
    }

    fn endian(&self) -> &u8 {
        match self {
            RawELFHeader::U32(h) => &h.e_ident.data,
            RawELFHeader::U64(h) => &h.e_ident.data,
        }
    }

    fn swap_bytes(&self) -> Self {
        match self {
            RawELFHeader::U32(h) => RawELFHeader::U32(&h.swap_bytes()),
            RawELFHeader::U64(h) => RawELFHeader::U64(&h.swap_bytes()),
        }
    }
}

pub struct ZeroAllocationELFParser<'a> {
    raw_elf_header: RawELFHeader<'a>,
    raw_data: &'a [u8],
}

impl<'a> ZeroAllocationELFParser<'a> {
    pub fn new(raw_data: &'a [u8]) -> Result<Self, crate::Error> {
        let elf_class = raw_data[ELF_CLASS_OFFSET];
        if raw_data.len() < 5 {
            return Err(crate::Error::InvalidFile);
        }
        let raw_elf_header = match elf_class {
            1 => {
                if size_of::<ELFHeaderAsC<u32>>() > raw_data.len() {
                    return Err(crate::Error::InvalidFile);
                }
                let elf_header_as_c: &ELFHeaderAsC<u32> = zero::read(raw_data);
                RawELFHeader::U32(elf_header_as_c)
            }
            2 => {
                if size_of::<ELFHeaderAsC<u32>>() > raw_data.len() {
                    return Err(crate::Error::InvalidFile);
                }
                let elf_header_as_c: &ELFHeaderAsC<u64> = zero::read(raw_data);
                RawELFHeader::U64(elf_header_as_c)
            }
        };
        Ok(Self {
            raw_elf_header,
            raw_data,
        })
    }
}

#[async_trait]
impl<Class> super::ELFParser<Class> for ZeroAllocationELFParser<'_> {
    fn has_valid_magic(&self) -> bool {
        self.raw_elf_header.magic() == &[0x7f, b'E', b'L', b'F']
    }

    async fn parse_elf_header(&self) -> Result<ELFHeader<Class>, crate::Error> {
        if self.has_valid_magic() {
            return Err(crate::Error::InvalidMagic(self.raw_elf_header.magic()));
        }
        let raw_elf_header = if cfg!(target_endian = "little") && self.raw_elf_header.endian() == &2
        {
            self.raw_elf_header.swap_bytes()
        } else if cfg!(target_endian = "big") && self.raw_elf_header.endian() == &1 {
            self.raw_elf_header.swap_bytes()
        } else {
            self.raw_elf_header
        };
        match raw_elf_header {
            RawELFHeader::U32(h) => {
                let elf_header = ELFHeader::from_c_struct(h);
                Ok(elf_header)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ELFParser;

    #[test]
    fn has_valid_magic() {
        let raw_data = include_bytes!("../main");
        let parser = ZeroAllocationELFParser::new(raw_data);
        assert!(parser.has_valid_magic());
    }
}

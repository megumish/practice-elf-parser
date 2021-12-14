use async_trait::async_trait;

use crate::ELFHeader;

pub mod zero_alloc;

#[async_trait]
pub trait ELFParser<Class> {
    fn has_valid_magic(&self) -> bool;
    async fn parse_elf_header(&self) -> Result<ELFHeader<Class>, crate::Error>;
}

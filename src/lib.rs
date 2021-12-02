use anyhow::Context;
use std::{io::BufRead, mem::transmute};

struct ELF64 {
    endian: Endian,
    version: Version,
    os_abi: OSABI,
    file_type: FileType,
    machine: MachineType,
    entry: Address,
    program_header_offset: u64,
    section_header_offset: u64,
    flags: u64,
    elf_header_size: u32,
    program_header_entry_size: u32,
    program_header_number: u32,
    section_header_entry_size: u32,
    section_header_number: u32,
    section_header_string_table_index: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
enum Endian {
    Little,
    Big,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
enum Version {
    None,
    Current,
    Unknown(u8),
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
enum OSABI {
    None,
    SystemV,
    HPUX,
    NetBSD,
    GNUOrLinux,
    Solaris,
    AIX,
    SGIIrix,
    FreeBSD,
    TRU64,
    NovellModesto,
    OpenBSD,
    ArmEABI,
    Arm,
    Standalone,
    Unknown(u8),
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
enum MachineType {
    AMD64,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
enum FileType {
    Executable,
    Relocatble,
    Core,
    DynamicLibrary,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
enum ELFClass {
    None,
    ELF32,
    ELF64,
    Unknown(u8),
}

struct Address(u64);

impl ELF64 {
    fn is_valid_magic(input: &[u8]) -> anyhow::Result<bool> {
        input
            .get(0..4)
            .map(|magic| magic == b"\x7fELF")
            .context("Failed to read magic number")
    }

    fn parse_elf_class(input: &[u8]) -> anyhow::Result<ELFClass> {
        let elf_class = input.get(4).context("Failed to read elf class")?;
        match elf_class {
            b'\x00' => Ok(ELFClass::None),
            b'\x01' => Ok(ELFClass::ELF32),
            b'\x02' => Ok(ELFClass::ELF64),
            b => Ok(ELFClass::Unknown(*b)),
        }
    }

    fn parse_endian(input: &[u8]) -> anyhow::Result<Endian> {
        let endian = input.get(5).context("Failed to read endian")?;
        match endian {
            b'\x01' => Ok(Endian::Little),
            b'\x02' => Ok(Endian::Big),
            _ => Err(anyhow::anyhow!("Invalid endian")),
        }
    }

    fn parse_version(input: &[u8]) -> anyhow::Result<Version> {
        let version = input.get(6).context("Failed to read elf version")?;
        match version {
            b'\x00' => Ok(Version::None),
            b'\x01' => Ok(Version::Current),
            b => Ok(Version::Unknown(*b)),
        }
    }

    fn parse_os_abi(input: &[u8]) -> anyhow::Result<OSABI> {
        let os_abi = input.get(7).context("Failed to read os abi")?;
        match os_abi {
            b'\x00' => Ok(OSABI::SystemV),
            b'\x01' => Ok(OSABI::HPUX),
            b'\x02' => Ok(OSABI::NetBSD),
            b'\x03' => Ok(OSABI::GNUOrLinux),
            b'\x06' => Ok(OSABI::Solaris),
            b'\x07' => Ok(OSABI::AIX),
            b'\x08' => Ok(OSABI::SGIIrix),
            b'\x09' => Ok(OSABI::FreeBSD),
            b'\x0A' => Ok(OSABI::TRU64),
            b'\x0B' => Ok(OSABI::NovellModesto),
            b'\x0C' => Ok(OSABI::OpenBSD),
            b'\x40' => Ok(OSABI::ArmEABI),
            b'\x61' => Ok(OSABI::Arm),
            b'\xff' => Ok(OSABI::Standalone),
            b => Ok(OSABI::Unknown(*b)),
        }
    }

    fn parse_abi_version(input: &[u8]) -> anyhow::Result<u8> {
        input
            .get(8)
            .map(|b| *b)
            .context("Failed to read abi version")
    }

    fn parse_file_type(input: &[u8], endian: Endian) -> anyhow::Result<FileType> {
        let file_type_bytes = input.get(10..12).context("Failed to read file type")?;
        let file_type = unsafe {
            let file_type_fixed_array = &*(file_type_bytes.as_ptr() as *const [u8; 2]);
            match endian {
                Endian::Little => transmute::<[u8; 2], u16>(*file_type_fixed_array).to_le(),
                Endian::Big => transmute::<[u8; 2], u16>(*file_type_fixed_array).to_be(),
            }
        };
        match file_type {
            0x00 => Ok(FileType::Executable),
            0x01 => Ok(FileType::Relocatble),
            0x02 => Ok(FileType::Core),
            0x03 => Ok(FileType::DynamicLibrary),
            _ => Err(anyhow::anyhow!("Invalid file type")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn is_valid_magic() -> anyhow::Result<()> {
        let buffer = include_bytes!("main");
        assert!(ELF64::is_valid_magic(buffer)?);
        Ok(())
    }

    #[test]
    fn is_elf64() -> anyhow::Result<()> {
        let buffer = include_bytes!("main");
        assert_eq!(ELF64::parse_elf_class(buffer)?, ELFClass::ELF64);
        Ok(())
    }

    #[test]
    fn is_little_endian() -> anyhow::Result<()> {
        let buffer = include_bytes!("main");
        assert_eq!(ELF64::parse_endian(buffer)?, Endian::Little);
        Ok(())
    }

    #[test]
    fn is_current_elf_version() -> anyhow::Result<()> {
        let buffer = include_bytes!("main");
        assert_eq!(ELF64::parse_version(buffer)?, Version::Current);
        Ok(())
    }

    #[test]
    fn is_system_v() -> anyhow::Result<()> {
        let buffer = include_bytes!("main");
        assert_eq!(ELF64::parse_os_abi(buffer)?, OSABI::SystemV);
        Ok(())
    }

    #[test]
    fn is_zero_abi_version() -> anyhow::Result<()> {
        let buffer = include_bytes!("main");
        assert_eq!(ELF64::parse_abi_version(buffer)?, 0);
        Ok(())
    }

    #[test]
    fn is_executable() -> anyhow::Result<()> {
        let buffer = include_bytes!("main");
        assert_eq!(
            ELF64::parse_file_type(buffer, Endian::Little)?,
            FileType::Executable
        );
        Ok(())
    }
}

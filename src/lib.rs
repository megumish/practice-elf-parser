use std::mem::size_of;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ELFHeader<Class>
where
    Class: SwapBytes + Copy,
{
    pub e_ident: ELFIdentifier,
    pub e_type: FileType,
    pub e_machine: MachineType,
    pub e_version: u32,
    pub e_entry: Class,
    pub e_phoff: Class,
    pub e_shoff: Class,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Endian {
    None,
    Little,
    Big,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Version {
    None,
    Current,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum OSABI {
    SystemV,
    HPUX,
    NetBSD,
    GNUOrLinux,
    Solaris = 6,
    AIX = 7,
    SGIIrix = 8,
    FreeBSD = 9,
    TRU64 = 10,
    NovellModesto = 11,
    OpenBSD = 12,
    ArmEABI = 64,
    Arm = 97,
    Standalone = 255,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
#[repr(u16)]
pub enum MachineType {
    AMD64 = 62,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
#[repr(u16)]
pub enum FileType {
    None,
    Executable,
    Relocatble,
    Core,
    DynamicLibrary,
    Number,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
#[repr(u8)]
pub enum ELFClass {
    None,
    ELF32,
    ELF64,
}

impl<Class> ELFHeader<Class>
where
    Class: SwapBytes + Copy,
{
    fn swap_bytes(&self) -> Self {
        Self {
            e_ident: self.e_ident,
            e_type: self.e_type.swap_bytes(),
            e_machine: self.e_machine.swap_bytes(),
            e_version: self.e_version.swap_bytes(),
            e_entry: self.e_entry.swap_bytes(),
            e_phoff: self.e_phoff.swap_bytes(),
            e_shoff: self.e_shoff.swap_bytes(),
            e_flags: self.e_flags.swap_bytes(),
            e_ehsize: self.e_ehsize.swap_bytes(),
            e_phentsize: self.e_phentsize.swap_bytes(),
            e_phnum: self.e_phnum.swap_bytes(),
            e_shentsize: self.e_shentsize.swap_bytes(),
            e_shnum: self.e_shnum.swap_bytes(),
            e_shstrndx: self.e_shstrndx.swap_bytes(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ELFIdentifier {
    pub magic: [u8; 4],
    pub class: ELFClass,
    pub data: Endian,
    pub version: Version,
    pub os_abi: OSABI,
    pub os_abi_version: u8,
    pub padding: [u8; 7],
}

pub trait SwapBytes {
    fn swap_bytes(self) -> Self;
}

impl SwapBytes for u32 {
    fn swap_bytes(self) -> u32 {
        self.swap_bytes()
    }
}

impl SwapBytes for u64 {
    fn swap_bytes(self) -> u64 {
        self.swap_bytes()
    }
}

impl SwapBytes for FileType {
    fn swap_bytes(self) -> FileType {
        unsafe {
            *(&(std::mem::transmute::<FileType, u16>(self).swap_bytes()) as *const u16
                as *const FileType)
        }
    }
}

impl SwapBytes for MachineType {
    fn swap_bytes(self) -> MachineType {
        unsafe {
            *(&(std::mem::transmute::<MachineType, u16>(self).swap_bytes()) as *const u16
                as *const MachineType)
        }
    }
}

pub fn parse_elf_header<'a>(bytes: &'a [u8]) -> &'a ELFHeader<u64> {
    if bytes.len() < size_of::<ELFHeader<u64>>() {
        panic!("ELF header is too small");
    }
    let header = unsafe {
        &*(&bytes[0..size_of::<ELFHeader<u64>>()] as *const [u8] as *const ELFHeader<u64>)
    };
    if cfg!(target_endian = "big") && header.e_ident.data == Endian::Little {
        Box::leak(Box::new(header.swap_bytes()))
    } else if cfg!(target_endian = "little") && header.e_ident.data == Endian::Big {
        Box::leak(Box::new(header.swap_bytes()))
    } else {
        header
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_elf_header;

    #[test]
    fn it_works() {
        let bytes = include_bytes!("main");
        let header = parse_elf_header(bytes);
        println!("{header:#x?}")
    }
}

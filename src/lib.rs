mod as_c_struct;
mod parser;

pub struct ELFHeader<'a, Class> {
    pub endian: Endian,
    pub version: Version<'a>,
    pub os_abi: OSABI<'a>,
    pub file_type: FileType<'a>,
    pub machine: MachineType<'a>,
    pub entry: Address<'a>,
    pub program_header_offset: &'a Class,
    pub section_header_offset: &'a Class,
    pub flags: &'a u32,
    pub elf_header_size: &'a u32,
    pub program_header_entry_size: &'a u32,
    pub program_header_number: &'a u32,
    pub section_header_entry_size: &'a u32,
    pub section_header_number: &'a u32,
    pub section_header_string_table_index: &'a u32,
}

impl<Class> ELFHeader<'_, Class> {
    fn from_c_struct(h: as_c_struct::ELFHeader<Class>) -> Self {
        Self {
            endian: if h.e_ident == 1 {
                Endian::Little
            } else {
                Endian::Big
            },
            version: if h.e_version == 0 {
                Version::None
            } else if h.e_version == 1 {
                Version::Current
            } else {
                Version::Unknown(&h.e_ident.version)
            },
            
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
pub enum Endian {
    Little,
    Big,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Version<'a> {
    None,
    Current,
    Unknown(&'a u8),
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum OSABI<'a> {
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
    Unknown(&'a u8),
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum MachineType<'a> {
    AMD64,
    Unknown(&'a u16),
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum FileType<'a> {
    None,
    Executable,
    Relocatble,
    Core,
    DynamicLibrary,
    Number,
    Unknown(&'a u16),
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ELFClass<'a> {
    None,
    ELF32,
    ELF64,
    Unknown(&'a u8),
}

// TODO: display this as hex value
#[derive(Debug, PartialEq)]
pub struct Address<'a>(&'a u64);

pub enum Error<'a> {
    InvalidMagic(&'a [u8]),
    InvalidFile,
}

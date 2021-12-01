struct ELF64

struct ELF64 {
    endian: Endian
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

#[non_exhaustive]
enum Endian {
    None,
    Little,
    Big,
}

#[non_exhaustive]
enum Version {
    None,
    Current,
}

#[non_exhaustive]
enum OSABI {
    None,
    SystemV,
    HPUX,
    NetBSD,
    GNU,
    Linux,
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
}

#[non_exhaustive]
enum MachineType {
    AMD64,
}

#[non_exhaustive]
enum FileType{
    Executable,
    Relocatble,
    Core,
    DynamicLibrary,
}

struct Address(u64);

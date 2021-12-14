#[repr(C)]
pub struct ELFHeader<Class> {
    pub e_ident: ELFIdentifier,
    pub e_type: u16,
    pub e_machine: u16,
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

impl<Class> ELFHeader<Class> {
    pub fn swap_bytes(&self) -> Self {
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

#[repr(C)]
pub struct ELFIdentifier {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub os_abi_version: u8,
    pub padding: [u8; 7],
}

pub const ELF_CLASS_OFFSET: usize = 4;

unsafe impl<Class> zero::Pod for ELFHeader<Class> {}

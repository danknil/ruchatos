use efi::Status;
use r_efi::efi::{protocols, SystemTable, self};

struct Elf64Header {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

struct Elf64ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesize: u64,
    p_memsize: u64,
    p_align: u64,
}

struct ElfApp {
    system: *mut SystemTable,
    kernel: *mut protocols::file::Protocol,
    header: Elf64Header,
    program_headers: *mut Elf64ProgramHeader,
    image_begin: u64,
    image_end: u64,
    page_size: u64,

    image_pages: u64,
    image_addr: u64,
    image_entry: u64,
}

impl ElfApp {
    fn start(&self) -> Result<(), Status> {
        let program_entry = self.image_entry as *mut unsafe extern "sysv64" fn() -> i32;
        let result = unsafe { (*program_entry)() };

        match result {
            0 => Ok(()),
            _ => Err(Status::LOAD_ERROR)
        }
    }
}

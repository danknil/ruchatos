#![no_std]
#![no_main]

pub(crate) mod elf_app;
pub(crate) mod allocator;
pub(crate) mod logger;
pub(crate) mod text_output;

use core::panic::PanicInfo;
use log::{LevelFilter, trace};
use r_efi::efi::{Boolean, Handle, Status, SystemTable};
use crate::allocator::EFIAllocator;
use crate::logger::EFILogger;

extern crate alloc;
#[global_allocator]

lazy_static! {
    static mut ALLOCATOR: EFIAllocator = EFIAllocator::new();
    static mut LOGGER: EFILogger = EFILogger::new();
    
}


type EfiResult<T> = Result<T, Status>;
#[panic_handler]
pub fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "efiapi" fn efi_main(image: Handle, system_table: SystemTable) -> Status {
    unsafe {
        ALLOCATOR.init(&system_table);
        LOGGER.init(&system_table);
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .unwrap();
    }

    match main(image, system_table) {
        Ok(_) => Status::SUCCESS,
        Err(x) => x,
    }
}

fn main(_image: Handle, system_table: SystemTable) -> EfiResult<()> {
    unsafe { ((&*system_table.con_out).reset)(system_table.con_out, Boolean::from(false)); }
    trace!("test");

    loop {}
}

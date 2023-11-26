#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate lazy_static;

use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::panic::PanicInfo;

use log::LevelFilter;
use r_efi::efi::{Handle, Status, SystemTable};

use crate::allocator::EFIAllocator;
use crate::logger::EFILogger;
use crate::stdout::EfiTextOutput;

pub(crate) mod elf_app;
pub(crate) mod allocator;
pub(crate) mod logger;
pub(crate) mod stdout;
mod stdin;

#[global_allocator]
static mut ALLOCATOR: EFIAllocator = EFIAllocator::new();
static mut LOGGER: EFILogger = EFILogger::new();

type EfiResult<T> = Result<T, Status>;

#[panic_handler]
pub fn panic(info: &PanicInfo<'_>) -> ! {
    // TODO: better panic handle
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
    let mut text_output = EfiTextOutput::new();
    text_output.init(&system_table);

    // reset screen
    text_output.reset(false)?;
    // set blue background and white foreground
    text_output.set_attribute(0x1F)?;
    // clear screen with background color we need
    text_output.clear_screen()?;

    let mut str: Vec<u16> = b"test string\r\n\0".into_iter().map(|x| *x as u16).collect();
    text_output.output_string(str.as_mut_ptr())?;

    loop {}
}

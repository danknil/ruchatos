#![no_std]
#![no_main]

// we can't use it without proper allocator
// extern crate alloc;

use core::ffi::c_void;
use core::panic::PanicInfo;
use log::{Log, Metadata, Record};

#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

// typedef struct {
//     EFI_TABLE_HEADER Hdr;
//     CHAR16 *FirmwareVendor;
//     UINT32 FirmwareRevision;
//     EFI_HANDLE ConsoleInHandle;
//     EFI_SIMPLE_TEXT_INPUT_PROTOCOL *ConIn;
//     EFI_HANDLE ConsoleOutHandle;
//     EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *ConOut;
//     EFI_HANDLE StandardErrorHandle;
//     EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL *StdErr;
//     EFI_RUNTIME_SERVICES *RuntimeServices;
//     EFI_BOOT_SERVICES *BootServices;
//     UINTN NumberOfTableEntries;
//     EFI_CONFIGURATION_TABLE *ConfigurationTable;
// } EFI_SYSTEM_TABLE;
#[repr(C)]
pub struct EfiSystemTable {
    pub header: EfiTableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub cin_handle: *mut c_void,
    pub cin: *mut c_void,
    pub cout_handle: *mut c_void,
    pub cout: *mut EfiSimpleTextOutputProtocol,
    pub stderr_handle: *mut c_void,
    pub stderr: *mut c_void,
    pub runtime_services: *mut c_void,
    pub boot_services: *mut c_void,
    pub table_entries_count: usize,
    pub cfg_table: *mut c_void,
}

// typedef struct _EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
//  EFI_TEXT_RESET Reset;
//  EFI_TEXT_STRING OutputString;
//  EFI_TEXT_TEST_STRING TestString;
//  EFI_TEXT_QUERY_MODE QueryMode;
//  EFI_TEXT_SET_MODE SetMode;
//  EFI_TEXT_SET_ATTRIBUTE SetAttribute;
//  EFI_TEXT_CLEAR_SCREEN ClearScreen;
//  EFI_TEXT_SET_CURSOR_POSITION SetCursorPosition;
//  EFI_TEXT_ENABLE_CURSOR EnableCursor;
//  SIMPLE_TEXT_OUTPUT_MODE *Mode;
// } EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    reset: unsafe extern "win64" fn(&Self, bool) -> EfiStatus,
    output_string: unsafe extern "win64" fn(&Self, *const u16) -> EfiStatus,
}

#[repr(usize)]
pub enum EfiStatus {
    SUCCESS = 0,
}

#[no_mangle]
pub extern "efiapi" fn efi_main(image: *mut c_void, system_table: EfiSystemTable) -> EfiStatus {
    let stdout = unsafe { &*(system_table.cout) };

    let hw = "Hello World".as_bytes();
    let mut loadout = [0u16; 32];

    for i in 0..hw.len() {
        loadout[i] = hw[i] as u16;
    }

    unsafe {
        (stdout.reset)(stdout, false);
        (stdout.output_string)(stdout, loadout.as_ptr());
    }

    loop {}
}

#[panic_handler]
pub fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

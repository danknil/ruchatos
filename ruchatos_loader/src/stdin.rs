use alloc::alloc::{alloc, dealloc};
use core::alloc::Layout;
use core::ffi::c_void;
use core::mem::align_of;
use core::ptr;

use r_efi::efi::{Status, SystemTable};
use r_efi::protocols::simple_text_input_ex;
use r_efi::protocols::simple_text_input_ex::{KeyData, KeyNotifyFunction};

use crate::EfiResult;

struct EFITextInput {
    stdin: *mut simple_text_input_ex::Protocol,
    key_data: *mut KeyData,
    handle: *mut c_void,
}

impl EFITextInput {
    /// creates new instance of EfiTextInput
    pub fn new(system_table: &SystemTable) -> EfiResult<Self> {
        let protocol: *mut simple_text_input_ex::Protocol = ptr::null_mut();

        unsafe {
            let result = ((&*(system_table.boot_services)).locate_protocol)(
                &mut simple_text_input_ex::PROTOCOL_GUID,
                ptr::null_mut(),
                &mut (protocol as *mut c_void),
            );

            match result {
                Status::SUCCESS => {
                    Ok(Self {
                        stdin: protocol,
                        // allocate keydata, because we need to store it for key_callback
                        key_data: alloc(Layout::from_size_align_unchecked(
                            core::mem::size_of::<KeyData>(),
                            align_of::<KeyData>(),
                        )) as *mut KeyData,
                        handle: ptr::null_mut(),
                    })
                }
                x => Err(x)
            }
        }
    }

    pub fn register(&mut self) {
        unsafe {
            // register key_callback as function to implement input
            ((&*(self.stdin)).register_key_notify)(
                self.stdin,
                self.key_data,
                Self::key_callback as KeyNotifyFunction,
                &mut self.handle,
            );
        }
    }

    pub fn unregister(&self) {
        unsafe {
            // unregister callback
            ((&*(self.stdin)).unregister_key_notify)(
                self.stdin,
                self.handle,
            );
            // dealloc keydata safely
            dealloc(self.key_data as *mut u8, Layout::from_size_align_unchecked(
                core::mem::size_of::<KeyData>(),
                core::mem::align_of::<KeyData>(),
            ))
        }
    }

    // TODO: key_callback impl
    extern "efiapi" fn key_callback(key_data: *mut KeyData) -> Status {
        Status::SUCCESS
    }
}
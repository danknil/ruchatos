use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use r_efi::efi::{BootServices, SystemTable};

pub struct EFIAllocator<'a> {
    boot_services: Option<&'a BootServices>
}

impl EFIAllocator<'_> {
    pub const fn new() -> Self {
        EFIAllocator {
            boot_services: None
        }
    }
    pub unsafe fn init(&mut self, system_table: &SystemTable) {
        self.boot_services = Some(&*(&*(system_table).boot_services))
    }
}

unsafe impl GlobalAlloc for EFIAllocator<'_> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let memory_alloc: *mut *mut u8 = core::ptr::null_mut();
        (self.boot_services.unwrap().allocate_pool)(2, layout.size(), memory_alloc as *mut *mut c_void);

        *memory_alloc
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        (self.boot_services.unwrap().free_pool)(ptr as *mut c_void);
    }
}

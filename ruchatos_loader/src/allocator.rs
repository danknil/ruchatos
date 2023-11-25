use core::{ 
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    ptr::null_mut
};
use r_efi::efi::{BootServices, SystemTable};

// marker that contains pointer to the actual data
#[repr(C)]
struct Marker(*mut u8);

// guaranted pool alignment for UEFI
const POOL_ALIGNMENT: usize = 8;

// get size with required align
fn with_alignment(size: usize, align: usize) -> usize {
    if align > POOL_ALIGNMENT {
        size + align
    } else {
        size
    }
}

unsafe fn align_block(ptr: *mut u8, align: usize) -> *mut u8 {
    if !(align > POOL_ALIGNMENT) {
        return ptr;
    }

    // 255 & 15 = 0b00010000 - (0b11111100 & 0b00001111) = 16 - 12 = 4
    let offset = align - (ptr as usize & (align - 1));

    assert!(offset >= POOL_ALIGNMENT);
    assert!(POOL_ALIGNMENT >= core::mem::size_of::<Marker>());
    assert!(POOL_ALIGNMENT >= core::mem::align_of::<Marker>());

    let aligned = ptr.add(offset);
    core::ptr::write((aligned as *mut Marker).offset(-1), Marker(ptr));
    aligned
}

unsafe fn unalign_block(ptr: *mut u8, align: usize) -> *mut u8 {
    if align > POOL_ALIGNMENT {
        core::ptr::read((ptr as *mut Marker).offset(-1)).0
    } else {
        ptr
    }
}

pub struct EFIAllocator<'a> {
    boot_services: Option<&'a BootServices>,
}

impl EFIAllocator<'_> {
    pub const fn new() -> Self {
        EFIAllocator {
            boot_services: None,
        }
    }
    pub unsafe fn init(&mut self, system_table: &SystemTable) {
        self.boot_services = Some(&*(&*(system_table).boot_services))
    }
}

unsafe impl GlobalAlloc for EFIAllocator<'_> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // you cant allocate ZSTs
        assert!(size > 0);

        let mut ptr: *mut c_void = core::ptr::null_mut();

        // return empty pointer if this is OOM
        if size.checked_add(align).is_none() {
            return ptr as *mut u8;
        }

        let result =
            unsafe { (self.boot_services.unwrap().allocate_pool)(2, layout.size(), &mut ptr) };

        if result.is_error() || ptr.is_null() {
            null_mut()
        } else {
            unsafe { align_block(ptr as *mut u8, align) }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        assert!(!ptr.is_null());
        assert!(layout.size() != 0);

        let original = unalign_block(ptr, layout.align());
        let result = (self.boot_services.unwrap().free_pool)(original as *mut _);

        assert!(!result.is_error());
    }
}

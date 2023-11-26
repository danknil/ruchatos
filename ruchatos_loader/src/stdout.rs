use r_efi::efi::{Boolean, Status, SystemTable};
use r_efi::protocols::simple_text_output;

use crate::EfiResult;

pub struct EfiTextOutput {
    con_out: *mut simple_text_output::Protocol,
}

impl EfiTextOutput {
    pub const fn new() -> Self {
        EfiTextOutput {
            con_out: core::ptr::null_mut()
        }
    }

    pub fn init(&mut self, system_table: &SystemTable) {
        self.con_out = system_table.con_out;
    }

    pub fn reset(&self, ext: bool) -> EfiResult<()> {
        match unsafe { ((&*(self.con_out)).reset)(self.con_out, Boolean::from(ext)) } {
            Status::SUCCESS => Ok(()),
            x => Err(x)
        }
    }

    pub fn clear_screen(&self) -> EfiResult<()> {
        match unsafe { ((&*(self.con_out)).clear_screen)(self.con_out) } {
            Status::SUCCESS => Ok(()),
            x => Err(x)
        }
    }

    pub fn set_attribute(&self, attribute: usize) -> EfiResult<()> {
        match unsafe { ((&*(self.con_out)).set_attribute)(self.con_out, attribute) } {
            Status::SUCCESS => Ok(()),
            x => Err(x)
        }
    }

    pub fn output_string(&self, loadout: *mut u16) -> EfiResult<()> {
        match unsafe { ((&*(self.con_out)).output_string)(self.con_out, loadout) } {
            Status::SUCCESS => Ok(()),
            x => Err(x)
        }
    }
}

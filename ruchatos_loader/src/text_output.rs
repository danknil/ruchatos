use r_efi::efi::{Boolean, Status, SystemTable};
use r_efi::protocols::simple_text_output;
use crate::EfiResult;

struct EfiTextOutput<'a> {
    con_out: &'a mut simple_text_output::Protocol,
}

impl EfiTextOutput<'_> {
    pub fn new(system_table: &SystemTable) -> Self {
        EfiTextOutput {
            con_out: unsafe { &mut *(system_table.con_out) }
        }
    }

    pub fn reset(&self) -> EfiResult<()> {
        match (self.con_out.reset)(self.con_out, Boolean::from(false)) {
            Status::SUCCESS => Ok(()),
            x => Err(x)
        }
    }

    pub fn clear_screen(&self) -> EfiResult<()> {
        match (self.con_out.clear_screen)(self.con_out) {
            Status::SUCCESS => Ok(()),
            x => Err(x)
        }
    }
    
    pub fn set_attribute(&self, attribute: usize) -> EfiResult<()> {
        match (self.con_out.set_attribute)(self.con_out, attribute) {
            Status::SUCCESS => Ok(()),
            x => Err(x)
        }
    }


}
use alloc::format;
use alloc::vec::Vec;
use log::{Level, Metadata, Record};
use r_efi::efi::SystemTable;
use r_efi::protocols::simple_text_output;

pub struct EfiLogger {
    con_out: Option<*mut simple_text_output::Protocol>
}
unsafe impl Send for EfiLogger {}
unsafe impl Sync for EfiLogger {}

impl EfiLogger {
    pub const fn new() -> Self {
        EfiLogger {
            con_out: None
        }
    }

    pub fn init(&mut self, system_table: &SystemTable) {
        self.con_out = unsafe { Some(system_table.con_out) };
    }
}

impl log::Log for EfiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let con_out = self.con_out.unwrap();

            let mut log_result = format!("[{}] ", record.level().as_str());
            if let Some(args) = record.args().as_str() {
                log_result += args;
            }

            let mut log_result = log_result.encode_utf16().collect::<Vec<_>>();
            log_result.push(0);

            unsafe { ((&*con_out).output_string)(con_out, log_result.as_mut_ptr()) };
        }
    }

    fn flush(&self) {}
}
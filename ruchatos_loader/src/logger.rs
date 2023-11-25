use alloc::format;
use alloc::vec::Vec;
use log::{Level, Metadata, Record};
use r_efi::efi::SystemTable;
use crate::text_output::EfiTextOutput;

pub struct EFILogger {
    stdout: EfiTextOutput
}
unsafe impl Send for EFILogger {}
unsafe impl Sync for EFILogger {}

impl EFILogger {
    pub fn new() -> Self {
        EFILogger {
            stdout: EfiTextOutput::new() 
        }
    }

    pub fn init(&mut self, system_table: &SystemTable) {
        self.stdout.init(system_table);
    }
}

impl log::Log for EFILogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {

            let mut log_result = format!("[{}] ", record.level().as_str());
            if let Some(args) = record.args().as_str() {
                log_result += args;
            }

            let mut log_result = log_result.encode_utf16().collect::<Vec<_>>();
            log_result.push(0);

            self.stdout.output_string(log_result.as_mut_ptr());
        }
    }

    fn flush(&self) {}
}

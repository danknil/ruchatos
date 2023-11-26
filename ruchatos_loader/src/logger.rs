use alloc::format;
use alloc::vec::Vec;

use log::{Level, Metadata, Record};
use r_efi::efi::SystemTable;

use crate::stdout::EfiTextOutput;

pub struct EFILogger {
    stdout: EfiTextOutput,
}

unsafe impl Send for EFILogger {}

unsafe impl Sync for EFILogger {}

impl EFILogger {
    pub const fn new() -> Self {
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
            match record.level() {
                // blue background and lightcyan foreground
                Level::Info => self.stdout.set_attribute(0x1B).unwrap(),
                // blue background and yellow foreground
                Level::Warn => self.stdout.set_attribute(0x1E).unwrap(),
                // red background and white foreground
                Level::Error => self.stdout.set_attribute(0x4F).unwrap(),
                _ => {}
            }

            let mut log_result = format!("[{}] ", record.level());
            if let Some(args) = record.args().as_str() {
                log_result += args;
            }

            log_result += "\r\n\0";

            let mut log_result = log_result.encode_utf16().collect::<Vec<_>>();

            self.stdout.output_string(log_result.as_mut_ptr());
            // return to default colors after output
            self.stdout.set_attribute(0x1F);
        }
    }

    fn flush(&self) {}
}

use std::{iter::Flatten, slice, vec};

use colored::Colorize;
use log::{set_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record};

pub use log::{debug, error, info, trace, warn};

#[cfg(feature = "ignore_wgpu")]
const WGPU_IGNORE_LIST: &'static [&'static str] = &["wgpu", "naga"];

static LOGGER: Logger = Logger;

pub fn init_logger() {
    set_logger(&LOGGER)
        .map(|()| set_max_level(LevelFilter::Trace))
        .expect("Could not set logger!")
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        let md = record.metadata();
        if self.enabled(md) {
            if let Some(path) = record.module_path() {
                let ignore_list: Flatten<vec::IntoIter<slice::Iter<&str>>> = vec![
                    #[cfg(feature = "ignore_wgpu")]
                    WGPU_IGNORE_LIST.iter(),
                ]
                .into_iter()
                .flatten();

                for e in ignore_list {
                    if path.contains(e) {
                        return;
                    }
                }
            }

            let level = record.level().to_string();
            let level = match record.level() {
                Level::Trace => level.magenta(),
                Level::Debug => level.bright_green(),
                Level::Info => level.bright_blue(),
                Level::Warn => level.yellow(),
                Level::Error => level.red(),
            };

            let mut log_origin = String::new();
            if let Some(file) = record.file() {
                log_origin += &format!("{file}:");
                if let Some(line) = record.line() {
                    log_origin += &line.to_string();
                }
            }

            #[rustfmt::skip]
            /* LOGGER PRINTLN! */ println!("{} {} | {}", log_origin, level, record.args());
        }
    }

    fn flush(&self) {}
}

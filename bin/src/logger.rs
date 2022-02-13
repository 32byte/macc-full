use colored::*;
use log::{Level, Metadata, Record};

pub struct CustomLogger;

impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let time = chrono::Utc::now().format("[%H:%M:%S %3f]").to_string();

            println!(
                "{}",
                color_text(format!("{} {}", time, record.args()), record)
            );
        }
    }

    fn flush(&self) {}
}

fn color_text(text: String, record: &Record) -> ColoredString {
    match record.level() {
        Level::Error => text.red().bold().reversed(),
        Level::Warn => text.bright_yellow().bold(),
        Level::Info => text.blue(),
        Level::Debug => text.white().dimmed(),
        _ => text.normal(),
    }
}

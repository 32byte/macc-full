use log::{Level, Metadata, Record};
use termion::color;

pub struct CustomLogger;

impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color: String = match record.level() {
                Level::Error => color::Fg(color::Red).to_string(),
                Level::Warn => color::Fg(color::Yellow).to_string(),
                Level::Info => color::Fg(color::Blue).to_string(),
                Level::Debug => color::Fg(color::LightBlack).to_string(),
                _ => "".to_string(),
            };

            let time = chrono::Utc::now().format("[%H:%M:%S %3f]").to_string();

            println!("{}{} {}", color, time, record.args());
        }
    }

    fn flush(&self) {}
}

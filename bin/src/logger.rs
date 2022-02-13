use log::{Level, Metadata, Record};

// TODO: implement colorful logging windows and linux
// #[cfg(not(target_os = "windows"))]
// use termion::color;

pub struct CustomLogger;

impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = ""; //self.get_prefix_color(record);

            let time = chrono::Utc::now().format("[%H:%M:%S %3f]").to_string();

            println!("{}{} {}", color, time, record.args());
        }
    }

    fn flush(&self) {}
}

/*
impl CustomLogger {
    #[cfg(target_os = "windows")]
    fn get_prefix_color(&self, _metadata: &Record) -> String {
        "".to_string()
    }

    #[cfg(not(target_os = "windows"))]
    fn get_prefix_color(&self, record: &Record) -> String {
        match record.level() {
            Level::Error => color::Fg(color::Red).to_string(),
            Level::Warn => color::Fg(color::Yellow).to_string(),
            Level::Info => color::Fg(color::Blue).to_string(),
            Level::Debug => color::Fg(color::LightBlack).to_string(),
            _ => "".to_string(),
        }
    }
}
 */

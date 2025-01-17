
use log::{self, Level, LevelFilter, Log, Metadata, Record};


struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metedata: &Metadata) -> bool{
        true
    }

    fn log(&self, record: &Record){
        if !self.enabled(record.metadata()){
            return;
        }
        
        let color = match record.level() {
            Level::Error => 31,
            Level::Warn => 93,
            Level::Info => 34,
            Level::Debug => 32,
            Level::Trace => 90,
        };
        
        crate::println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
       );

    }
    fn flush(&self) {}
}
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

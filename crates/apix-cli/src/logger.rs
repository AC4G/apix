use ansi_term::Colour::{Blue, Green, Purple, Red, Yellow};
use env_logger::Env;
use log::Level;
use std::io::Write;

pub fn init_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let ts = buf.timestamp();
            let level_str = match record.level() {
                Level::Error => Red.bold().paint(format!("{}", record.level())),
                Level::Warn => Yellow.bold().paint(format!("{}", record.level())),
                Level::Info => Green.paint(format!("{}", record.level())),
                Level::Debug => Blue.paint(format!("{}", record.level())),
                Level::Trace => Purple.paint(format!("{}", record.level())),
            };

            writeln!(buf, "[apix] [{}] [{}] {}", ts, level_str, record.args())
        })
        .init();
}

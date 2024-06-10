use log::{info, warn, error, debug, trace, LevelFilter};
use env_logger::{Env, Builder};
use std::io::Write;
use chrono::Local;

pub fn init_logger(level: LevelFilter) {
    let mut builder = Builder::from_env(Env::default().default_filter_or("info"));
    builder.filter(None, level).init();
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_warn(message: &str) {
    warn!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}

pub fn log_trace(message: &str) {
    trace!("{}", message);
}

pub fn set_custom_format() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

pub fn log_to_file(log_file: &str) {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file)
        .unwrap();

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
}

pub fn set_max_log_level(level: LevelFilter) {
    log::set_max_level(level);
}

pub fn log_custom(level: LevelFilter, message: &str) {
    match level {
        LevelFilter::Error => log_error(message),
        LevelFilter::Warn => log_warn(message),
        LevelFilter::Info => log_info(message),
        LevelFilter::Debug => log_debug(message),
        LevelFilter::Trace => log_trace(message),
    }
}

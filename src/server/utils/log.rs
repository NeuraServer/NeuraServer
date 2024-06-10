use log::{info, warn, error, debug, trace, LevelFilter};
use env_logger::{Env, Builder};
use std::io::Write;

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
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

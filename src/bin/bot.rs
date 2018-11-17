extern crate accounting;
extern crate chrono;
extern crate env_logger;

pub fn main() {
    env_logger::Builder::from_env("RUST_LOG")
        .format(accounting::log_util::format_log)
        .write_style(env_logger::WriteStyle::Auto)
        .init();
    accounting::bot::start();
}

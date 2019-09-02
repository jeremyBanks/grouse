///! Entry point for binary target.
use env_logger;

static DEFAULT_LOG_FILTER: &str = "warn,grouse=trace";

pub fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(DEFAULT_LOG_FILTER));
    grouse::run().unwrap();
}

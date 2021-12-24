use pretty_env_logger::env_logger::{Builder, Env};

pub fn init() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
}

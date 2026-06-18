use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use log::LevelFilter;



pub fn init_log() {
    let mut log_config = ConfigBuilder::new();
    log_config.set_target_level(LevelFilter::Error);
    #[cfg(debug_assertions)]
    log_config.set_location_level(LevelFilter::Error);

    TermLogger::init(
        LevelFilter::Debug,
        log_config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();
}
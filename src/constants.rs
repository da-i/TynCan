use console::style;

/// Application name
pub const APP_NAME: &str = "TynCan";

/// Application subtitle/description
pub const APP_SUBTITLE: &str = "Turn Your Node into a Castable Audio Network";

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default port for the audio streaming service
pub const DEFAULT_PORT: u16 = 8080;

/// Configuration file path
pub const CONFIG_FILE_PATH: &str = "~/.config/tyncan/config.toml";

/// Default log level
pub const DEFAULT_LOG_LEVEL: &str = "INFO";

pub const DARKICE_SOURCE: &str = "https://github.com/x20mar/darkice-with-mp3-for-raspberry-pi/blob/master/darkice_1.0.1-999~mp3+1_armhf.deb?raw=true";
pub const DARKICE_HASH: &str = "d1081c42152119e69219ab46fb2ca201fd781c4d93d3e99ef87e434e90e52e07";

/// Prints application information.
/// If `detailed` is true, prints port, config file path, and log level as well.
pub fn print_app_info(detailed: bool) {
    println!("{}", style(&format!( "🎵 {} - {}", APP_NAME, APP_SUBTITLE)).bold().green());
    println!("Version: {}", APP_VERSION);

    if detailed {
        println!("Default Port: {}", DEFAULT_PORT);
        println!("Config File Path: {}", CONFIG_FILE_PATH);
        println!("Default Log Level: {}", DEFAULT_LOG_LEVEL);
    }
}

/// Prints application information with default detail (false).
pub fn print_app_info_default() {
    print_app_info(false);
}

pub fn print_app_info_detailed() {
    print_app_info(true);
}

pub fn underdevelopment_notice() {
    println!("{}", style("This feature is under development.").yellow());
}
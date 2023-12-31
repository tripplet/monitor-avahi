use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Config {
    /// The interval in which to check the hostname use by avahi (in seconds)
    #[arg(short, long, default_value = "60", env)]
    pub check_interval: u16,

    /// Overwrite the hostname to check for
    #[arg(long, env)]
    pub overwrite_hostname: Option<String>,

    /// Avahi systemd service name
    #[arg(long, env, default_value = "avahi-daemon.service")]
    pub service_name: String,

    /// Logging level (error, info, debug)
    #[arg(short, long, env, default_value="info")]
    pub log_level: log::Level,
}

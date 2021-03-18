#[macro_use]
extern crate lazy_static;

use std::process::Command;
use std::time::Duration;

use crossbeam_channel::tick;
use dbus::blocking::Connection;
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "monitor-avahi", about = "Monitor/Restart avahi for invalid hostname")]
struct Config {
    /// The interval in which to check the hostname use by avahi (in seconds)
    #[structopt(short, long, default_value = "60")]
    check_interval: u16,

    /// Overwrite the hostname to check for
    #[structopt(long)]
    overwrite_hostname: Option<String>,

    /// Enable verbose output
    #[structopt(short)]
    verbose: bool,
}

lazy_static! {
    static ref HOSTNAME_REGEX: Regex = Regex::new(r"(?m)\[(?P<host>[^\]]+)\.local\]").unwrap();
}

fn main() {
    let cfg = Config::from_args(); // Parse arguments

    let system_hostname = if let Some(overwrite_hostname) = cfg.overwrite_hostname {
        overwrite_hostname
    } else {
        hostname::get()
            .unwrap()
            .into_string()
            .expect("hostname must be available")
            .trim()
            .into()
    };

    let timer = tick(Duration::from_secs(cfg.check_interval.into()));

    loop {
        match get_current_avahi_hostname() {
            Err(err) => eprintln!("Error communicating with avahi-daemon via dbus: {}", err),
            Ok(avahi_hostname) => {
                if cfg.verbose {
                    println!("Found running avahi with hostname '{}'", avahi_hostname);
                }
        
                if avahi_hostname != system_hostname {
                    println!("Hostname invalid, trying to restart avahi-daemon");
        
                    let status = Command::new("/usr/bin/systemctl")
                        .arg("restart")
                        .arg("avahi-daemon.service")
                        .status();
        
                    let status_str = status.map_or_else(
                        |err| err.to_string(),
                        |s| s.code().map_or("Process terminated".into(), |c| c.to_string())
                    );
        
                    println!("Restarted avahi-daemon because of name conflict, result: {}", status_str);
                }
            }
        }

        if cfg.verbose {
            println!("Checking again in {}s", cfg.check_interval);
        }

        let _ = timer.recv();
    }
}

fn get_current_avahi_hostname() -> Result<String, Box<dyn std::error::Error>> {
    // Open connection to the system bus and create proxy wrapper
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy("org.freedesktop.Avahi", "/", Duration::from_millis(5000));

    let (avahi_hostname,): (String,) = proxy.method_call("org.freedesktop.Avahi.Server", "GetHostName", ())?;
    Ok(avahi_hostname)
}

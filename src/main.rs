#[macro_use]
extern crate lazy_static;

use std::process::Command;
use std::time::Duration;

use crossbeam_channel::tick;
use regex::Regex;
use structopt::StructOpt;
use sysinfo::{ProcessExt, System, SystemExt};

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

    let mut system = System::new();
    let timer = tick(Duration::from_secs(cfg.check_interval.into()));

    loop {
        if let Some(avahi_hostname) = get_avahi_hostname(&mut system) {
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
        } else {
            if cfg.verbose {
                println!("Avahi not running/found");
            }
        }

        if cfg.verbose {
            println!("Checking again in {}s", cfg.check_interval);
        }

        let _ = timer.recv();
    }
}

fn get_avahi_hostname(system: &mut System) -> Option<String> {
    system.refresh_processes();

    let avahi = system.get_processes().into_iter().find(|(_, proc)| {
        proc.cmd()
            .get(0)
            .map_or(false, |c| c.starts_with("avahi-daemon: running"))
    });

    Some(
        HOSTNAME_REGEX
            .captures(&avahi?.1.cmd()[0])?
            .name("host")?
            .as_str()
            .trim()
            .into(),
    )
}

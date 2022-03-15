use clap::Parser;
use dbus::blocking::Connection;
use std::{thread, time::Duration};

#[derive(Parser)]
#[clap(author, version, about)]
struct Config {
    /// The interval in which to check the hostname use by avahi (in seconds)
    #[clap(short, long, default_value = "60", env)]
    check_interval: u16,

    /// Overwrite the hostname to check for
    #[clap(long, env)]
    overwrite_hostname: Option<String>,

    /// Avahi systemd service name
    #[structopt(long, default_value_t = String::from("avahi-daemon.service"), env)]
    service_name: String,

    /// Enable verbose output
    #[clap(short)]
    verbose: bool,
}

fn main() {
    let cfg = Config::parse();

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

    let delay = Duration::from_secs(cfg.check_interval.into());

    loop {
        match get_current_avahi_hostname() {
            Err(err) => eprintln!("Error communicating with avahi-daemon via dbus: {}", err),
            Ok(avahi_hostname) => {
                if cfg.verbose {
                    println!("Found running avahi with hostname '{}'", avahi_hostname);
                }

                if avahi_hostname != system_hostname {
                    println!("Hostname invalid, trying to restart avahi-daemon");

                    match restart_service(&cfg.service_name) {
                        Err(err) => eprintln!("Error restarting serivce: {}", err),
                        Ok(result) => println!(
                            "Restarted avahi-daemon because of name conflict, result: {}",
                            result
                        ),
                    }
                }
            }
        }

        if cfg.verbose {
            println!("Checking again in {}s", cfg.check_interval);
        }

        thread::sleep(delay);
    }
}

fn get_current_avahi_hostname() -> Result<String, Box<dyn std::error::Error>> {
    // Open connection to the system bus and create proxy wrapper
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy("org.freedesktop.Avahi", "/", Duration::from_millis(5000));

    let (avahi_hostname,): (String,) = proxy.method_call("org.freedesktop.Avahi.Server", "GetHostName", ())?;
    Ok(avahi_hostname)
}

fn restart_service(service_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy("org.freedesktop.systemd1", "/org/freedesktop/systemd1", Duration::from_millis(5000));

    let (job_path,): (dbus::strings::Path,) = proxy.method_call(
        "org.freedesktop.systemd1.Manager",
        "TryRestartUnit",
        (service_name, "replace"),
    )?;
    Ok(job_path.into_cstring().into_string()?)
}

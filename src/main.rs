#[macro_use]
extern crate lazy_static;

use std::time::Duration;

use crossbeam_channel::tick;
use regex::Regex;
use sysinfo::{ProcessExt, System, SystemExt};

lazy_static! {
    static ref HOSTNAME_REGEX: Regex = Regex::new(r"(?m)\[(?P<host>[^\]]+)\]").unwrap();
}

fn main() {
    let mut now = std::time::Instant::now();    
    let mut system = System::new();
    let timer = tick(Duration::from_millis(2000));
    loop {
        if let Some(hostname) = get_hostname(&mut system) {
            println!("{}i [{}ms]", hostname, now.elapsed().as_millis());
        } else {
            println!("Avahi not running/found");
        }
        
        let _ = timer.recv();
        now = std::time::Instant::now();
    }
}

fn get_hostname(system: &mut System) -> Option<String> {
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
            .into(),
    )
}

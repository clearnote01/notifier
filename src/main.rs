extern crate notify_rust;
extern crate regex;

use notify_rust::Notification;
use std::{thread, time};

use std::process::Command;
use regex::Regex;

struct Config {
    pub check_interval: u64, // in seconds
    pub battery_threshold: i8 // in percengtage
}

fn get_default_conf() -> Config {
    Config {
        check_interval: 60*10, // 10 minutes
        battery_threshold: 20
    }
}

fn notify_msg(summary: &str, body: &str, icon: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .icon(icon)
        .show().unwrap();
}

fn cur_battery() -> i8 {
    let acpi = Command::new("acpi").arg("-b").output().unwrap();
    let acpi = String::from_utf8(acpi.stdout).unwrap();
    let re = Regex::new(r"(?x)
        \s(?P<bat>\d+)%  # battery percentage
    ").unwrap();
    let percent = re.captures(&acpi).unwrap();
    let percent: i8 = percent["bat"].parse().unwrap();
    percent
}
 
fn main() {
    let config = get_default_conf();
    let mut battery: i8; 
    loop {
        battery = cur_battery();
        println!("current battery is: {:?}%", battery );
        if battery < config.battery_threshold {
            notify_msg(
                "LOW BATTERY",
                "PUT CHARGER PLEASE",
                "dialog-information"
                      );
        }
        let duration = time::Duration::from_secs(config.check_interval);
        thread::sleep(duration);
    }
}

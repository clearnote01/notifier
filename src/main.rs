extern crate notify_rust;
extern crate regex;

use notify_rust::Notification;
use std::{thread, time};

use std::process::Command;
use regex::Regex;

enum BatteryStatus {
    CHARGING,
    DISCHARGING
}

struct Config {
    pub check_interval: u64, // in seconds
    pub battery_threshold: i8 // in percengtage
}

fn get_default_conf() -> Config {
    Config {
        check_interval: 60*5, // 5 minutes
        battery_threshold: 20 // 20 percent
    }
}

fn notify_msg(summary: &str, body: &str, icon: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .icon(icon)
        .show().unwrap();
}

fn cur_battery() -> (i8, BatteryStatus) {
    let acpi = Command::new("acpi").arg("-b").output().unwrap();
    let acpi = String::from_utf8(acpi.stdout).unwrap();
    let re = Regex::new(r"(?x)
        (?P<chr>Discharging|Charging) # matched status
        ,\s
        (?P<bat>\d+)%  # battery percentage
    ").unwrap();
    let matches = re.captures(&acpi).unwrap();
    let percent: i8 = matches["bat"].parse().unwrap();
    let chr: String = matches["chr"].parse().unwrap();
    let chr_status: BatteryStatus = match chr.as_str() {
        "Charging" => BatteryStatus::CHARGING,
        "Discharging" => BatteryStatus::DISCHARGING,
        _ => BatteryStatus::DISCHARGING // if some pattern does nto match
    };
    (percent, chr_status)
}
 
fn main() {
    let config = get_default_conf();
    loop {
        let (battery, status) = cur_battery();
        println!("current battery is: {:?}%", battery );
        if battery < config.battery_threshold {
            if let BatteryStatus::DISCHARGING = status {
                notify_msg(
                    "LOW BATTERY",
                    "PUT CHARGER PLEASE",
                    "dialog-information"
                          );
            }

        }
        let duration = time::Duration::from_secs(config.check_interval);
        thread::sleep(duration);
    }
}

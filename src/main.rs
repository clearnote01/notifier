use notify_rust::{Timeout, Notification};
use regex::Regex;
use clap::clap_app;
use std::{thread, time};
use std::process::Command;


enum BatteryStatus {
    CHARGING,
    DISCHARGING
}

struct BatConfig {
    pub check_interval: u64, // in seconds
    pub battery_threshold: i8 // in percengtage
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

fn run_bat_loop(config: BatConfig) {
    loop {
        let (battery, status) = cur_battery();
        println!("current battery is: {:?}%", battery );
        if battery < config.battery_threshold {
            if let BatteryStatus::DISCHARGING = status {
                Notification::new()
                    .summary("You laptop battery is low")
                    .body("PUT YOUR CHARGER PLEASE")
                    .icon("battery-caution")
                    .timeout(Timeout::Milliseconds(10000))
                    .show().unwrap();
            }

        }
        let duration = time::Duration::from_secs(config.check_interval * 60); // convert minutes to secs
        thread::sleep(duration);
    }
}
 
fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "clearnote01")
        (about: "Sends notification to remind you of things")
        (@subcommand bat =>
                        (about: "Notification about battery low at frequenct intervals")
                        (@arg check_interval: -i --interval default_value("5") +takes_value "when to trigger notificaton (in minutes)")
                        (@arg battery_threshold: -t --threshold default_value("20") +takes_value "below this percentage start notification")
        )
    ).get_matches();

    if let Some (bat_matches) = matches.subcommand_matches("bat") {
        let check_interval = bat_matches.value_of("check_interval").unwrap();
        let check_interval: u64 = check_interval.parse().unwrap();
        let battery_threshold = bat_matches.value_of("battery_threshold").unwrap();
        let battery_threshold: i8 = battery_threshold.parse().unwrap();

        let bat_conf = BatConfig {
            check_interval,
            battery_threshold
        };
        run_bat_loop(bat_conf);
    };
}

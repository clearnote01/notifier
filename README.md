# notifier-rs
A simple service to send linux system notification written in rust. Written because I wanted notifications for low-battery to stay longer and give me more warnings

## Actions
- Sends a battery low notification when it gets lower than 20%, checks every 5 minutes.

## CONFIGURE
```
USAGE:
    notifier bat [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --threshold <battery_threshold>    below this percentage start notification [default: 20]
    -i, --interval <check_interval>        when to trigger notificaton (in minutes) [default: 5]
```

## Requirements
- Only work on linux based GUI distros (libnotify/dbus dependency)

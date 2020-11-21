use std::process::Command;
use std::str;

use chrono::prelude::*;

fn main() {
    let vpn_status = get_vpn_status();
    let date = get_date();
    println!("{} | {}", vpn_status, date);
}

fn get_vpn_status() -> String {
    let output = Command::new("nmcli")
        .arg("connection")
        .output()
        .expect("failed to run nmcli");

    let output = str::from_utf8(&output.stdout).expect("Got invalid utf-8 on stdout");

    for line in output.lines() {
        if !line.contains("wireguard") {
            continue;
        }
        let cols = line.split_whitespace();
        let server = cols.last().unwrap();
        return String::from(format!("VPN: {}", server));
    }

    String::from("VPN: not connected")
}

fn get_date() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%a %b %e %H:%M").to_string()
}

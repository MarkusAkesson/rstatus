use chrono::prelude::*;
use reqwest;
use serde_json::Value;

fn main() {
    let vpn_status = get_vpn_status();
    let date = get_date();
    println!("{} | {}", vpn_status, date);
}

fn get_vpn_status() -> String {
    let res = reqwest::get("https://am.i.mullvad.net/json");

    let status = match res {
        Err(_) => String::from("VPN: Error"),
        Ok(_) => {
            let body: Value = res.unwrap().json().unwrap();
            let connected = if body["mullvad_exit_ip"].as_bool().unwrap() {
                String::from("Connected")
            } else {
                String::from("Disconnected")
            };
            format!(
                "VPN status: {} | VPN Server: {}",
                connected, body["mullvad_exit_ip_hostname"]
            )
        }
    };

    status
}

fn get_date() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%a %b %e %H:%M").to_string()
}

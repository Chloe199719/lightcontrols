use std::env;

use serde::{ Serialize, Deserialize };
#[derive(Serialize, Deserialize, Debug)]
struct Light {
    on: Option<bool>,
    hue: Option<u16>,
    bri: Option<u8>,
    sat: Option<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut light = Light {
        on: None,
        hue: None,
        bri: None,
        sat: None,
    };
    let mut light_number: u8 = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: hue-cli [options]");
        println!("Options:");
        println!("--light <light number>");
        println!("--hue <0-65535>");
        println!("--bri <0-254>");
        println!("--sat <0-254>");
        println!("--toggle <on/off>");
        return Ok(());
    }
    for (i, arg) in args.iter().enumerate() {
        match arg.to_lowercase().as_str() {
            "--light" => {
                light_number = args[i + 1].parse::<u8>().unwrap();
            }
            "--hue" => {
                light.hue = Some(args[i + 1].parse::<u16>().unwrap());
            }
            "--bri" => {
                light.bri = Some(args[i + 1].parse::<u8>().unwrap());
            }
            "--sat" => {
                light.sat = Some(args[i + 1].parse::<u8>().unwrap());
            }
            "--toggle" => {
                light.on = match args[i + 1].to_lowercase().as_str() {
                    "on" => Some(true),
                    "off" => Some(false),
                    _ => Some(false),
                };
            }
            _ => {}
        }
    }

    let client = reqwest::Client::new();
    let url =
        format!("http://192.168.2.115/api/xsUfuAh0yY58fxuB0eA3HGWe1JdHnCkzDMAzP5ys/lights/{}/state", light_number);
    // println!("light: {:?}, url {}", light, url);

    let serialized = serde_json::to_string(&light)?;

    let _ = client.put(url).body(serialized).send().await?;

    Ok(())
}

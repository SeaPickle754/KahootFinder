use reqwest::Client;
use std::process::Command;
use rand::prelude::*;

const NAME:&str = "lava chicken";   
const PIN_CHECKS:u32 = 250;


fn click_at(x: u32, y: u32) {
    // Move mouse
    Command::new("xdotool")
        .args(["mousemove", &x.to_string(), &y.to_string()])
        .status()
        .expect("failed to move mouse");

    // Click left mouse button
    Command::new("xdotool")
        .args(["click", "1"])
        .status()
        .expect("failed to click mouse");
}
fn open_and_type(pin: u32, nickname: &str) {
    // Open kahoot.it in Firefox

    // Sleep to wait for the page to load
    std::thread::sleep(std::time::Duration::from_secs(4));
    click_at(1278,584);
    // Simulate typing the pin and pressing Enter
    Command::new("xdotool")
        .args(["type", &pin.to_string()])
        .status()
        .expect("failed to type pin");

    Command::new("xdotool")
        .args(["key", "Return"])
        .status()
        .expect("failed to press Enter");

    // Sleep for the name field to load
    std::thread::sleep(std::time::Duration::from_secs(3));
    click_at(1277,584);
    // Type nickname
    Command::new("xdotool")
        .args(["type", nickname])
        .status()
        .expect("failed to type nickname");

    Command::new("xdotool")
        .args(["key", "Return"])
        .status()
        .expect("failed to press Enter");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let mut nums: Vec<u32> = (100_000..1_000_000).collect();
    nums.shuffle(&mut rng);
    // only run this once 
    Command::new("firefox")
        .arg("--new-tab")
        .arg("https://kahoot.it")
        .spawn()
        .expect("failed to open Firefox");
    
    let client = Client::new();

    for i in 0..PIN_CHECKS{
        
        let pin: u32 = nums[i as usize];
        let url = format!("https://kahoot.it/reserve/session/{}", pin);
        let res = client.get(&url).send().await?;

        if res.status().is_success() {
            println!("!!! PIN FOUND : {pin} !!! {i}/{PIN_CHECKS}");
            open_and_type(pin, NAME);
        } else {
            println!("❌ PIN {pin} is not valid. {i}/{PIN_CHECKS}");
        }
    }
    // random return value (needed)
    Ok(())
}

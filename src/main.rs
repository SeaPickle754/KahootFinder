use reqwest::Client;
use rand::Rng;
use serde::Deserialize;
use quick_js::{Context, JsValue};
use std::process::Command;
use rand::prelude::*;
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
    Command::new("firefox")
        .arg("--new-tab")
        .arg("https://kahoot.it")
        .spawn()
        .expect("failed to open Firefox");

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

#[derive(Debug, Deserialize)]
struct KahootSessionResponse {
    challenge: Option<String>,
    sessionToken: Option<String>,
    status: Option<String>,
    message: Option<String>,
}

fn solve_challenge(js_code: &str) -> Option<String> {
    let context = Context::new().ok()?;
    let result = context.eval(js_code).ok()?;
    match result {
        JsValue::String(s) => Some(s),
        JsValue::Int(i) => Some(i.to_string()),
        JsValue::Float(f) => Some(f.to_string()),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let mut nums: Vec<u32> = (100_000..1_000_000).collect();
    nums.shuffle(&mut rng);
    for i in 0..250{
        let client = Client::new();
        let name = "lava chicken";
        let pin: u32 = nums[i];
        let url = format!("https://kahoot.it/reserve/session/{}", pin);

        let res = client.get(&url).send().await?;

        if res.status().is_success() {
            let body: KahootSessionResponse = res.json().await?;
            println!("!!! PIN FOUND : {pin} !!!");
            if let Some(challenge_js) = &body.challenge {
                println!("✅ Challenge received.");
                open_and_type(pin, &name);
               // if let Some(response_token) = solve_challenge(challenge_js) {
               //     println!("🎉 Solved challenge: {}", response_token);
               //     println!("Now you can try to join the game with sessionToken + challenge response.");
               // } else {
               //     println!("💀 Failed to solve the challenge.");
               // }
            } else if let Some(msg) = &body.message {
                println!("⚠️ Cannot join game: {}", msg);
            } else {
                println!("❌ Unknown response. Game may not be joinable.");
            }
        } else {
            println!("❌ PIN {} is not valid.", pin);
        }
    }
    Ok(())
}

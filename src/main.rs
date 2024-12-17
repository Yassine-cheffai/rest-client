// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value;
use slint::SharedString;
use std::error::Error;

slint::include_modules!();

#[tokio::main]
async fn get_request(url: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.json::<Value>().await?;
    return Ok(resp);
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_send({
        let ui_handle = ui.as_weak();
        move |url, method| {
            println!("calling url: {}, with method: {}", url, method);
            let ui = ui_handle.unwrap();
            let method: String = method.into();
            match get_request(url.as_str()) {
                Ok(value) => {
                    let pretty_response =
                        format!("{}", serde_json::to_string_pretty(&value).unwrap());
                    ui.set_response(pretty_response.into());
                }
                Err(error) => {
                    println!("error: call failed due to: {:?}", error);
                }
            };
        }
    });

    ui.run()?;
    Ok(())
}

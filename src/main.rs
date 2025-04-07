// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value;
use slint::SharedString;
use std::error::Error;
use std::time::Instant;

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

            let start = Instant::now();
            let result = get_request(url.as_str());
            let duration = start.elapsed();

            let request_duration = format!("{:?}", duration);
            // ui.set_request_duration(request_duration.into());
            ui.set_request_duration(SharedString::from(request_duration));
            match result {
                Ok(value) => {
                    // let pretty_response =
                    // format!("{}", serde_json::to_string_pretty(&value).unwrap());
                    // ui.set_response(pretty_response.into());
                    // ui.set_response(SharedString::from(pretty_response));
                    match serde_json::to_string_pretty(&value) {
                        Ok(value) => {
                            ui.set_response(SharedString::from(value));
                        }
                        Err(err) => {
                            ui.set_response(SharedString::from("failed"));
                        }
                    }
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

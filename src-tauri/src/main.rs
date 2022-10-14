#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    /*collections::HashMap,*/
    fs/* ,
    io::{self, Write},*/
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

//static tokenxbox:&'static str = "YOUR STRING HERE";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*#[tauri::command]
async fn greet()-> String  {
    /*let token= token().await;
    format!("Result: {:#?}", token)*/
    format!("I was invoked from JS!")
}*/



#[derive(Deserialize, Serialize, Debug)]
struct MinecraftAuthenticationResponse {
    /// Some UUID of the account
    username: String,
    /// The minecraft JWT access token
    access_token: String,
    /// The type of access token
    token_type: String,
    /// How many seconds until the token expires
    expires_in: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MinecraftProfileResponse {
    /// The UUID of the account
    id: String,
    /// The name of the user
    name: String,
}

/*#[tauri::command]
async fn token() -> Result<String , reqwest::Error> {
    let client = Client::new();
    let minecraft_resp: MinecraftAuthenticationResponse = Client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&json!({
            "identityToken":
                format!(
                    "XBL3.0 x={user_hash};{xsts_token}",
                    user_hash = 12312313,
                    xsts_token = "asdasd"
                )
        }))
        .send()
        .await?
        .json()
        .await?;
    /*fs::write(
        "minecraft_token.json",
        serde_json::to_string(&minecraft_resp)?,
    )?;*/

    let minecraft_token: &String = &minecraft_resp.access_token;
    println!("{:#?}", minecraft_token);
  Ok(minecraft_token.to_string())
}*/

mod xboxlive;

//#[tokio::main]
fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![xboxlive::funxbox])
    .run(tauri::generate_context!())
    .expect("error while running tauri application")
}

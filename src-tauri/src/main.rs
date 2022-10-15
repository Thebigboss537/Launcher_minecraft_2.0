
/*#[tauri::command]
async fn token() -> Result {
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

use std::any::Any;

use oauth2::AuthType;
use serde_json::json;
use reqwest::Client;
use reqwest::StatusCode;
use tokio::time::error;
use reqwest::Error;
use serde::{ Serialize, Deserialize };

#[tauri::command]
async fn principal(userhash: String, xststoken: String)-> String {
    let a = asdsasda(userhash,xststoken).await;
    format!("{:?}", a)
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
}



async fn asdsasda(userhash: String,xststoken: String) ->  Result<String,Error> {
    let client = Client::new();

    let body1 = json!({
        "identityToken" : format!("XBL3.0 x={};{}",userhash, xststoken),
        "ensureLegacyEnabled" : true
    });

    let request_url1 = "https://api.minecraftservices.com/authentication/login_with_xbox";
    let response1 = client
        .post(request_url1)
        .json(&body1)
        .send().await?;
        
    let gist:AuthResponse = response1.json().await?;
    let token:String = gist.access_token;
    //println!("{:?}", token);
    
    

    let request_url2 = "https://api.minecraftservices.com/minecraft/profile";
    let response2:Profile = client
        .get(request_url2)
        .bearer_auth(&token)
        .send().await?
        .json().await?;
        
    let name = response2.name;
    //let token:String = gist.access_token;


    Ok(name)
}



//#[tokio::main]
fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![principal])
    .run(tauri::generate_context!())
    .expect("error while running tauri application")
}

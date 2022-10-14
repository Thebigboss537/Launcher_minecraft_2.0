#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
};


use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// The Azure Application client ID
const CLIENT_ID: &str = "04e788a5-9628-4244-a9f3-8583a9351800";

/// The response from authenticating with Microsoft OAuth flow
#[derive(Deserialize, Serialize)]
struct AuthorizationTokenResponse {
    /// The type of token for authentication
    token_type: String,
    /// The scope we have access to
    scope: String,
    /// Seconds until the authentication token expires
    expires_in: u32,
    /// Seconds until the authentication token expires
    ext_expires_in: u32,
    /// The authentication token itself
    access_token: String,
    /// The token used for refreshing access
    refresh_token: String,
    /// The ID of the token
    id_token: String,
}

/// The response from Xbox when authenticating with a Microsoft token
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveAuthenticationResponse {
    /// An ISO-8601 timestamp of when the token was issued
    issue_instant: String,
    /// An ISO-8601 timestamp of when the token expires
    not_after: String,
    /// The xbox authentication token to use
    token: String,
    /// An object that contains a vec of `uhs` objects
    /// Looks like { "xui": [{"uhs": "xbl_token"}] }
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

/// The response from Minecraft when attempting to authenticate with an xbox token
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

/// The response from Minecraft when attempting to retrieve a users profile
#[derive(Serialize, Deserialize, Debug)]
struct MinecraftProfileResponse {
    /// The UUID of the account
    id: String,
    /// The name of the user
    name: String,
}



#[tauri::command]
pub async fn funxbox() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // step 1: attempt to login to microsoft account (OAuth flow)
    // requires authorization from the user
    println!("Generating login page...");
    let login_html = client
        .get("https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize")
        .form(&vec![
            ("client_id", CLIENT_ID),
            ("response_type", "code"),
            ("scope", "XboxLive.signin%20offline_access"),
        ])
        .send()
        .await?
        .text()
        .await?;

    // save html to user and tell them to open it
    fs::write("index.html", login_html)?;
    println!("Saved login page to index.html, please open the file and enter the token generated");

    // retrieve the code from them the user
    let code = {
        let mut buffer = String::new();

        print!("Authorization code: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        buffer
    };

    // step 2: convert authorization code into authorization token
    let authorization_token = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&vec![
            ("client_id", CLIENT_ID),
            ("code", &code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", "PASTE_AZURE_APPLICATION_REDIRECT_URI"),
        ])
        .send()
        .await?
        .json::<AuthorizationTokenResponse>()
        .await?;

    println!("Access token: {:?}", &authorization_token.access_token);
    fs::write(
        "authorization_token.json",
        serde_json::to_string_pretty(&authorization_token)?,
    )?;

    let authorization_token: AuthorizationTokenResponse =
        serde_json::from_slice(&fs::read("authorization_token.json")?)?;

    // step 3: authenticate with xbox live
    let xbox_authenticate_json = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": &format!("d={}", authorization_token.access_token)
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });
    println!("{:#?}", xbox_authenticate_json);

    let xbox_resp: XboxLiveAuthenticationResponse = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&xbox_authenticate_json)
        .send()
        .await?
        .json()
        .await?;
    fs::write("xbox_token.json", serde_json::to_string_pretty(&xbox_resp)?)?;

    let xbox_token = &xbox_resp.token;
    let user_hash = &xbox_resp.display_claims["xui"][0]["uhs"];

    println!("{:#?}", xbox_resp);

    // step 4: convert xbox token into xbox security token
    let xbox_security_token_resp: XboxLiveAuthenticationResponse = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbox_token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }))
        .send()
        .await?
        .json()
        .await?;
    fs::write(
        "xbox_security_token.json",
        serde_json::to_string_pretty(&xbox_security_token_resp)?,
    )?;

    let xbox_security_token = &xbox_security_token_resp.token;

    println!("{:#?}", xbox_security_token_resp);

    // step 5: authenticate with minecraft
    let minecraft_resp: MinecraftAuthenticationResponse = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&json!({
            "identityToken":
                format!(
                    "XBL3.0 x={user_hash};{xsts_token}",
                    user_hash = user_hash,
                    xsts_token = xbox_security_token
                )
        }))
        .send()
        .await?
        .json()
        .await?;
    fs::write(
        "minecraft_token.json",
        serde_json::to_string(&minecraft_resp)?,
    )?;

    let minecraft_token = &minecraft_resp.access_token;
    println!("{:#?}", minecraft_resp);

    // step 6: retrieve the users profile using the minecraft token
    let minecraft_profile_resp: MinecraftProfileResponse = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(minecraft_token)
        .send()
        .await?
        .json()
        .await?;
    fs::write(
        "minecraft_profile.json",
        serde_json::to_string(&minecraft_profile_resp)?,
    )?;

    println!("{:#?}", minecraft_profile_resp);

    Ok(())
}
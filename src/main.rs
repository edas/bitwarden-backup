use anyhow::Result;
use clap::Parser;
use reqwest;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(long, value_name = "FILE", value_delimiter = '=')]
    config: String,

    /// Directory path where to export the data
    #[arg(value_name = "DIR")]
    output_dir: PathBuf,
}

#[derive(Debug, Deserialize)]
struct LocalConf {
    email: String,
    api_url: String,
    identity_url: String,
    client_id: String,
    client_secret: String,
    scope: String,
    device_type: String,
    device_identifier: String,
    device_name: String,
    grant_type: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read and parse the configuration file
    let config_content = fs::read_to_string(&args.config)?;
    let config: LocalConf = serde_yaml::from_str(&config_content)?;
    
    let user_agent = "survol/1.0".to_string();

    // Create prelogin request to identity URL
    let prelogin_identity_url = format!("{}/accounts/prelogin", config.identity_url);
    
    let prelogin_payload = serde_json::json!({
        "email": config.email
    });

    let client = reqwest::Client::new();
    let prelogin_identity_response = client
        .post(prelogin_identity_url)
        .header("user-agent", user_agent.clone())
        .json(&prelogin_payload)
        .send()
        .await?;

    let prelogin_identity_json = prelogin_identity_response.json::<serde_json::Value>().await?;

    // Save identity prelogin response to file
    let prelogin_identity_file = args.output_dir.join(format!("bitwarden.{}.prelogin.json", config.email));
    let prelogin_identity_str = serde_json::to_string_pretty(&prelogin_identity_json)?;
    fs::write(&prelogin_identity_file, prelogin_identity_str)?;


    // Create token request
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/x-www-form-urlencoded; charset=utf-8".parse()?
    );
    headers.insert(
        reqwest::header::ACCEPT,
        "application/json".parse()?
    );
    headers.insert("Device-Type", "21".to_string().parse()?);
    headers.insert("user-agent", user_agent.parse()?);
    //headers.insert("Auth-Email", URL_SAFE_NO_PAD.encode(config.email.as_bytes()).parse()?);

    let token_url = format!("{}/connect/token", config.identity_url);
    
    let form_data = [
        ("scope", config.scope.as_str()),
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
        ("deviceType", config.device_type.as_str()),
        ("deviceIdentifier", config.device_identifier.as_str()),
        ("deviceName", config.device_name.as_str()),
        ("grant_type", config.grant_type.as_str()),
    ];

    let client = reqwest::Client::new();
    let response = client
        .post(token_url)
        .headers(headers)
        .form(&form_data)
        .send()
        .await?;

    let token_response = response.json::<serde_json::Value>().await?;

    // Save token response to file
    let token_file = args.output_dir.join(format!("bitwarden.{}.token.json", config.email));
    let token_json = serde_json::to_string_pretty(&token_response)?;
    fs::write(&token_file, token_json)?;


    // Get profile from API
    let profile_url = format!("{}/accounts/profile", config.api_url);
    let access_token = token_response["access_token"].as_str().unwrap();
    
    let mut auth_headers = reqwest::header::HeaderMap::new();
    auth_headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", access_token).parse()?
    );
    auth_headers.insert(
        reqwest::header::ACCEPT,
        "application/json".parse()?
    );
    auth_headers.insert(
        reqwest::header::USER_AGENT,
        user_agent.parse()?
    );

    let profile_response = client
        .get(profile_url)
        .headers(auth_headers.clone())
        .send()
        .await?;

    let profile_json = profile_response.json::<serde_json::Value>().await?;

    // Save profile response to file
    let profile_file = args.output_dir.join(format!("bitwarden.{}.profile.json", config.email));
    let profile_json_str = serde_json::to_string_pretty(&profile_json)?;
    fs::write(&profile_file, profile_json_str)?;
    


    // Get sync data from API
    let sync_url = format!("{}/sync", config.api_url);
    
    let sync_response = client
        .get(sync_url)
        .headers(auth_headers)
        .send()
        .await?;

    let sync_json = sync_response.json::<serde_json::Value>().await?;

    // Save sync response to file
    let sync_file = args.output_dir.join(format!("bitwarden.{}.sync.json", config.email));
    let sync_json_str = serde_json::to_string_pretty(&sync_json)?;
    fs::write(&sync_file, sync_json_str)?;


    Ok(())
}

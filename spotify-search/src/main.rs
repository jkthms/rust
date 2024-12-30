use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::collections::HashMap;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls{
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist{
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album{
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track{
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse{
    tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T>{
    items: Vec<T>
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

async fn get_token(client_id: String, client_secret: String) -> String {
    // The URL to get the access token
    let url = "https://accounts.spotify.com/api/token";

    // Create the client to make requests
    let client = reqwest::Client::new();

    // Prepare the form data for x-www-form-urlencoded
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id.as_str());
    params.insert("client_secret", client_secret.as_str());

    // Send the POST request
    let response = client
        .post(url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params) // Attach the form data as application/x-www-form-urlencoded
        .send()
        .await
        .unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        // Deserialize the response into a TokenResponse struct
        let token_response: TokenResponse = response.json().await.unwrap();
        
        // Print the access token and its expiration
        println!("Access Token: {}", token_response.access_token);
        println!("Expires In: {} seconds", token_response.expires_in);

        return token_response.access_token;
    } else {
        // Handle the error (you could also print more details here)
        println!("Failed to retrieve the access token. Status: {}", response.status());
        return "".to_string();
    }
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("Track: {}", track.name);
    }
}

// tokio for async processing
#[tokio::main]
async fn main() {
    // Load the .env file
    dotenv().ok();  // This loads the environment variables from .env

    // Read the environment variables for client_id and client_secret
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set");

    let auth_token = get_token(client_id, client_secret).await;

    // this is using the ::env import
    let args: Vec<String> = env::args().collect();

    let search_query = &args[1];

    let url = format!("https://api.spotify.com/v1/search?q={}&type=track", search_query);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .send()
        .await
        .unwrap();

    match response.status(){
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => {
                    let tracks = parsed.tracks.items;
                    print_tracks(tracks.iter().collect());
                }
                Err(e) => {
                    println!("Error: Response did not match the shape of APIResponse {}", e);
                }
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Error: Unauthorized");
        }
        _ => {
            println!("Error: {}", response.status());
        }
    }
}

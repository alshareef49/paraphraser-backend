#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket_cors::{CorsOptions};
use reqwest::Client;
use std::env;
use dotenv::dotenv;
use rocket::serde::json::serde_json;

#[get("/")]
fn index() -> &'static str {
    "🚀 Paraphraser API is running!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ParaphraseRequest {
    text: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ParaphraseResponse {
    paraphrased: String,
}

#[post("/paraphrase", format = "application/json", data = "<req>")]
async fn paraphrase_handler(req: Json<ParaphraseRequest>) -> Result<Json<ParaphraseResponse>, Status> {
    match call_openrouter(&req.text).await {
        Ok(paraphrased_text) => Ok(Json(ParaphraseResponse {
            paraphrased: paraphrased_text,
        })),
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(Status::InternalServerError)
        }
    }
}

async fn call_openrouter(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENROUTER_API_KEY")?;
    let client = Client::new();

    let body = serde_json::json!({
        "model": "mistralai/mistral-7b-instruct",
        "messages": [
            {
                "role": "user",
                "content": format!("Just Paraphrase this text and not return any other thing: {}", text)
            }
        ]
    });

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "http://localhost:5173")
        .header("X-Title", "Paraphraser App")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    println!("OpenRouter Response: {:#}", json); // log response

    let paraphrased = json
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .and_then(|msg| msg.get("content"))
        .and_then(|content| content.as_str())
        .unwrap_or("No paraphrased content returned.")
        .trim()
        .to_string();

    Ok(paraphrased)
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore
) -> shuttle_rocket::ShuttleRocket {
    dotenv().ok();

    let api_key = secrets
        .get("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY not set in secrets");
    
    std::env::set_var("OPENROUTER_API_KEY", api_key);
    
    let cors = CorsOptions::default()
        .to_cors()
        .expect("Failed to create CORS fairing");

    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![paraphrase_handler])
        .attach(cors);

    Ok(rocket.into())
}

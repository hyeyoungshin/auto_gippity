use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call Large Language Model (i.g. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok(); // enables getting info from env vars

    // Extract APT Key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in .env");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in .env");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/vi/chat/completions";

    // Create headers
    let mut headers: HeaderMap = HeaderMap::new(); // key value pairs

    // Creat api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(), // json web tokens need to be sent with a bearer _space_ an api key
    );

    // Create Open AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create client
    let client = Client::builder().default_headers(headers).build().unwrap();

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages: messages,
        temperature: 0.1, // answer not very random and exploratory; consistent
    };

    // Troubleshooting, in case api request doesn't work -> comment out when working
    let res_raw = client
        .post(url) // send post request to our url
        .json(&chat_completion) // in reference to our chat completion
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
}

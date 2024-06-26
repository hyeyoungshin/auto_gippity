use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

// Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    // Advantages of `Box<dyn std::error::Error>`
    // 1. trait object for std::error::Error i.e. can hold any type that implements std::error:Error
    // so different errors can be returned
    // 2. simplify code
    // we can use ? instead of unwrap
    // 3. compatitlbe
    // third party libraries also use it
    // 4. dynamic dispatch with dyn
    // which method to run is decided at runtime
    // 5. about `+ Send``
    // A trait, ownership of type implementing `Send` can be transferred safely between threads
    // Important since we will call this twice if it fails once

    dotenv().ok(); // enables getting info from env vars

    // Extract APT Key information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in .env");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in .env");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: HeaderMap = HeaderMap::new(); // key value pairs

    // Creat api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)) // json web tokens need to be sent with a bearer _space_ an api key
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create Open AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1, // answer not very random and exploratory; more consistent
    };

    // Troubleshooting, in case api request doesn't work -> comment out when working
    // let res_raw = client
    //     .post(url) // send post request to our url
    //     .json(&chat_completion) // in reference to our chat completion
    //     .send()
    //     .await
    //     .unwrap();
    // dbg!(res_raw.text().await.unwrap());

    // Extract API Response (json string)
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send Response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*; // import everything from above

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response.".to_string(),
        };

        let messages = vec![message];

        let res: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(messages).await;
        // if let Ok(res_str) = res {
        //     dbg!(res_str);
        //     assert!(true)
        // } else {
        //     assert!(false)
        // }

        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}

use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub struct Discord {
    webhook_url: String,
}

impl Discord {
    pub fn new(webhook_url: &str) -> Self {
        Discord {
            webhook_url: webhook_url.to_string(),
        }
    }

    pub async fn send_channel_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        // Create an HTTP client
        let client = Client::new();

        // Create the payload as JSON
        let payload = json!({ "content": message });

        // Send a POST request to the webhook URL
        let response = client.post(&self.webhook_url).json(&payload).send().await?;

        // Check if the request was successful
        if response.status().is_success() {
            println!("Message sent successfully!");
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await?;
            Err(format!(
                "Failed to send message. Status: {}, Response: {}",
                status, text
            )
            .into())
        }
    }
}

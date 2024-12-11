use rand::Rng;
use tokio::time::{sleep, Duration};

use crate::{core::agent::Agent, providers::twitter::Twitter};

pub struct Runtime {
    openai_api_key: String,
    twitter: Twitter,
    agents: Vec<Agent>,
}

impl Runtime {
    pub fn new(
        openai_api_key: &str,
        twitter_consumer_key: &str,
        twitter_consumer_secret: &str,
        twitter_access_token: &str,
        twitter_access_token_secret: &str,
    ) -> Self {
        let twitter = Twitter::new(
            twitter_consumer_key,
            twitter_consumer_secret,
            twitter_access_token,
            twitter_access_token_secret,
        );

        let agents = Vec::new();

        Runtime {
            openai_api_key: openai_api_key.to_string(),
            agents,
            twitter,
        }
    }

    pub fn add_agent(&mut self, prompt: &str) {
        let agent = Agent::new(&self.openai_api_key, prompt);
        self.agents.push(agent);
    }

    pub async fn run(&self) -> Result<(), anyhow::Error> {
        let mut rng = rand::thread_rng();
        let selected_agent = &self.agents[rng.gen_range(0..self.agents.len())];

        let response = selected_agent.prompt("tweet").await?;
        println!("AI Response: {}", response);
        self.twitter.tweet(response).await?;
        Ok(())
    }

    pub async fn run_periodically(&self) -> Result<(), anyhow::Error> {
        let _ = self.run().await;

        loop {
            let random_delay = rand::thread_rng().gen_range(60..=300);
            println!(
                "Waiting for {} seconds before running again...",
                random_delay
            );

            sleep(Duration::from_secs(random_delay)).await;

            if let Err(e) = self.run().await {
                eprintln!("Error running process: {}", e);
            }
        }
    }
}

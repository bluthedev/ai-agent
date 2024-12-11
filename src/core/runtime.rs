use rand::Rng;
use tokio::time::{sleep, Duration};

use crate::{core::agent::Agent, memory::MemoryStore, providers::twitter::Twitter};

pub struct Runtime {
    openai_api_key: String,
    twitter: Twitter,
    agents: Vec<Agent>,
    memory: Vec<String>,
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
        let memory: Vec<String> = MemoryStore::load_memory().unwrap_or_else(|_| Vec::new());

        Runtime {
            memory,
            openai_api_key: openai_api_key.to_string(),
            agents,
            twitter,
        }
    }

    pub fn add_agent(&mut self, prompt: &str) {
        let agent = Agent::new(&self.openai_api_key, prompt);
        self.agents.push(agent);
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        if self.agents.is_empty() {
            return Err(anyhow::anyhow!("No agents available")).map_err(Into::into);
        }

        let mut rng = rand::thread_rng();
        let selected_agent = &self.agents[rng.gen_range(0..self.agents.len())];
        let response = selected_agent.prompt("tweet").await?;

        match MemoryStore::add_to_memory(&mut self.memory, &response) {
            Ok(_) => println!("Response saved to memory."),
            Err(e) => eprintln!("Failed to save response to memory: {}", e),
        }

        println!("AI Response: {}", response);
        self.twitter.tweet(response).await?;
        Ok(())
    }

    pub async fn run_periodically(&mut self) -> Result<(), anyhow::Error> {
        loop {
            sleep(Duration::from_secs(3600)).await;

            if let Err(e) = self.run().await {
                eprintln!("Error running process: {}", e);
            }
        }
    }
}

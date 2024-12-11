use rig::agent::Agent as RigAgent;
use rig::providers::openai::CompletionModel;
use rig::{completion::Prompt, providers};

pub struct Agent {
    agent: RigAgent<CompletionModel>,
}

impl Agent {
    pub fn new(openai_api_key: &str, prompt: &str) -> Self {
        let openai_client = providers::openai::Client::new(openai_api_key);
        let agent: rig::agent::Agent<providers::openai::CompletionModel> = openai_client
            .agent("chatgpt-4o-latest")
            .preamble(prompt)
            .temperature(1.0)
            .build();

        Agent { agent }
    }

    pub async fn prompt(&self, input: &str) -> Result<String, anyhow::Error> {
        let response = self.agent.prompt(input).await?;
        Ok(response)
    }
}

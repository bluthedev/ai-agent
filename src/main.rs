mod characteristics;
mod core;
mod memory;
mod providers;
use core::{instruction_builder::InstructionBuilder, runtime::Runtime};
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let twitter_consumer_key = env::var("TWITTER_CONSUMER_KEY").ok();
    let twitter_consumer_secret = env::var("TWITTER_CONSUMER_SECRET").ok();
    let twitter_access_token = env::var("TWITTER_ACCESS_TOKEN").ok();
    let twitter_access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").ok();
    let twitter_username = env::var("TWITTER_USERNAME").ok();
    let twitter_password = env::var("TWITTER_PASSWORD").ok();

    let mut runtime = Runtime::new(
        &env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        &env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL not set"),
        twitter_consumer_key.as_deref(),
        twitter_consumer_secret.as_deref(),
        twitter_access_token.as_deref(),
        twitter_access_token_secret.as_deref(),
        twitter_username.as_deref(),
        twitter_password.as_deref(),
    );

    let mut instruction_builder = InstructionBuilder::new();
    instruction_builder.build_instructions("degenspartan");

    runtime.add_agent(instruction_builder.get_instructions());
    runtime.run().await?;
    runtime.run_periodically().await?;

    Ok(())
}

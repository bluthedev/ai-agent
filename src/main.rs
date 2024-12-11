mod core;
mod memory;
mod providers;
use core::runtime::Runtime;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let mut runtime = Runtime::new(
        &env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        &env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY not set"),
        &env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET not set"),
        &env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN not set"),
        &env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET not set"),
    );

    runtime.run_periodically().await?;

    Ok(())
}

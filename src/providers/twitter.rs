use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

pub struct Twitter {
    auth: Oauth1aToken,
}
impl Twitter {
    pub fn new(
        twitter_consumer_key: &str,
        twitter_consumer_secret: &str,
        twitter_access_token: &str,
        twitter_access_token_secret: &str,
    ) -> Self {
        let auth = Oauth1aToken::new(
            twitter_consumer_key.to_string(),
            twitter_consumer_secret.to_string(),
            twitter_access_token.to_string(),
            twitter_access_token_secret.to_string(),
        );

        Twitter { auth }
    }

    pub async fn tweet(&self, text: String) -> Result<(), anyhow::Error> {
        let tweet = TwitterApi::new(self.auth.clone())
            .post_tweet()
            .text(text)
            .send()
            .await?
            .into_data()
            .expect("this tweet should exist");
        println!("Tweet posted successfully with ID: {}", tweet.id);

        Ok(())
    }
}

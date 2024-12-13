use std::process::Command;

pub struct Ai16zTwitter {
    username: String,
    password: String,
}
impl Ai16zTwitter {
    pub fn new(username: &str, password: &str) -> Self {
        Ai16zTwitter {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub async fn tweet(&self, text: String) -> Result<(), anyhow::Error> {
        let output = Command::new("node")
            .arg("twitter.js")
            .arg(&self.username)
            .arg(&self.password)
            .arg(&text)
            .output()
            .expect("Failed to execute process");

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to execute the command").into());
        }

        Ok(())
    }
}

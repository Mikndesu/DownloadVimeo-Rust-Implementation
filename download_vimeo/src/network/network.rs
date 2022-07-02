pub struct NetworkStuff {
    url: String
}

impl NetworkStuff {
    pub fn origin(url: &str) -> NetworkStuff {
        NetworkStuff {url:url.to_string()}
    }
    pub async fn perform(self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get(self.url).await?;
        dbg!(&resp.text().await?);
        Ok(())
    }
}


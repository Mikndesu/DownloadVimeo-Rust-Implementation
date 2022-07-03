use crate::model::model::Model;

pub struct FetchMetadata {
    url: String
}

impl FetchMetadata {
    pub fn origin(url: &str) -> FetchMetadata {
        FetchMetadata {url:url.to_string()}
    }
    pub async fn perform(self) -> Result<Model, Box<dyn std::error::Error>> {
        let resp = reqwest::get(self.url).await?;
        let json_text = &resp.text().await?;
        let deserialized:Model = serde_json::from_str(&json_text).unwrap();
        Ok(deserialized)
    }
}


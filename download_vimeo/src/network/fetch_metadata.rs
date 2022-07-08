use crate::model::model::Model;
use url::Url;

pub struct FetchMetadata {
    url: Url
}

impl FetchMetadata {
    pub fn origin(url:Url) -> FetchMetadata {
        FetchMetadata {url}
    }
    pub async fn perform(self) -> Result<Model, Box<dyn std::error::Error>> {
        let resp = reqwest::get(self.url).await?;
        let json_text = &resp.text().await?;
        let deserialized:Model = serde_json::from_str(&json_text).unwrap();
        Ok(deserialized)
    }
}


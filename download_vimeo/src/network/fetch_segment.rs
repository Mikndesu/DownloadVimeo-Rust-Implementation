use std::io::Cursor;

pub struct FetchSegment {
    url: String,
    file_path: String
}

impl FetchSegment {
    pub fn origin(url: &str, file_path: &str) -> FetchSegment {
        FetchSegment {url:url.to_string(), file_path:file_path.to_string()}
    }
    pub async fn perform(self) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(self.url).await?;
        let mut file = std::fs::File::create(self.file_path)?;
        let mut content =  Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }
}


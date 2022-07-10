use std::{io::Cursor, path::Path};
use url::Url;

pub struct FetchSegment<'a> {
    url: Url,
    file_path: &'a Path
}

impl FetchSegment<'_> {
    pub fn origin<'a>(url: &'a str, file_path: &'a str) -> FetchSegment<'a> {
        FetchSegment {url:Url::parse(url).ok().unwrap(), file_path:Path::new(file_path)}
    }
    pub async fn perform(self) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(self.url).await?;
        let mut file = std::fs::File::create(self.file_path)?;
        let mut content =  Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }
}


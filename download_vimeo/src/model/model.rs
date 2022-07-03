use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[serde(rename = "clip_id")]
    pub clip_id: String,
    #[serde(rename = "base_url")]
    pub base_url: String,
    pub video: Vec<Video>,
    pub audio: Vec<Audio>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    #[serde(rename = "base_url")]
    pub base_url: String,
    pub format: String,
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    pub codecs: String,
    pub bitrate: i64,
    #[serde(rename = "avg_bitrate")]
    pub avg_bitrate: i64,
    pub duration: f64,
    pub framerate: f64,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "max_segment_duration")]
    pub max_segment_duration: i64,
    #[serde(rename = "init_segment")]
    pub init_segment: String,
    pub segments: Vec<Segment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub url: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub id: String,
    #[serde(rename = "base_url")]
    pub base_url: String,
    pub format: String,
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    pub codecs: String,
    pub bitrate: i64,
    #[serde(rename = "avg_bitrate")]
    pub avg_bitrate: i64,
    pub duration: f64,
    pub channels: i64,
    #[serde(rename = "sample_rate")]
    pub sample_rate: i64,
    #[serde(rename = "max_segment_duration")]
    pub max_segment_duration: i64,
    #[serde(rename = "init_segment")]
    pub init_segment: String,
    pub segments: Vec<Segment>,
}
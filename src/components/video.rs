#![allow(unused_imports)]
#![allow(dead_code)]
use serde::Deserialize;

/// # Video
/// Video model
#[derive(Deserialize, Debug)]
pub struct Video {
    #[serde(rename = "__media")]
    pub media: Option<VideoMedia>,
    pub url: Option<String>,
    #[serde(default)]
    pub controls: bool,
    #[serde(rename = "loop")]
    pub is_loop: bool,
    pub playing: bool,
    pub poster: Option<VideoPoster>,
    #[serde(rename = "startTime")]
    pub start_time: String
}

#[derive(Deserialize, Debug)]
pub struct VideoPoster {
    pub source: Option<PosterSource>
}

#[derive(Deserialize, Debug)]
pub struct VideoMedia {
    pub src: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct PosterSource {
    pub url: String
}

/// Helper to identify whether video is set and should render
pub fn should_render_video(video: &Option<Video>) -> bool {
    video.as_ref().is_some_and(|v| v.media.as_ref().is_some_and(|m| m.src.as_ref().is_some_and(|s| !s.is_empty())))
}

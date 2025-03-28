#![allow(unused_imports)]
#![allow(dead_code)]
use serde::Deserialize;

/// # Image
/// Image model
#[derive(Deserialize, Debug)]
pub struct Image {
    pub mobile: Option<ImageData>,
    pub tablet: Option<ImageData>,
    pub desktop: Option<ImageData>
}

#[derive(Deserialize, Debug)]
pub struct ImageData {
    pub source: Option<ImageSource>
}

#[derive(Deserialize, Debug)]
pub struct ImageSource {
    pub url: Option<String>
}
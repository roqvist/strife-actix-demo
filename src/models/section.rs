#![allow(unused_imports)]
#![allow(dead_code)]
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

use crate::components;

/// # Section
/// Represents a generic Strife `section`, for this specific demo content.
#[derive(Debug)]
pub struct Section {
    pub strife: StrifeSection
}

/// # StrifeSection
/// Represents a specific `section` type, identified/tagged on the `template` text.
#[derive(Deserialize, Debug)]
#[serde(tag = "template")]
pub enum StrifeSection {
    #[serde(rename = "hero")]
    Hero(components::Hero),
    #[serde(rename = "content")]
    Content(components::Content),
    #[serde(rename = "text")]
    Text(components::Text),
    #[serde(rename = "expandable")]
    Expandable(components::Expandable),
    #[serde(rename = "bento")]
    Bento(components::Bento),
    #[serde(other)]
    Unknown
}

impl<'de> Deserialize<'de> for Section {
    fn deserialize<D>(deserializer: D) -> Result<Section, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialisera hela objektet som ett serde_json::Value
        let mut value = Value::deserialize(deserializer)?;
        // Förvänta oss att vi får ett objekt (en map)
        let obj = value
            .as_object_mut()
            .ok_or_else(|| de::Error::custom("expected a JSON object"))?;
        
        // Ta ut "@strife"-objektet
        let strife_value = obj
            .remove("@strife")
            .ok_or_else(|| de::Error::missing_field("@strife"))?;
        
        // Förvänta oss att "@strife" är ett objekt
        let mut strife_obj = strife_value
            .as_object()
            .cloned()
            .ok_or_else(|| de::Error::custom("@strife must be an object"))?;
        
        // Merge:a de återstående fälten från obj in i strife_obj
        // Detta lägger t.ex. in "align": "left" i strife_obj
        for (key, value) in obj.iter() {
            strife_obj.insert(key.clone(), value.clone());
        }
        
        // Nu deserialiserar vi den merge:ade mappen till vårt enum StrifeSection
        let strife_section = StrifeSection::deserialize(Value::Object(strife_obj))
            .map_err(de::Error::custom)?;
        
        Ok(Section {
            strife: strife_section,
        })
    }
}
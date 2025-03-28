use actix_web::{
    dev::Payload, web::Data, Error, FromRequest, HttpRequest
};
use futures::Future;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::pin::Pin;

use crate::settings;

/// # StrifeContent
/// Extractor for Strife CMS content, through RavenDB
/// 
/// Returns generic type `T` (your view model)
pub struct StrifeContent<T>(pub T);

impl<T> FromRequest for StrifeContent<T>
where
    T: DeserializeOwned + 'static,
{
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let client = req
            .app_data::<Data<Client>>()
            .expect("Client AppData not found")
            .clone();
        let settings = req
            .app_data::<Data<settings::RavenDBSettings>>()
            .expect("RavenDB Settings AppData not found")
            .clone();

        let path = req.path().to_owned();
        
        let url = format!("{}/databases/{}/queries", settings.db_url, settings.db_name);
        
        let query_payload = json!({
            "Query": format!("FROM INDEX 'Content/ByUrl' WHERE url = '{}'", path)
        });
        
        let fut = async move {
            let response = client
                .post(&url)
                .json(&query_payload)
                .send()
                .await
                .map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("Request error: {}", e))
                })?;
            let json = response.json().await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Error reading response: {:?}", e))
            })?;
            Ok(StrifeContent(json))
        };

        Box::pin(fut)
    }
}

/// # StrifeContentJson
/// Extractor for Strife CMS content, through RavenDB
/// 
/// Returns raw text response
pub struct StrifeContentJson(pub Value);

impl FromRequest for StrifeContentJson {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let client = req
            .app_data::<Data<Client>>()
            .expect("Client AppData not found")
            .clone();
        let settings = req
            .app_data::<Data<settings::RavenDBSettings>>()
            .expect("RavenDB Settings AppData not found")
            .clone();
        let path = req.path().to_owned().replace("/rawjson", "");
        let url = format!("{}/databases/{}/queries", settings.db_url, settings.db_name);
        let query_payload = json!({
            "Query": format!("FROM INDEX 'Content/ByUrl' WHERE url = '{}'", path)
        });

        let fut = async move {
            let response = client
                .post(&url)
                .json(&query_payload)
                .send()
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Request error: {}", e)))?;
            
            let json = response.json().await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Error reading response: {}", e))
            })?;
            Ok(StrifeContentJson(json))
        };

        Box::pin(fut)
    }
}
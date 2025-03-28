#![allow(unused_imports)]
#![allow(dead_code)]
mod views;
mod extractors;
mod settings;
mod models;
mod components;

use actix_web::{
    get, web::Data, App, HttpResponse, HttpServer, Responder, Result as ActixResult
};

use actix_files::Files as ActixFiles;

use extractors::{StrifeContent, StrifeContentJson};
use maud::Markup;
use reqwest::Client;
use serde_json::to_string_pretty;

/// # sections
/// This service is used when compiled without any features.
/// The page is loaded with Strife data from the middleware only;
/// no js, no realtime preview, no hooks.
#[cfg(not(feature = "realtime"))]
#[get("/{tail:.*}")]
async fn sections(data: StrifeContent<views::Sections>) -> ActixResult<Markup> {
    Ok(data.0.render(None))
}

/// # sections - realtime
/// This service is used only when compiled with `--features realtime`
/// It loads the page with content from the middleware (for initial store/rendering),
/// and also uses Alpine.js to subscribe to changes and subscribe
/// to Strife realtime preview.
#[cfg(feature = "realtime")]
#[get("/{tail:.*}")]
async fn sections(json: StrifeContentJson) -> ActixResult<Markup> {
    Ok(views::Sections{}.render(Some(to_string_pretty(&json.0)?)))
}

/// # `/rawjson/`
/// Displays a raw json response with the data from Strife/Raven.
/// For debugging etc.
/// 
/// Example:
/// ```
/// http://localhost:8080/rawjson/instant-preview-instant-groove
/// ```
#[get("/rawjson/{tail:.*}")]
async fn rawjson(json: StrifeContentJson) -> ActixResult<impl Responder> {
    let pretty = to_string_pretty(&json.0)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(pretty))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read settings from .env
    let (db_name, db_url, identity) = settings::from_env()?;
    
    // The client is created once, stored in app state and reused for all requests.
    let client = Client::builder().identity(identity).build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unable to build client from Identity: {}", e)))?;

    // Raven settings is created once, stored in app state and reused for all requests.
    let raven_settings = settings::RavenDBSettings {
        db_name,
        db_url
    };
    
    // Create Actix app and run
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .app_data(Data::new(raven_settings.clone()))
            .service(ActixFiles::new("/styles", "./public/styles").prefer_utf8(true))
            .service(ActixFiles::new("/js", "./public/js").prefer_utf8(true))
            .service(rawjson)
            .service(sections)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
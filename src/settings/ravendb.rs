/// # RavenDBSettings
/// Actix Appdata required for querying RavenDB
#[derive(Clone)]
pub struct RavenDBSettings {
    pub db_name: String,
    pub db_url: String
}
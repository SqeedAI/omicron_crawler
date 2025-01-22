use crate::linkedin::api::json::Profile;
//Move this whole crate under linkedin
#[derive(serde::Deserialize)]
pub struct ProfileIds {
    pub profile_ids: Vec<String>,
    pub request_metadata: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CrawledProfiles {
    pub profiles: Vec<Profile>,
    pub request_metadata: Option<String>,
}

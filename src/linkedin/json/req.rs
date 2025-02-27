#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Signup {
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub password: String,
    pub submission_id: Option<String>,
    pub resolved_challenge_url: Option<String>,
}

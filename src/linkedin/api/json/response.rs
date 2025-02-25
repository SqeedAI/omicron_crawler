#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupChallenge {
    pub submission_id: String,
    pub challenge_url: String,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupSuccess {
    pub submission_id: String,
    pub redirect_url: String,
    pub member_urn: String,
}
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum Signup {
    Challenge(SignupChallenge),
    Success(SignupSuccess),
}

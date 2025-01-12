#[derive(serde::Deserialize)]
pub struct FetchCookiesResponse {
    pub status: String,
}
#[derive(serde::Deserialize)]
pub struct AuthenticateResponse {
    pub login_result: String,
    pub challenge_url: String,
}

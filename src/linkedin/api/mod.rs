static LINKEDIN_URL: &str = "https://www.linkedin.com";
static API_URL: &str = concat!(LINKEDIN_URL, "/voyager/api");

pub struct LinkedinSession {
    pub session_id: String,
}

impl LinkedinSession {
    pub fn new(username: String, password: String) -> Self {
        Self { session_id }
    }
}

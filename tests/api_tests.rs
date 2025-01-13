use omicron_crawler::linkedin::api::LinkedinSession;

#[tokio::test(flavor = "multi_thread")]
pub async fn api_auth_test() {
    let mut linkedin_session = LinkedinSession::new();
    if let Err(e) = linkedin_session.authenticate("erik9631@gmail.com", "soRMoaN7C2bX2mKbV9V4").await {
        assert!(false, "Failed to authenticate {}", e);
    }
}

pub async fn api_profile_test() {
    let mut linkedin_session = LinkedinSession::new();
    if let Err(e) = linkedin_session.authenticate("erik9631@gmail.com", "soRMoaN7C2bX2mKbV9V4").await {
        assert!(false, "Failed to authenticate {}", e);
    }
    match linkedin_session.profile("matus-chochlik-154a7827".to_string()).await {
        Ok(profile) => println!("{}", profile.profile.first_name),
        Err(e) => println!("{}", e),
    }
}

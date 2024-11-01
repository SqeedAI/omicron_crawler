use omicron_crawler::linkedin::crawler::Crawler;

#[tokio::test]
async fn test_parse_1() {
    let selenium = Crawler::new("8888".to_string()).await;
    let profile_url =
        "https://www.linkedin.com/sales/lead/ACwAAAWs1dABZXg7RDqKugFxlSeo7gasFL1FPHQ,NAME_SEARCH,cypw?_ntb=xTZht7tmSNWO81Egbmk6Xg%3D%3D";
    let results = selenium.parse_profile(profile_url).await;
    assert_eq!(results.name, "Matus Chochlik");
    assert_eq!(results.url, "https://www.linkedin.com/in/matus-chochlik-154a7827");
    selenium.cleanup().await;
}

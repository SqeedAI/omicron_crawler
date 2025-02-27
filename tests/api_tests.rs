use log::LevelFilter;
use omicron_crawler::azure::{AzureClient, Label};
use omicron_crawler::cookies::new_cookie_jar;
use omicron_crawler::env::{get_env, load_env};
use omicron_crawler::linkedin::json::{GeoUrnMap, SearchItem, SearchParams, SearchResult};
use omicron_crawler::linkedin::Client;
use omicron_crawler::logger::Logger;

#[tokio::test(flavor = "multi_thread")]
pub async fn api_auth_test() {
    Logger::init(LevelFilter::Trace);
    let cookie = new_cookie_jar();
    let mut linkedin_session = Client::new(cookie);
    let username = "test_user";
    let password = "test_password";
    if let Err(e) = linkedin_session.authenticate(username, password).await {
        assert!(false, "Failed to authenticate {}", e);
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn api_skills_test() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();
    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let skills = match linkedin_session.skills("matus-chochlik-154a7827").await {
        Ok(skills) => skills,
        Err(e) => {
            assert!(false, "Failed to get skills {}", e);
            return;
        }
    };
    assert_eq!(skills.elements.len(), 42);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn api_profile_test_1() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();
    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let mut profile = match linkedin_session.profile("matus-chochlik-154a7827").await {
        Ok(profile) => profile,
        Err(e) => {
            assert!(false, "Failed to get profile {}", e);
            return;
        }
    };
    profile.skill_view = if let Ok(skills) = linkedin_session.skills("matus-chochlik-154a7827").await {
        skills
    } else {
        assert!(false, "Failed to get skills");
        return;
    };

    assert_eq!(profile.profile.first_name, "Matus");
    assert_eq!(profile.profile.last_name, "Chochlik");
    assert_eq!(profile.profile.headline, "SW engineer C++/Python/Shell/OpenGL/SQL ISO WG21 member");
    assert_eq!(profile.profile.geo_country_name, "Slovakia");
    assert_eq!(profile.profile.location_name, Some("Slovak Republic".to_string()));
    assert_eq!(profile.profile.industry_name, Some("Higher Education".to_string()));
    assert_eq!(profile.profile.summary.as_ref().unwrap(), "Solution architect / Tech lead / SW engineer, mostly C++, Python. Member of C++ standards committee. Having experience with modern OpenGL programming and with relational database systems.\n\nSpecialties: software development, C++, meta-programming, OpenGL 4 programming");
    println!("{}", profile.profile.picture_url.unwrap());
    assert_eq!(profile.education_view.elements.len(), 1);
    assert_eq!(
        profile.education_view.elements[0].school_name,
        Some("University of Zilina".to_string())
    );
    assert_eq!(
        profile.education_view.elements[0].field_of_study,
        Some("Applied Computer Science".to_string())
    );
    assert_eq!(profile.education_view.elements[0].degree_name, Some("PhD".to_string()));
    assert_eq!(
        profile.education_view.elements[0]
            .time_period
            .as_ref()
            .unwrap()
            .start_date
            .as_ref()
            .unwrap()
            .year,
        2005
    );
    assert_eq!(
        profile.education_view.elements[0]
            .time_period
            .as_ref()
            .unwrap()
            .end_date
            .as_ref()
            .unwrap()
            .year,
        2008
    );
    assert_eq!(profile.project_view.elements.len(), 3);
    assert_eq!(profile.project_view.elements[0].title, "JGL - A Java wrapper for OpenGL 3");
    assert_eq!(profile.project_view.elements[0].description, Some("This project is inspired on and aims to provide similar functionality as the OGLPlus C++ OpenGL framework. jgl makes use of the JOGL Java library to access OpenGL in a cross platform fashion.\n\nWith jgl, you get a set of wrapper classes which abstract most of the housekeeping tasks one inevitably faces when dealing with OpenGL. This framework is primarily focused on OpenGL 3 since it is the currently accepted programming standard for OpenGL and is on par with DirectX feature wise (if not more ;)).".to_string()));
    assert_eq!(
        profile.project_view.elements[0]
            .time_period
            .as_ref()
            .unwrap()
            .start_date
            .as_ref()
            .unwrap()
            .year,
        2011
    );
    assert_eq!(
        profile.project_view.elements[0]
            .time_period
            .as_ref()
            .unwrap()
            .start_date
            .as_ref()
            .unwrap()
            .month
            .unwrap(),
        7
    );
    assert_eq!(profile.project_view.elements[1].title, "OGLplus");
    assert_eq!(profile.project_view.elements[2].title, "Mirror reflection utilities");
    assert_eq!(profile.position_view.elements.len(), 5);
    assert_eq!(profile.position_view.elements[0].company_name.as_ref().unwrap(), "Asseco CEIT");
    assert_eq!(profile.position_view.elements[0].title, "Team Lead / SW engineer / Architect");
    assert_eq!(
        profile.position_view.elements[0]
            .time_period
            .as_ref()
            .unwrap()
            .start_date
            .as_ref()
            .unwrap()
            .year,
        2023
    );
    assert_eq!(
        profile.position_view.elements[0]
            .time_period
            .as_ref()
            .unwrap()
            .start_date
            .as_ref()
            .unwrap()
            .month
            .unwrap(),
        12
    );
    assert_eq!(profile.position_view.elements[1].title, "Opensource Software Developer");
    assert_eq!(
        profile.position_view.elements[1].description.as_ref().unwrap(),
        "Management and development of several opensource software projects, including OGLplus and Mirror reflection utilities."
    );
    assert_eq!(profile.position_view.elements[2].company_name.as_ref().unwrap(), "Illumio");
    assert_eq!(
        profile.position_view.elements[3].company_name.as_ref().unwrap(),
        "GlobalLogic Slovakia"
    );
    assert_eq!(
        profile.position_view.elements[3].location_name.as_ref().unwrap(),
        "Žilina, Slovakia"
    );

    assert_eq!(profile.language_view.elements.len(), 2);
    assert_eq!(profile.language_view.elements[0].name, "English");
    assert_eq!(
        profile.language_view.elements[0].proficiency.as_ref().unwrap(),
        "PROFESSIONAL_WORKING"
    );
    assert_eq!(profile.language_view.elements[1].name, "German");
    assert_eq!(profile.language_view.elements[1].proficiency.as_ref().unwrap(), "LIMITED_WORKING");
    assert_eq!(profile.skill_view.elements.len(), 42);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn api_profile_test_2() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();
    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let profile = match linkedin_session.profile("peter-hamran-151a6317a").await {
        Ok(profile) => profile,
        Err(e) => {
            assert!(false, "Failed to get profile {}", e);
            return;
        }
    };
    assert_eq!(profile.profile.first_name, "Peter");
    assert_eq!(profile.profile.last_name, "Hamran");
    assert_eq!(profile.profile.summary, None);
    assert_eq!(profile.profile.industry_name, Some("Computer Software".to_string()));
    assert_eq!(profile.profile.headline, "Cofounder of Sqeed s.r.o.".to_string());
    assert_eq!(profile.position_view.elements.len(), 1);
    assert_eq!(profile.position_view.elements[0].title, "System Engineer".to_string());
    assert_eq!(profile.position_view.elements[0].company_name.as_ref().unwrap(), "ESET");
}

#[tokio::test(flavor = "multi_thread")]
pub async fn api_profile_test_3() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8001", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let profile = match linkedin_session.profile("patrik-bujňák-dev").await {
        Ok(profile) => profile,
        Err(e) => {
            assert!(false, "Failed to get profile {}", e);
            return;
        }
    };
    assert_eq!(profile.profile.first_name, "Patrik");
    assert_eq!(profile.profile.last_name, "Bujňák");
}

#[tokio::test(flavor = "multi_thread")]
pub async fn api_profile_test_4() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let profile = match linkedin_session.profile("kamil-psenak").await {
        Ok(profile) => profile,
        Err(e) => {
            assert!(false, "Failed to get profile {}", e);
            return;
        }
    };
    assert_eq!(profile.profile.first_name, "Kamil");
    assert_eq!(profile.profile.last_name, "Pšenák");
}

#[tokio::test(flavor = "multi_thread")]
pub async fn api_profile_test_5() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8001", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }
    let profile = match linkedin_session.profile("tomas-danis").await {
        Ok(profile) => profile,
        Err(e) => {
            assert!(false, "Failed to get profile {}", e);
            return;
        }
    };
    assert_eq!(profile.profile.first_name, "Tomáš");
    assert_eq!(profile.profile.last_name, "Daniš");
    assert_eq!(profile.course_view.elements.len(), 6);
    assert_eq!(profile.course_view.elements[0].name, "Bare minimum course");
    assert_eq!(profile.course_view.elements[1].name, "Full course");
    assert_eq!(profile.course_view.elements[1].number, Some("FC101".to_string()));

    assert_eq!(profile.volunteer_experience_view.elements.len(), 3);
    assert_eq!(profile.volunteer_experience_view.elements[0].role, "Testing role");
    assert_eq!(profile.volunteer_experience_view.elements[0].company_name, "sqeed");
    assert_eq!(
        profile.volunteer_experience_view.elements[0].description,
        Some("Description present".to_string())
    );
    assert_eq!(profile.volunteer_experience_view.elements[2].role, "Role");
    assert_eq!(profile.volunteer_experience_view.elements[2].company_name, "Bare minimum");

    assert_eq!(profile.certification_view.elements.len(), 4);

    assert_eq!(profile.certification_view.elements[0].authority, Some("Cisco".to_string()));
    assert_eq!(profile.certification_view.elements[0].name, "CCNA Routing and Switching");
    assert_eq!(profile.certification_view.elements[2].authority, Some("sqeed".to_string()));
    assert_eq!(profile.certification_view.elements[2].name, "Empty certificate");

    assert_eq!(profile.test_score_view.elements.len(), 2);
    assert_eq!(profile.test_score_view.elements[0].name, "Full test score");
    assert_eq!(profile.test_score_view.elements[0].description, Some("Description".to_string()));
    assert_eq!(profile.test_score_view.elements[0].score, "2");

    assert_eq!(profile.test_score_view.elements[1].name, "Bare minimum test score");
    assert_eq!(profile.test_score_view.elements[1].description, None);
    assert_eq!(profile.test_score_view.elements[1].score, "100");
}

#[tokio::test(flavor = "multi_thread")]
async fn api_profile_test_6() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }
    let profile = match linkedin_session.profile("ACoAACFknJIB6xq2mpBDnc9Ik9IQz3asuMw9Jww").await {
        Ok(profile) => profile,
        Err(e) => {
            assert!(false, "Failed to get profile {}", e);
            return;
        }
    };
    assert!(profile.profile.picture_url.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn api_profile_test_7() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let profile = match linkedin_session.profile("ACoAABAbOBkBAK5E_stVP4ex6jMlV2qABdj2n3E").await {
        Ok(profile) => profile,
        Err(e) => {
            assert!(false, "Failed to get profile {}", e);
            return;
        }
    };
}

#[tokio::test]
async fn test_search_people_1() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let params = SearchParams {
        page: 0,
        keywords: Some("AWS".to_string()),
        keyword_first_name: Some("Alexandra".to_string()),
        keyword_last_name: None,
        keyword_title: None,
        keyword_company: None,
        keyword_school: None,
        countries: Some(vec![GeoUrnMap::Slovakia, GeoUrnMap::Czechia]),
        profile_language: None,
        network_depth: None,
        end: 100,
        request_metadata: None,
    };
    let search_result = match linkedin_session.search_people(params).await {
        Ok(result) => result,
        Err(e) => panic!("Failed to search people {}", e),
    };

    for i in search_result.elements.iter() {
        println!("{}\nhttps://www.linkedin.com/in/{}\n\n", i.url, i.profile_urn)
    }

    assert!(search_result.total > 0);
    assert!(search_result.elements.len() > 0);
    assert_eq!(search_result.total_lookup, 37);
}

#[tokio::test]
async fn test_search_people_2() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let params = SearchParams {
        page: 0,
        keywords: Some("AWS".to_string()),
        keyword_first_name: None,
        keyword_last_name: None,
        keyword_title: None,
        keyword_company: None,
        keyword_school: None,
        countries: Some(vec![GeoUrnMap::Slovakia, GeoUrnMap::Czechia]),
        profile_language: None,
        network_depth: None,
        end: 2,
        request_metadata: None,
    };
    let search_result = match linkedin_session.search_people(params).await {
        Ok(result) => result,
        Err(e) => panic!("Failed to search people {}", e),
    };

    for i in search_result.elements.iter() {
        println!("{}\nhttps://www.linkedin.com/in/{}\n\n", i.url, i.profile_urn)
    }

    assert!(search_result.total > 0);
    assert_eq!(search_result.elements.len(), 1);
}

#[tokio::test]
async fn test_search_people_3() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let params = SearchParams {
        page: 0,
        keywords: Some("Java,C++".to_string()),
        keyword_first_name: Some("Dušan".to_string()),
        keyword_last_name: None,
        keyword_title: None,
        keyword_company: None,
        keyword_school: None,
        countries: Some(vec![GeoUrnMap::Slovakia, GeoUrnMap::Czechia]),
        profile_language: None,
        network_depth: None,
        end: 1,
        request_metadata: None,
    };
    let search_result = match linkedin_session.search_people(params).await {
        Ok(result) => result,
        Err(e) => panic!("Failed to search people {}", e),
    };

    for i in search_result.elements.iter() {
        println!("{}\nhttps://www.linkedin.com/in/{}\n\n", i.url, i.profile_urn)
    }

    assert!(search_result.total > 0);
    assert!(search_result.elements.len() > 0);
}

#[tokio::test]
async fn test_search_people_max() {
    Logger::init(LevelFilter::Trace);
    load_env();
    let mut linkedin_session = Client::new_proxy("ddc.oxylabs.io:8002", "sqeed_i0J4T", "fqXJbuiUEHaXyFd6DCQZ_+");
    let username = get_env().await.linkedin_username.as_str();
    let password = get_env().await.linkedin_password.as_str();

    if let Err(e) = linkedin_session.authenticate(username, password, false).await {
        assert!(false, "Failed to authenticate {}", e);
    }

    let params = SearchParams {
        page: 0,
        keywords: Some("AWS".to_string()),
        keyword_first_name: Some("Tomas".to_string()),
        keyword_last_name: None,
        keyword_title: None,
        keyword_company: None,
        keyword_school: None,
        countries: Some(vec![GeoUrnMap::Slovakia]),
        profile_language: None,
        network_depth: None,
        end: 100,
        request_metadata: None,
    };
    let search_result = match linkedin_session.search_people(params).await {
        Ok(result) => result,
        Err(e) => panic!("Failed to search people {}", e),
    };

    for i in search_result.elements.iter() {
        println!("{:?}", i.url);
    }

    assert!(search_result.total > 0);
    assert!(search_result.elements.len() > 0);
}

#[tokio::test]
async fn push_to_bus_search_complete_test() {
    load_env();
    let azure_client = AzureClient::new().await;
    let search_result = SearchResult {
        elements: vec![SearchItem {
            first_name: "Tomas".to_string(),
            last_name: "Stranak".to_string(),
            subtitle: Some("Software Engineer".to_string()),
            summary: None,
            profile_urn: "urn:li:fsd_profile:B0B1B01A".to_string(),
            url: "https://www.linkedin.com/in/tomas-stranak-b0b1b01a".to_string(),
        }],
        request_metadata: None,
        total_lookup: 0,
        total: 1,
    };
    match azure_client.push_to_bus(&search_result, Label::SearchComplete).await {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to push to bus {}", e),
    }
}

#[tokio::test]
async fn test_push_profile_to_queue() {
    load_env();
    let azure_client = AzureClient::new().await;
    let test_data = String::from("test_data");

    match azure_client.push_to_queue(&test_data).await {
        Ok(_) => {}
        Err(e) => assert!(false, "Failed to push to bus {}", e),
    }
}

use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::logger::Logger;

#[tokio::test(flavor = "multi_thread")]
async fn test_parse_1() {
    Logger::init(log::LevelFilter::Trace);
    let selenium = Crawler::new("8888".to_string()).await;
    let profile_url =
        "https://www.linkedin.com/sales/lead/ACwAAAWs1dABZXg7RDqKugFxlSeo7gasFL1FPHQ,NAME_SEARCH,cypw?_ntb=xTZht7tmSNWO81Egbmk6Xg%3D%3D";
    let results = fatal_unwrap_e!(selenium.parse_profile(profile_url).await, "{}");
    assert_eq!(results.name, "Matus Chochlik");
    assert_eq!(results.url, "https://www.linkedin.com/in/matus-chochlik-154a7827");
    assert_eq!(results.profile_picture_url, "https://media.licdn.com/dms/image/v2/C4D03AQGhNg5cATVIJQ/profile-displayphoto-shrink_100_100/profile-displayphoto-shrink_100_100/0/1517475003399?e=1735776000&v=beta&t=NO-k0V1cI_w4Rbwh3DHf31qpMIqAhE10YHtR_I5B0ow");
    assert_eq!(results.description, "SW engineer C++/Python/Shell/OpenGL/SQL ISO WG21 member");
    assert_eq!(results.location, "Slovakia");
    assert_eq!(results.about.is_some(), true);
    println!("{}", results.about.unwrap());
    match results.experience {
        Some(experience) => {
            for experience in experience.iter() {
                println!("{}", *experience);
            }
            assert_eq!(experience.len(), 8);
        }
        None => {
            assert!(false, "No experience found");
        }
    }
    match results.education {
        Some(education) => {
            assert_eq!(education.len(), 1);
            assert_eq!(education[0].title, "University of Zilina");
            assert_eq!(education[0].field, "Applied Computer Science");
            assert_eq!(education[0].degree, "PhD");
        }
        None => {
            assert!(false, "No education found");
        }
    }
    match results.skills {
        Some(skills) => {
            assert_eq!(skills.len(), 42);
        }
        None => {
            assert!(false, "No skills found");
        }
    }
    match results.languages {
        Some(languages) => {
            assert_eq!(languages.len(), 2);
            assert_eq!(languages[0].language, "English");
            assert_eq!(languages[0].fluency, "Professional working proficiency");
            assert_eq!(languages[1].language, "German");
            assert_eq!(languages[1].fluency, "Limited working proficiency");
        }
        None => {
            assert!(false, "No languages found");
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_parse_2() {
    Logger::init(log::LevelFilter::Trace);
    let selenium = Crawler::new("8888".to_string()).await;
    let profile_url =
        "https://www.linkedin.com/sales/lead/ACwAAAy0mZcBlmERvP-yDTL3gnlTLSELF6c7hrk,NAME_SEARCH,UDAQ?_ntb=aRijRPOnTBeuYBCnRY718Q%3D%3D";
    let results = fatal_unwrap_e!(selenium.parse_profile(profile_url).await, "{}");
    assert_eq!(results.name, "Patrik Bujňák");
    assert_eq!(results.url, "https://www.linkedin.com/in/patrik-buj%C5%88%C3%A1k-dev");
    assert_eq!(results.profile_picture_url, "https://media.licdn.com/dms/image/v2/C5603AQHICPthc6Rpbw/profile-displayphoto-shrink_100_100/profile-displayphoto-shrink_100_100/0/1562843874807?e=1735776000&v=beta&t=eOeTW1RZxx-8RuJiSo8t0_dBK3u0tbxg-ZRSnm5b3q4");
    assert_eq!(results.description, "Full-stack Developer at Vissim");
    assert_eq!(results.location, "Slovakia");
    assert_eq!(results.about.is_some(), false);
    match results.experience {
        Some(experience) => {
            for experience in experience.iter() {
                println!("{}", *experience);
            }
            assert_eq!(experience.len(), 6);
        }
        None => {
            assert!(false, "No experience found");
        }
    }
    match results.education {
        Some(education) => {
            assert_eq!(education.len(), 2);
            assert_eq!(education[0].title, "Technical University of Košice");
            assert_eq!(education[0].field, "Business Informatics");
            assert_eq!(education[0].degree, "Master's degree");

            assert_eq!(education[1].title, "Technical University of Košice");
            assert_eq!(education[1].field, "Business Informatics");
            assert_eq!(education[1].degree, "Bachelor's degree");
        }
        None => {
            assert!(false, "No education found");
        }
    }
    match results.skills {
        Some(skills) => {
            assert_eq!(skills.len(), 18);
        }
        None => {
            assert!(false, "No skills found");
        }
    }

    match results.languages {
        Some(languages) => {
            assert_eq!(languages.len(), 3);
            assert_eq!(languages[0].language, "English");
            assert_eq!(languages[0].fluency, "Professional working proficiency");
            assert_eq!(languages[1].language, "Spanish");
            assert_eq!(languages[1].fluency, "Elementary proficiency");
            assert_eq!(languages[2].language, "Slovak");
            assert_eq!(languages[2].fluency, "Native or bilingual proficiency");
        }
        None => {
            assert!(false, "No languages found");
        }
    }
}

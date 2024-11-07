use omicron_crawler::driver::driver_pool::DriverSessionPool;
use omicron_crawler::driver::driver_service::ChromeDriverService;
use omicron_crawler::linkedin::crawler::Crawler;
use omicron_crawler::logger::Logger;
use omicron_crawler::utils::{chrome_driver_path_from_env, driver_host_from_env, driver_port_from_env};
use std::sync::Arc;

#[tokio::test(flavor = "multi_thread")]
async fn test_parse_1() {
    Logger::init(log::LevelFilter::Trace);
    let host = driver_host_from_env();
    let port = driver_port_from_env();
    let path = chrome_driver_path_from_env();
    let _driver_service = ChromeDriverService::new(port.clone(), path.as_str()).await;
    let pool = DriverSessionPool::new(host.as_str(), port.as_str(), 1).await;
    {
        let proxy = pool.acquire().unwrap();
        let crawler = Crawler::new(proxy).await;
        let profile_url =
            "https://www.linkedin.com/sales/lead/ACwAAAWs1dABZXg7RDqKugFxlSeo7gasFL1FPHQ,NAME_SEARCH,cypw?_ntb=xTZht7tmSNWO81Egbmk6Xg%3D%3D";
        let results = fatal_unwrap_e!(crawler.parse_profile(profile_url).await, "{}");
        assert_eq!(results.name, "Matus Chochlik");
        assert_eq!(results.url, "https://www.linkedin.com/in/matus-chochlik-154a7827");
        assert!(results.profile_picture_url.len() > 0);
        println!("{}", results.profile_picture_url);
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
    pool.quit().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_parse_2() {
    Logger::init(log::LevelFilter::Trace);
    let host = driver_host_from_env();
    let port = driver_port_from_env();
    let path = chrome_driver_path_from_env();
    let _driver_service = ChromeDriverService::new(port.clone(), path.as_str()).await;
    let pool = DriverSessionPool::new(host.as_str(), port.as_str(), 1).await;
    let proxy = pool.acquire().unwrap();
    let crawler = Crawler::new(proxy).await;
    let profile_url =
        "https://www.linkedin.com/sales/lead/ACwAAAy0mZcBlmERvP-yDTL3gnlTLSELF6c7hrk,NAME_SEARCH,UDAQ?_ntb=aRijRPOnTBeuYBCnRY718Q%3D%3D";
    let results = fatal_unwrap_e!(crawler.parse_profile(profile_url).await, "{}");
    assert_eq!(results.name, "Patrik Bujňák");
    assert_eq!(results.url, "https://www.linkedin.com/in/patrik-buj%C5%88%C3%A1k-dev");
    assert!(results.profile_picture_url.len() > 0);
    println!("{}", results.profile_picture_url);
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

#[tokio::test(flavor = "multi_thread")]
async fn test_parse_3() {
    Logger::init(log::LevelFilter::Trace);
    let host = driver_host_from_env();
    let port = driver_port_from_env();
    let path = chrome_driver_path_from_env();
    let _driver_service = ChromeDriverService::new(port.clone(), path.as_str()).await;
    let pool = DriverSessionPool::new(host.as_str(), port.as_str(), 1).await;
    let proxy = pool.acquire().unwrap();
    let crawler = Crawler::new(proxy).await;
    let profile_url = "https://www.linkedin.com/sales/lead/ACwAACqD0w0BfMn9-aCXZ3eaubNSkpwpMw-3XLw,NAME_SEARCH,4Pzc";
    let results = fatal_unwrap_e!(crawler.parse_profile(profile_url).await, "{}");
    assert_eq!(results.name, "Peter Hamran");
    assert_eq!(results.url, "https://www.linkedin.com/in/peter-hamran-151a6317a");
    assert!(results.profile_picture_url.len() > 0);
    println!("{}", results.profile_picture_url);
    assert_eq!(results.description, "Cofounder of Sqeed s.r.o.");
    assert_eq!(results.location, "Slovakia");
    assert_eq!(results.about.is_some(), false);
    match results.experience {
        Some(experience) => {
            for experience in experience.iter() {
                println!("{}", *experience);
            }
            assert_eq!(experience.len(), 1);
        }
        None => {
            assert!(false, "No experience found");
        }
    }
    match results.education {
        Some(education) => {
            assert_eq!(education.len(), 2);
            assert_eq!(education[0].title, "Brno University of Technology");
            assert_eq!(education[0].field, "Inteligent Systems");
            assert_eq!(education[0].degree, "Masters");

            assert_eq!(education[1].title, "Brno University of Technology");
            assert_eq!(education[1].field, "Information Technology");
            assert_eq!(education[1].degree, "Bachelor's degree");
        }
        None => {
            assert!(false, "No education found");
        }
    }

    match results.skills {
        Some(skills) => {
            assert_eq!(skills.len(), 3);
        }
        None => {
            assert!(false, "No skills found");
        }
    }

    match results.languages {
        Some(_) => {
            assert!(false, "Languages should be empty");
        }
        None => {}
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_parse_4() {
    Logger::init(log::LevelFilter::Info);
    let host = driver_host_from_env();
    let port = driver_port_from_env();
    let path = chrome_driver_path_from_env();
    let _driver_service = ChromeDriverService::new(port.clone(), path.as_str()).await;
    let pool = DriverSessionPool::new(host.as_str(), port.as_str(), 2).await;
    let proxy = pool.acquire().unwrap();
    let crawler = Crawler::new(proxy).await;
    let profile_url = "https://www.linkedin.com/sales/lead/ACwAABpJtzoBf8gnSQxzTTAesZe6DCoutpzIcY0,NAME_SEARCH,ZBW0";
    let results = fatal_unwrap_e!(crawler.parse_profile(profile_url).await, "{}");
    assert_eq!(results.name, "Kamil Pšenák");
    assert_eq!(results.url, "https://www.linkedin.com/in/kamil-psenak");
    assert!(results.profile_picture_url.len() > 0);
    println!("{}", results.profile_picture_url);
    assert_eq!(results.description, "AI | Cybersecurity & ESET | Back To The Essentials");
    assert_eq!(results.location, "Bratislava, Slovakia");
    assert_eq!(results.about.is_some(), true);
    println!("{}", results.about.unwrap());
    match results.experience {
        Some(experience) => {
            for experience in experience.iter() {
                println!("{}", *experience);
            }
            assert_eq!(experience.len(), 5);
        }
        None => {
            assert!(false, "No experience found");
        }
    }
    match results.education {
        Some(education) => {
            assert_eq!(education.len(), 1);
            assert_eq!(education[0].title, "Brno University of Technology");
            assert_eq!(education[0].field, "Information Technology");
            assert_eq!(education[0].degree, "Bachelor's degree");
        }
        None => {
            assert!(false, "No education found");
        }
    }
    if let Some(_) = results.skills {
        assert!(false, "Skills should be empty");
    }
    match results.languages {
        Some(languages) => {
            assert_eq!(languages.len(), 2);
            assert_eq!(languages[0].language, "English");
            assert_eq!(languages[1].language, "Slovak");
        }
        None => {
            assert!(false, "No languages found");
        }
    }
}

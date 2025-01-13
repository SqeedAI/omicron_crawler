use omicron_crawler::linkedin::api::LinkedinSession;

#[tokio::test(flavor = "multi_thread")]
pub async fn api_auth_test() {
    let mut linkedin_session = LinkedinSession::new();
    if let Err(e) = linkedin_session.authenticate("erik9631@gmail.com", "soRMoaN7C2bX2mKbV9V4").await {
        assert!(false, "Failed to authenticate {}", e);
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn api_skills_test() {
    let mut linkedin_session = LinkedinSession::new();
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
    let mut linkedin_session = LinkedinSession::new();
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
    assert_eq!(profile.profile.location_name, "Slovak Republic");
    assert_eq!(profile.profile.industry_name, "Higher Education");
    assert_eq!(profile.profile.summary.as_ref().unwrap(), "Solution architect / Tech lead / SW engineer, mostly C++, Python. Member of C++ standards committee. Having experience with modern OpenGL programming and with relational database systems.\n\nSpecialties: software development, C++, meta-programming, OpenGL 4 programming");
    println!("{}", profile.profile.picture_url);
    assert_eq!(profile.education_view.elements.len(), 1);
    assert_eq!(profile.education_view.elements[0].school_name, "University of Zilina");
    assert_eq!(profile.education_view.elements[0].field_of_study, "Applied Computer Science");
    assert_eq!(profile.education_view.elements[0].degree_name, "PhD");
    assert_eq!(
        profile.education_view.elements[0].time_period.start_date.as_ref().unwrap().year,
        2005
    );
    assert_eq!(profile.education_view.elements[0].time_period.end_date.as_ref().unwrap().year, 2008);
    assert_eq!(profile.project_view.elements.len(), 3);
    assert_eq!(profile.project_view.elements[0].title, "JGL - A Java wrapper for OpenGL 3");
    assert_eq!(profile.project_view.elements[0].description, "This project is inspired on and aims to provide similar functionality as the OGLPlus C++ OpenGL framework. jgl makes use of the JOGL Java library to access OpenGL in a cross platform fashion.\n\nWith jgl, you get a set of wrapper classes which abstract most of the housekeeping tasks one inevitably faces when dealing with OpenGL. This framework is primarily focused on OpenGL 3 since it is the currently accepted programming standard for OpenGL and is on par with DirectX feature wise (if not more ;)).");
    assert_eq!(profile.project_view.elements[0].time_period.start_date.as_ref().unwrap().year, 2011);
    assert_eq!(
        profile.project_view.elements[0]
            .time_period
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
        profile.position_view.elements[0].time_period.start_date.as_ref().unwrap().year,
        2023
    );
    assert_eq!(
        profile.position_view.elements[0]
            .time_period
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
    let mut linkedin_session = LinkedinSession::new();
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
    assert_eq!(profile.profile.industry_name, "Computer Software".to_string());
    assert_eq!(profile.profile.headline, "Cofounder of Sqeed s.r.o.".to_string());
    assert_eq!(profile.position_view.elements.len(), 1);
    assert_eq!(profile.position_view.elements[0].title, "System Engineer".to_string());
    assert_eq!(profile.position_view.elements[0].company_name.as_ref().unwrap(), "ESET");
}

#[tokio::test(flavor = "multi_thread")]
pub async fn api_profile_test_3() {
    let mut linkedin_session = LinkedinSession::new();
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
    let mut linkedin_session = LinkedinSession::new();
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

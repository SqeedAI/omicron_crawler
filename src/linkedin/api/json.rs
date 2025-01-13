use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct FetchCookiesResponse {
    pub status: String,
}
#[derive(serde::Deserialize)]
pub struct AuthenticateResponse {
    pub login_result: String,
    pub challenge_url: String,
}

#[derive(serde::Deserialize)]
pub struct Date {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TimePeriod {
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Profile {
    pub education_view: EducationView,
    pub organization_view: OrganizationView,
    pub project_view: ProjectView,
    pub position_view: PositionView,
    pub profile: ProfileView,
    pub language_view: LanguageView,
    pub certification_view: CertificateView,
    pub test_score_view: TestScoreView,
    pub course_view: CourseView,
    pub honor_view: HonorView,
    pub skill_view: SkillView,
    pub volunteer_experience_view: VolunteerExperienceView,
    pub publication_view: PublicationView,
}
#[derive(serde::Deserialize)]
pub struct EducationView {
    pub elements: Vec<Education>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Education {
    pub degree_name: String,
    pub school_name: String,
    pub field_of_study: String,
    pub school_urn: String,
    pub time_period: TimePeriod,
}
#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OrganizationView {}

#[derive(serde::Deserialize)]
pub struct ProjectView {
    pub elements: Vec<Project>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Project {
    pub title: String,
    pub description: String,
    pub url: String,
    pub time_period: TimePeriod,
}
#[derive(serde::Deserialize)]
pub struct PositionView {
    pub elements: Vec<Position>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Position {
    pub title: String,
    pub location_name: Option<String>,
    pub description: Option<String>,
    pub time_period: TimePeriod,
    pub company_name: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProfileView {
    pub summary: Option<String>,
    pub industry_name: String,
    pub first_name: String,
    pub last_name: String,
    pub location_name: String,
    pub geo_country_name: String,
    pub headline: String,
    #[serde(deserialize_with = "deserialize_profile_url")]
    #[serde(rename = "miniProfile")]
    pub picture_url: String,
}

fn deserialize_profile_url<'a, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'a>,
{
    #[derive(serde::Deserialize)]
    struct Helper {
        picture: Picture,
    }

    #[derive(serde::Deserialize)]
    struct Picture {
        #[serde(rename = "com.linkedin.common.VectorImage")]
        vector_image: VectorImage,
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all(deserialize = "camelCase"))]
    struct VectorImage {
        artifacts: Vec<Artifact>,
        root_url: String,
    }
    #[derive(serde::Deserialize)]
    #[serde(rename_all(deserialize = "camelCase"))]
    struct Artifact {
        file_identifying_url_path_segment: String,
    }

    let helper = Helper::deserialize(deserializer)?;
    Ok(format!(
        "{}{}",
        helper.picture.vector_image.root_url, helper.picture.vector_image.artifacts[0].file_identifying_url_path_segment
    ))
}
#[derive(serde::Deserialize)]
pub struct LanguageView {
    pub elements: Vec<Language>,
}
#[derive(serde::Deserialize)]
pub struct Language {
    pub name: String,
    pub proficiency: Option<String>,
}
#[derive(serde::Deserialize)]
pub struct CertificateView {
    pub elements: Vec<Certificate>,
}
#[derive(serde::Deserialize)]
pub struct Certificate {}
#[derive(serde::Deserialize)]
pub struct TestScoreView {
    pub elements: Vec<TestScore>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TestScore {}
#[derive(serde::Deserialize)]
pub struct CourseView {
    pub elements: Vec<Course>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Course {}
#[derive(serde::Deserialize)]
pub struct HonorView {
    pub elements: Vec<Honor>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Honor {}

#[derive(serde::Deserialize)]
pub struct SkillView {
    pub elements: Vec<Skill>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Skill {
    pub name: String,
    pub entity_urn: String,
}

#[derive(serde::Deserialize)]
pub struct VolunteerExperienceView {
    pub elements: Vec<VolunteerExperience>,
}

#[derive(serde::Deserialize)]
pub struct VolunteerExperience {}

#[derive(serde::Deserialize)]
pub struct PublicationView {
    pub elements: Vec<Publication>,
}

#[derive(serde::Deserialize)]
pub struct Publication {
    pub date: Date,
    pub name: String,
    pub publisher: String,
    pub url: String,
}

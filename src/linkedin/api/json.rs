use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum GeoUrnMap {
    Czechia,
    Slovakia,
}

impl Display for GeoUrnMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoUrnMap::Czechia => write!(f, "104508036"),
            GeoUrnMap::Slovakia => write!(f, "103119917"),
        }
    }
}
impl FromStr for GeoUrnMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "czechia" => Ok(GeoUrnMap::Czechia),
            "slovakia" => Ok(GeoUrnMap::Slovakia),
            _ => Err(()),
        }
    }
}
#[derive(serde::Deserialize)]
pub struct SearchParams {
    pub countries: Option<Vec<GeoUrnMap>>,
    pub keywords: Option<String>,
    pub keyword_first_name: Option<String>,
    pub keyword_last_name: Option<String>,
    pub keyword_title: Option<String>,
    pub keyword_company: Option<String>,
    pub keyword_school: Option<String>,
    pub profile_language: Option<Vec<String>>,
    pub page: u32,
    pub end: u32,
}
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
pub struct SearchResult {
    pub elements: Vec<SearchItem>,
    pub total: u64,
}
impl<'de> Deserialize<'de> for SearchResult {
    fn deserialize<D>(deserializer: D) -> Result<SearchResult, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Root {
            data: Data,
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Data {
            search_dash_clusters_by_all: SearchDashClusters,
        }

        #[derive(serde::Deserialize)]
        struct SearchDashClusters {
            elements: Vec<SearchMetaItem>,
            metadata: Metadata,
        }
        #[derive(serde::Deserialize)]
        #[serde(rename_all(deserialize = "camelCase"))]
        struct Metadata {
            total_result_count: u64,
        }

        #[derive(serde::Deserialize)]
        struct SearchMetaItem {
            #[serde(deserialize_with = "deserialize_search_item")]
            pub items: Vec<SearchItem>,
        }

        // Deserialize into the intermediate structure
        let root = Root::deserialize(deserializer)?;

        if (root.data.search_dash_clusters_by_all.elements.len() == 0) {
            return Ok(SearchResult {
                elements: Vec::new(),
                total: 0,
            });
        }
        let mut items = Vec::with_capacity(root.data.search_dash_clusters_by_all.elements[0].items.len());
        let total = root.data.search_dash_clusters_by_all.metadata.total_result_count;
        for item in root.data.search_dash_clusters_by_all.elements[0].items.iter() {
            items.push(item.clone());
        }

        Ok(SearchResult { elements: items, total })
    }
}

fn deserialize_search_item<'de, D>(deserializer: D) -> Result<Vec<SearchItem>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(rename_all(deserialize = "camelCase"))]
    struct ItemInner {
        item: ItemEntity,
    }
    #[derive(serde::Deserialize)]
    #[serde(rename_all(deserialize = "camelCase"))]
    struct ItemEntity {
        entity_result: Item,
    }
    #[derive(serde::Deserialize)]
    #[serde(rename_all(deserialize = "camelCase"))]
    struct Item {
        pub title: Title,
        pub primary_subtitle: PrimarySubtitle,
        pub summary: Option<Summary>,
        pub navigation_url: String,
    }
    #[derive(serde::Deserialize)]
    struct Title {
        pub text: String,
    }
    #[derive(serde::Deserialize)]

    struct PrimarySubtitle {
        pub text: String,
    }

    #[derive(serde::Deserialize)]
    struct Summary {
        pub text: String,
    }

    //TODO possible case with three names
    let items: Vec<ItemInner> = Vec::deserialize(deserializer)?;
    let mut out = Vec::with_capacity(items.len());
    for item_inner in items {
        let mut name_split = item_inner.item.entity_result.title.text.split(" ");
        let first_name = name_split.next().unwrap().to_string();
        let last_name = name_split.next().unwrap().to_string();
        let summary = match item_inner.item.entity_result.summary {
            Some(summary) => Some(summary.text),
            None => None,
        };
        out.push(SearchItem {
            first_name,
            last_name,
            subtitle: item_inner.item.entity_result.primary_subtitle.text,
            summary,
            url: item_inner.item.entity_result.navigation_url,
        });
    }
    Ok(out)
}

#[derive(serde::Deserialize, Clone)]
pub struct SearchItem {
    pub first_name: String,
    pub last_name: String,
    pub subtitle: String,
    pub summary: Option<String>,
    pub url: String,
}

use std::fmt;
use std::fmt::Display;
#[derive(serde::Deserialize, Debug)]
pub enum String {
    Accounting,
    Administrative,
    ArtsAndDesign,
    BusinessDevelopment,
    CommunityAndSocialServices,
    Consulting,
    Education,
    Engineering,
    Entrepreneurship,
    Finance,
    HealthServices,
    HumanResources,
    InformationTechnology,
    Legal,
    Marketing,
    MediaAndCommunications,
    Operations,
    ProductManagement,
    ProgramAndProjectManagement,
    Purchasing,
    QualityAssurance,
    RealEstate,
    Research,
    Sales,
    CustomerSuccessAndSupport,
}

impl Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            String::Accounting => "Accounting",
            String::Administrative => "Administrative",
            String::ArtsAndDesign => "Arts and Design",
            String::BusinessDevelopment => "Business Development",
            String::CommunityAndSocialServices => "Community and Social Services",
            String::Consulting => "Consulting",
            String::Education => "Education",
            String::Engineering => "Engineering",
            String::Entrepreneurship => "Entrepreneurship",
            String::Finance => "Finance",
            String::HealthServices => "Health Services",
            String::HumanResources => "Human Resources",
            String::InformationTechnology => "Information Technology",
            String::Legal => "Legal",
            String::Marketing => "Marketing",
            String::MediaAndCommunications => "Media and Communications",
            String::Operations => "Operations",
            String::ProductManagement => "Product Management",
            String::ProgramAndProjectManagement => "Program and Project Management",
            String::Purchasing => "Purchasing",
            String::QualityAssurance => "Quality Assurance",
            String::RealEstate => "Real Estate",
            String::Research => "Research",
            String::Sales => "Sales",
            String::CustomerSuccessAndSupport => "Customer Success and Support",
        };
        write!(f, "{}", s)
    }
}
#[derive(serde::Deserialize)]
pub enum SeniorityLevel {
    Owner,
    CXO,
    VicePresident,
    Director,
    ExperienceManager,
    EntryLevelManager,
    Strategic,
    Senior,
    EntryLevel,
    InTraining,
}

impl Display for SeniorityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SeniorityLevel::Owner => "Owner",
            SeniorityLevel::CXO => "CXO",
            SeniorityLevel::VicePresident => "Vice President",
            SeniorityLevel::Director => "Director",
            SeniorityLevel::ExperienceManager => "Experience Manager",
            SeniorityLevel::EntryLevelManager => "Entry Level Manager",
            SeniorityLevel::Strategic => "Strategic",
            SeniorityLevel::Senior => "Senior",
            SeniorityLevel::EntryLevel => "Entry Level",
            SeniorityLevel::InTraining => "In Training",
        };
        write!(f, "{}", s)
    }
}

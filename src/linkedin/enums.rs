use std::fmt;
use std::fmt::Display;

pub enum Functions {
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

impl Display for Functions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Functions::Accounting => "Accounting",
            Functions::Administrative => "Administrative",
            Functions::ArtsAndDesign => "Arts and Design",
            Functions::BusinessDevelopment => "Business Development",
            Functions::CommunityAndSocialServices => "Community and Social Services",
            Functions::Consulting => "Consulting",
            Functions::Education => "Education",
            Functions::Engineering => "Engineering",
            Functions::Entrepreneurship => "Entrepreneurship",
            Functions::Finance => "Finance",
            Functions::HealthServices => "Health Services",
            Functions::HumanResources => "Human Resources",
            Functions::InformationTechnology => "Information Technology",
            Functions::Legal => "Legal",
            Functions::Marketing => "Marketing",
            Functions::MediaAndCommunications => "Media and Communications",
            Functions::Operations => "Operations",
            Functions::ProductManagement => "Product Management",
            Functions::ProgramAndProjectManagement => "Program and Project Management",
            Functions::Purchasing => "Purchasing",
            Functions::QualityAssurance => "Quality Assurance",
            Functions::RealEstate => "Real Estate",
            Functions::Research => "Research",
            Functions::Sales => "Sales",
            Functions::CustomerSuccessAndSupport => "Customer Success and Support",
        };
        write!(f, "{}", s)
    }
}

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

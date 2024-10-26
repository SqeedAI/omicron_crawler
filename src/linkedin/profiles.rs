use std::fmt::{Display, Formatter};

pub struct SearchResult {
    pub name: String,
    pub title: String,
    pub sales_url: String,
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {} title: {} url: {}", self.name, self.title, self.sales_url)
    }
}
#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub url: String,
    pub description: String,
    pub about: String,
    pub location: String,
    pub connections: String,
    pub experience: Vec<Experience>,
    pub education: Vec<Experience>,
    pub skills: Vec<String>,
    pub languages: Vec<Language>,
    pub industry: String,
    pub seniority: String,
}
#[derive(Debug)]
pub struct Experience {
    pub title: String,
    pub duration: Duration,
}
#[derive(Debug)]
pub struct Education {
    pub title: String,
    pub degree: String,
    pub duration: Duration,
}
#[derive(Debug)]
pub struct Duration {
    pub start: String,
    pub end: String,
}
#[derive(Debug)]
pub struct Language {
    pub language: String,
    pub fluency: String,
}

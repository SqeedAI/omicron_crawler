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
    pub profile_picture_url: String,
    pub description: String,
    pub about: Option<String>,
    pub location: String,
    pub experience: Option<Vec<Experience>>,
    pub education: Option<Vec<Education>>,
    pub skills: Option<Vec<Skill>>,
    pub languages: Option<Vec<Language>>,
}
impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}\nurl: {}\ndescription: {}\nlocation: {}\nprofile picture: {}\n",
            self.name, self.url, self.description, self.location, self.profile_picture_url
        )?;

        if let Some(about) = &self.about {
            write!(f, "About: {}\n", about)?;
        }

        if let Some(experience) = &self.experience {
            write!(f, "Experience:\n")?;
            for experience in experience.iter() {
                write!(f, "{}\n", *experience)?;
            }
        }
        if let Some(education) = &self.education {
            write!(f, "Education:\n")?;
            for education in education.iter() {
                write!(f, "{}\n", *education)?;
            }
        }
        if let Some(skills) = &self.skills {
            write!(f, "Skills:\n")?;
            for skill in skills.iter() {
                write!(f, "{}\n", *skill)?;
            }
        }

        if let Some(languages) = &self.languages {
            write!(f, "Languages:\n")?;
            for language in languages.iter() {
                write!(f, "{}\n", *language)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Experience {
    pub position: String,
    pub interval: Interval,
}
impl Display for Experience {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "title: {} {}", self.position, self.interval)
    }
}
#[derive(Debug)]
pub struct Education {
    pub title: String,
    pub field: String,
    pub degree: String,
    pub interval: Interval,
}
impl Display for Education {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "title: {}\ndegree: {}\nduration: {:?}", self.title, self.degree, self.interval)
    }
}
#[derive(Debug)]
pub struct Interval {
    pub start: String,
    pub end: String,
}

impl Interval {
    pub fn from_str(s: &str, delimiter: &str) -> Result<Self, &'static str> {
        s.split_once(delimiter)
            .map(|(start, end)| Interval {
                start: start.to_string(),
                end: end.to_string(),
            })
            .ok_or("failed to find delimiter")
    }
}
impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {} end: {}", self.start, self.end)
    }
}
#[derive(Debug)]
pub struct Language {
    pub language: String,
    pub fluency: String,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "language: {} fluency: {}", self.language, self.fluency)
    }
}

#[derive(Debug)]
pub struct Skill {
    pub name: String,
    pub endorsements: u16,
}
impl Display for Skill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "skill: {} endorsements: {}", self.name, self.endorsements)
    }
}

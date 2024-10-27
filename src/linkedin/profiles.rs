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
    pub about: Option<String>,
    pub location: String,
    pub connections: String,
    pub experience: Option<Vec<Experience>>,
    pub education: Option<Vec<Education>>,
    pub skills: Vec<String>,
    pub languages: Vec<Language>,
}
impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}\nurl: {}\ndescription: {}\nlocation: {}\nconnections: {}\n",
            self.name, self.url, self.description, self.location, self.connections
        )?;

        if let Some(about) = &self.about {
            write!(f, "About: {}\n", about)?;
        }

        if let Some(experience) = &self.experience {
            write!(f, "\tExperience:\n")?;
            for experience in experience.iter() {
                write!(f, "\t{}\n", *experience)?;
            }
        }
        if let Some(education) = &self.education {
            write!(f, "\tEducation:\n")?;
            for education in education.iter() {
                write!(f, "\t{}\n", *education)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Experience {
    pub position: String,
    pub start: String,
    pub end: String,
}
impl Display for Experience {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "title: {} {} - {}", self.position, self.start, self.end)
    }
}
#[derive(Debug)]
pub struct Education {
    pub title: String,
    pub degree: String,
    pub duration: Duration,
}
impl Display for Education {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "title: {} degree: {} duration: {:?}", self.title, self.degree, self.duration)
    }
}
#[derive(Debug)]
pub struct Duration {
    pub start: String,
    pub end: String,
}
impl Display for Duration {
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

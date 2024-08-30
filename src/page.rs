use std::result;

use regex::Regex;
use scraper::Selector;

pub struct DetailsPage {
    // html: String,
    doc: scraper::Html,
}

impl DetailsPage {
    pub fn new(html: &str) -> Self {
        DetailsPage {
            // html: String::from(html),
            doc: scraper::Html::parse_document(html),
        }
    }

    // fn parse_document(&self) -> scraper::Html {
    //     scraper::Html::parse_document(&self.html)
    // }

    pub fn is_logined(&self) -> bool {
        match self.name() {
            Ok(name) => name.len() != 0,
            _ => false,
        }
    }

    pub fn name(&self) -> Result<String> {
        let name_selector =
            Selector::parse("#logoutForm > ul > li:nth-child(1) > a > span.text-success").unwrap();

        match self.doc.select(&name_selector).next() {
            Some(name) => Ok(serialize_string(name.text().collect::<String>().as_str())),
            _ => Err(PageError::ParseError(
                "Cannot find name element".to_string(),
            )),
        }
    }

    pub fn class(&self) -> Result<String> {
        let class_selector =
            Selector::parse("#logoutForm > ul > li:nth-child(1) > ul > li:nth-child(3) > a > span")
                .unwrap();

        match self.doc.select(&class_selector).next() {
            Some(class) => Ok(serialize_string(class.text().collect::<String>().as_str())),
            _ => Err(PageError::ParseError(
                "Cannot find class element".to_string(),
            )),
        }
    }

    pub fn courses(&self) -> Result<Vec<Course>> {
        let courses_selector = Selector::parse(
            "#PrintArea > div:nth-child(2) > table:nth-child(2) > tbody:nth-child(1) > tr:not(:first-child)",
        ).unwrap();
        let td_selector = Selector::parse("td").unwrap();

        let mut courses = Vec::new();

        for c in self.doc.select(&courses_selector) {
            let texts = c
                .select(&td_selector)
                .map(|x| x.text().collect::<String>())
                .collect::<Vec<_>>();

            if texts.len() < 6 {
                return Err(PageError::ParseError(
                    "Parse courses info failed".to_string(),
                ));
            }

            let course_no = serialize_string(texts[0].as_str());
            let name = serialize_string(texts[1].as_str());
            let credits = serialize_string(texts[2].as_str())
                .parse::<f32>()
                .map_err(|err| PageError::ParseError(format!("Parse credits failed: {}", err)))?;
            let required = serialize_string(texts[3].as_str());
            let teacher = serialize_string(texts[4].as_str());
            let notes = serialize_string(texts[5].as_str());

            courses.push(Course {
                course_no,
                name,
                credits,
                required,
                teacher,
                notes,
            });
        }

        Ok(courses)
    }

    pub fn to_string(&self) -> String {
        self.doc.html()
    }
}

pub struct Course {
    pub course_no: String,
    pub name: String,
    pub credits: f32,
    pub required: String,
    pub teacher: String,
    pub notes: String,
}

fn serialize_string(v: &str) -> String {
    v.replace("\r", "")
        .replace("\n", "")
        .replace("\t", "")
        .replace("                ", "")
        .trim()
        .to_string()
}

pub struct SelectResultPage {
    html: String,
}

impl SelectResultPage {
    pub fn new(html: &str) -> Self {
        SelectResultPage {
            html: String::from(html),
        }
    }

    pub fn result_message(&self) -> Option<String> {
        let re = Regex::new(
            r#"<script type="text\/javascript">[^<]*alert\(['"]([^'"]+)['"]\);[^<]*<\/script>"#,
        )
        .unwrap();

        match re.captures(&self.html.as_str()) {
            Some(captures) => match captures.get(1) {
                Some(message) => Some(String::from(message.as_str())),
                None => None,
            },
            None => None,
        }
    }
}

#[derive(Debug)]
pub enum PageError {
    ParseError(String),
    NotFound(String),
    Other(String),
}

impl std::fmt::Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PageError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            PageError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            PageError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl std::error::Error for PageError {}

pub type Result<T> = result::Result<T, PageError>;

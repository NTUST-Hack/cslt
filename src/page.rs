use scraper::Selector;

pub struct DetailsPage {
    doc: scraper::Html,
}

impl DetailsPage {
    pub fn new(html: &str) -> Self {
        DetailsPage {
            doc: scraper::Html::parse_document(html),
        }
    }

    pub fn is_logined(&self) -> bool {
        match self.name() {
            Ok(name) => name.len() != 0,
            _ => false,
        }
    }

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let name_selector =
            Selector::parse("#logoutForm > ul > li:nth-child(1) > a > span.text-success")?;

        match self.doc.select(&name_selector).next() {
            Some(name) => Ok(serialize_string(name.text().collect::<String>().as_str())),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Cannot find name element",
            ))),
        }
    }

    pub fn class(&self) -> Result<String, Box<dyn std::error::Error>> {
        let class_selector = Selector::parse(
            "#logoutForm > ul > li:nth-child(1) > ul > li:nth-child(3) > a > span",
        )?;

        match self.doc.select(&class_selector).next() {
            Some(class) => return Ok(serialize_string(class.text().collect::<String>().as_str())),
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Cannot find class element",
                )))
            }
        }
    }

    pub fn courses(&self) -> Result<Vec<Course>, Box<dyn std::error::Error>> {
        let courses_selector = Selector::parse(
            "#PrintArea > div:nth-child(2) > table:nth-child(2) > tbody:nth-child(1) > tr:not(:first-child)",
        )?;
        let td_selector = Selector::parse("td")?;

        let mut courses = Vec::new();

        for c in self.doc.select(&courses_selector) {
            let texts = c
                .select(&td_selector)
                .map(|x| x.text().collect::<String>())
                .collect::<Vec<_>>();

            if texts.len() < 6 {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Parse courses info failed",
                )));
            }

            let course_no = serialize_string(texts[0].as_str());
            let name = serialize_string(texts[1].as_str());
            let credits = serialize_string(texts[2].as_str()).parse::<f32>()?;
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
}

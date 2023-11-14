use regex::Regex;

pub struct FileCreator {
    pub path: String,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl From<String> for FileCreator {
    fn from(value: String) -> Self {
        let date_extractor_regex =
            Regex::new(r"[0-9a-zA-Z]*_(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})_[0-9a-zA-Z]*")
                .expect("FileCreate: unable to create extractor regex");
        let path = value.clone();
        let capture = date_extractor_regex
            .captures(&path)
            .expect(&format!(
                "FileCreator: cannot extract YYYY-MM-DD from '{value}'."
            ));

        let year: u16 = capture["year"].parse().expect(&format!(
            "Unable to parse year after capturing {} from regex",
            &capture["year"]
        ));
        let month: u8 = capture["month"].parse().expect(&format!(
            "Unable to parse month after capturing {} from regex",
            &capture["month"]
        ));
        let day: u8 = capture["day"].parse().expect(&format!(
            "Unable to parse day after capturing {} from regex",
            &capture["day"]
        ));

        FileCreator {
            path,
            year,
            month,
            day,
        }
    }
}

impl From<&str> for FileCreator {
    fn from(value: &str) -> Self {
        FileCreator::from(String::from(value).clone())
    }
}

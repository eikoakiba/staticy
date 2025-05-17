use chrono::{DateTime, Local, NaiveDate};

#[derive(Default, Debug)]
pub struct Content {
    pub name: String,
    pub title: String,
    pub info: String,
    pub date: NaiveDate,
}

impl Content {
    // @STATIC
    pub fn new(name: String, title: String, info: String, date: NaiveDate) -> Self {
        Content {
            name,
            title,
            info,
            date,
        }
    }
    pub fn get_link(&self) -> String {
        format!("'{}.html'", self.name)
    }
}

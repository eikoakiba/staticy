#[derive(Default, Debug)]
pub struct Content {
    pub name: String,
    pub title: String,
    pub info: String,
    pub date: String,
}

impl Content {
    pub fn new(name: String, title: String, info: String, date: String) -> Self {
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

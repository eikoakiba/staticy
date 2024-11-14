use chrono::Utc;

pub fn time_handler(event: &String) -> String {
    Utc::now().to_string()
}

pub fn bash_handler(event: &String) -> String {
    // Run the process with the value event
    format!("Hello")
}

pub fn image_handler(event: &String) -> String {
    format!("<img src=\"{}\"></img>", event.replace("'", ""))
}

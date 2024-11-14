use chrono::Utc;
use std::{fs, io::Write};

use crate::content;
use crate::file;

pub fn read_base(which: &str) -> String {
    // panic if can't find the file with expect
    let base_content_sample: String =
        fs::read_to_string(&format!("base/{}", which)).expect("ERROR: Can't open the base file"); // read
    return base_content_sample;
}

pub fn save_contents(ret_str: String, file_name: &String) -> content::Content {
    // Save the HTML contents files
    let mut base_content: String = file::read_base("blog.html"); // read the base file
    let base_content_lines: Vec<&str> = ret_str.split("\n").collect();
    let title: String = base_content_lines[0].to_string();
    let date: String = match base_content_lines[1] {
        "-" => Utc::now().to_string(),
        value => value.to_string(),
    };
    let info: String = base_content_lines[2].to_string();
    let content: content::Content = content::Content::new(
        file_name.clone(), // instead of to_string
        title.clone(),
        info.clone(),
        date.clone(),
    );

    base_content = base_content
        .replace("<ContentTitle/>", &title)
        .replace("<ContentDate/>", &date)
        .replace("<ContentInfo/>", &info)
        .replace(
            "<Content/>",
            &base_content_lines[3..base_content_lines.len()].join("<br/>"),
        ); // Put Actual Value
    let mut file = fs::File::create(format!("./dist/{file_name}.html")).unwrap(); // Create html
    let _ = file.write_all(base_content.as_bytes()); // Write content inside the html file
    content
}

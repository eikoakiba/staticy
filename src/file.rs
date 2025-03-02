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

pub fn save_contents(
    ret_str: &str,
    file_name: &str,
    title: &str,
    info: &str,
    date: &str,
) -> content::Content {
    // Save the HTML contents files
    let mut base_content: String = file::read_base("blog.html"); // read the base file
    let base_content_lines: Vec<&str> = ret_str.split("\n").collect();
    let content: content::Content = content::Content::new(
        file_name.to_string(), // instead of to_string
        title.to_string(),
        info.to_string(),
        date.to_string(),
    );

    base_content = base_content
        .replace("<ContentTitle/>", &title)
        .replace("<ContentDate/>", &date)
        .replace("<ContentInfo/>", &info)
        .replace(
            "<Content/>",
            &base_content_lines[2..base_content_lines.len()].join(""),
        ); // Put Actual Value
    let mut file = fs::File::create(format!("dist/{file_name}.html"));
    if let Err(msg) = file {
        println!("{}", msg.to_string())
    } else if let Ok(mut file_u) = file {
        let _ = file_u.write_all(base_content.as_bytes()); // Write content inside the html file
    }
    content
}

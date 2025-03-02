use chrono::prelude::Utc;
use content::Content;
use file::save_contents;
use markdown::Options;
use phf::phf_map;
use std::{default, fs, io::Write};
use walkdir::WalkDir;

pub mod content; // it's pub because main.rs will use this
mod file;
mod magic;

type Callback = fn(event: &String) -> String;

#[derive(Debug)]
struct Handler {
    name: &'static str,
    runner: Callback,
}

impl Handler {
    const fn new(name: &'static str, runner: Callback) -> Self {
        // this function should be const
        // cause static feature of the
        // static map.
        Handler { name, runner }
    }
    fn run(&self, event: &String) -> Option<String> {
        Some((self.runner)(event))
    }
}

// add this to the magic file
static COUNTRIES: phf::Map<&'static str, Handler> = phf_map! {
    //"time" => Handler{name: "time", runner: time_handler},
    "time" => Handler::new("time", magic::time_handler),
    "image" => Handler::new("image", magic::image_handler),
    "bash" => Handler::new("bash", magic::bash_handler), // run a bash script withing staticily process
                                                  // and return the STDOUT to use as html's blog
                                                  // content.
};

//// TODO: Make a Lexer(Token) interface for this function
//fn magic(input: String, file_name: &String) -> Option<content::Content> {
//    let mut ret_str: String = input.clone();
//    let mut state: usize = 0;
//    let mut event: String = String::default();
//    let mut key: String = String::default();
//    let mut chr: usize = 0;
//
//    // For performance stuff - TODO: Better string conrains handeling
//    if ret_str.contains("(%") {
//        while let Some(token) = ret_str.chars().nth(chr) {
//            chr += 1;
//            if (token == '(' && ret_str.chars().nth(chr).unwrap() == '%') {
//                chr += 1; // to remove the %
//                state = 1;
//                continue;
//            }
//            /*
//            if (state == 3 && token != '\n') {
//                continue;
//            }
//            */
//            if (token == ')' && state != 0) {
//                println!("{} > {}", key, &event);
//                let handler: &Handler;
//                if let Some(res) = COUNTRIES.get(&key) {
//                    handler = res;
//                } else {
//                    println!("WARNING: Can't any handler for the key {}", &key); // return the string
//                                                                                 // instead of println it
//                                                                                 // :D
//                    return None;
//                }
//
//                //println!("#{}={}", &key, &event);
//                let mut repl_str: String = String::default();
//                if let Some(value) = handler.run(&event) {
//                    if state == 1 {
//                        repl_str = format!("(%{})", &key);
//                    } else {
//                        repl_str = format!("(%{} {})", &key, &event);
//                    }
//                    ret_str = ret_str.replace(repl_str.as_str(), &value);
//
//                    // replace the buffer with the new value
//                } else {
//                    return None;
//                }
//                // Reset All values to the default
//                key = String::default();
//                event = String::default();
//                state = 0;
//            }
//
//            if state == 1 && token == ' ' {
//                state = 2;
//                continue;
//            }
//
//            if state == 1 {
//                // event += &token.to_string();
//                key += &token.to_string();
//            } else if state == 2 {
//                event += &token.to_string();
//            } else if state == 3 && token != '\n' {
//                continue;
//            }
//        }
//    }
//    let ret_content: content::Content = file::save_contents(ret_str, file_name);
//    Some(ret_content)
//}

pub fn generate_blog(files: Vec<content::Content>) -> Option<String> {
    let mut base_blog_cont: String = file::read_base("index.html");
    if let Some(value) = base_blog_cont.find("<ContentList>") {
        let base_blog_tag: usize = base_blog_cont
            .find("</ContentList")
            .expect("ERROR: Please close the ContentList Element");
        let mut chr: usize = value + "<ContentList>".len();
        let mut token: char = char::default();
        let mut element_inner: String = String::default();
        loop {
            chr += 1;
            token = base_blog_cont.chars().nth(chr).unwrap();

            if chr >= base_blog_tag {
                break;
            }

            element_inner += &token.to_string();
        }

        let form_repl: String = format!("<ContentList>\n{}</ContentList>", element_inner);
        let mut final_cont: String = String::default();
        files.iter().for_each(|x| {
            final_cont += &element_inner
                .replace("<BlogName/>", &x.title)
                .replace("<BlogLink/>", &x.get_link())
                .replace("<BlogDate/>", &x.date)
                .replace("<BlogInfo/>", &x.info);
        });
        //println!("{}", final_cont);
        base_blog_cont = base_blog_cont.replace(&form_repl, &final_cont);

        let mut file = fs::File::create("./dist/index.html").unwrap(); // Create html
        let _ = file.write_all(base_blog_cont.as_bytes()); // Write content inside the html file
    } else {
        return None;
    }
    Some(String::from("GOOD"))
}

////pub fn generate_base_blog(files: Vec<String>) -> Result<(), String> {
//pub fn generate_contents() -> Result<Vec<content::Content>, String> {
//    let mut files: Vec<content::Content> = Vec::default(); // == Vec::new();
//    for file in WalkDir::new("./content")
//        .into_iter()
//        .filter_map(|file| file.ok())
//    {
//        let file_name_main: &str = file.file_name().to_str().unwrap();
//        // Check if we have the same dir name or something other than files
//        let file_name: String = file.path().display().to_string();
//        if file_name == "./content".to_string()
//            || !file.metadata().unwrap().is_file()
//            || !file_name.contains(".con")
//        // beta
//        {
//            println!(
//                "WARNING: This file {} can't process because it's not a .con file",
//                file_name_main
//            );
//            continue;
//        }
//
//        let file_content: String = match fs::read_to_string(&file_name) {
//            Ok(value) => value.trim().to_string(),
//            Err(error) => panic!("{}", error),
//        };
//
//        match magic(
//            file_content,
//            &file_name_main[0..file_name_main.len() - 4].to_string(),
//        ) {
//            Some(value) => {
//                files.push(value);
//            }
//            None => {
//                println!("WARNING: This file didn't provide any magic symbols")
//            }
//        };
//
//        //files.push(res);
//
//        //println!("{}", file.path().display());
//    }
//    Ok(files)
//}

pub fn generate_html() -> Result<Vec<content::Content>, String> {
    let mut files: Vec<Content> = Vec::default(); // == Vec::new();
    for file in WalkDir::new("./content")
        .into_iter()
        .filter_map(|file| file.ok())
    {
        let file_name_main: &str = file.file_name().to_str().unwrap();

        // Check if we have the same dir name or something other than files
        let file_name: String = file.path().display().to_string();
        if file_name == "./content"
            || !file.metadata().unwrap().is_file()
            || !file_name.contains(".md")
        {
            println!(
                "INFO: This file {} can't process because it's not .md file",
                file_name_main
            );
            continue;
        }

        let file_content: String = match fs::read_to_string(&file_name) {
            Ok(value) => value.trim().to_string(),
            Err(error) => panic!("{}", error),
        };

        //match magic(
        //    file_content,
        //    &file_name_main[0..file_name_main.len() - 4].to_string(),
        //) {
        //    Some(value) => {
        //        files.push(value);
        //    }
        //    None => {
        //        println!("WARNING: This file didn't provide any magic symbols")
        //    }
        //};
        let res = markdown::to_html_with_options(&file_content, &Options::gfm());
        if let Err(msg) = res {
            return Err(msg.to_string());
        } else if let Ok(html_content) = res {
            let res_chr: Vec<&str> = html_content.split('\n').collect();
            let title = res_chr[0];
            let info = res_chr[1];
            let date: String = Utc::now().to_string();

            let content: Content =
                save_contents(&html_content, &file_name_main, &title, &info, &date);

            files.push(content);
        }
    }
    Ok(files)
}

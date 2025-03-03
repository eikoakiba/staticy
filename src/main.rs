use log::{error, warn};
use staticy::content::Content;

fn main() {
    // Set the Base files as base blog file
    let result = staticy::generate_html();
    if let Err(ref err) = result {
        error!("ERROR: {}", err);
    }
    let result = result.unwrap();
    let base = staticy::generate_blog(&result);
    match base {
        Ok(()) => println!("Awesome! All files generated inside dist/ folder"),
        Err(err) => error!("ERROR: {}", err),
    }

    // Read each Contetn file
    // Render the fansy text converter
    // write/create them base on base blog file
    // save them inside a directory called blog
    //
    // Updates:
    // Add config file for more things
    // Read blog content base on STDIN
    // Add a good tool for temping the day by day things and gather them and make it a content
    //
    // Fansy Text Render Options:
    // 1. Provide images inside content file (as file names)
    // 2. Render text with the special class name
    // 3. provide current time
    // 4. and more things.
}

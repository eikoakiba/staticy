use log::error;
use staticy::sort_by_time;

fn main() {
    // Set the Base files as base blog file
    let result = staticy::generate_html();
    if let Err(ref err) = result {
        println!("ERROR: {}", err);
        return;
    }

    let mut result = result.unwrap();
    sort_by_time(&mut result);
    let base = staticy::generate_blog(&result);
    match base {
        Ok(()) => println!("Awesome! All files generated inside dist/ folder"),
        Err(err) => println!("ERROR: {}", err),
    }

    // Read each Contetn file
}

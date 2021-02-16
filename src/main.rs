// MAIN OBJECTIVES //
//
// Print a banner with package information
// Output:
//  //  [CARGO_PKG_NAME] (v[CARGO_PKG_VERSION]), [CARGO_PKG_DESCRIPTION]
//  //  Written by [CARGO_PKG_AUTHORS]
//  //  Homepage: [CARGO_PKG_HOMEPAGE]
//  //  Usage: bitsymd <somefile.md>
//
// Create a program with the following lifecycle:
// Given a call to bitsymd...
//   When a markdown file is passed as an argument it will:
//      1. open the file
//      2. parse the file line by life into a buffer
//      3. export the buffer to a new html file
//   When anything else or no argument is passed it should:
//      1. show the banner

fn get_title() -> String {
    // Create local String var to hold data we want for our output
    // Var must be mutable since we'll append other strings to it
    // The strings to append (ref output at top):
    //  1.  " (v"
    //  2.  env!("CARGO_PKG_VERSION")
    //  3.  "), "
    //  4.  env!("CARGO_PKG_DESCRIPTION")
    //
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));

    // To append a `&str` use .push_str()
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    // Need a return statement for the get_title function to actually return the string
    // we just built!
    return the_title;
}

// Print first line of banner (title, version, description)
fn print_short_banner() {
    // println!() only accepts string literals, so we must use substitution ("{}")
    println!("{}", get_title());
}

// Print full banner (short_banner + author + usage example)
fn print_long_banner() {
    // Create a string variable for each line of the banner. First line covered already.
    //  1. "Written by: " + env!("CARGO_PKG_AUTHORS")
    //  2. "Homepage: " + env!("CARGO_PKG_HOMEPAGE")
    //  3. "Usage: bitsymd <somefile>.md"
    //
    // 1.
    let mut the_author = String::from("Written by: ");
    the_author.push_str(env!("CARGO_PKG_AUTHORS"));

    // 2.
    let mut the_homepage = String::from("Homepage: ");
    the_homepage.push_str(env!("CARGO_PKG_HOMEPAGE"));

    // 3.
    //let the_usage = "Usage: bitsymd <filename>.md"

    // Print out
    print_short_banner();
    println!("{}", the_author);
    println!("{}", the_homepage);
    println!("Usage: bitsymd <filename>.md");
}

// A far more elegant solution
// fn print_long_banner() {
//     print_short_banner();
//     println!("Written by: {}\nHomepage: {}\n Usage: bitsymd <filename>.md\n",
//         env!("CARGO_PKG_AUTHORS"),
//         env!("CARGO_PKG_HOMEPAGE")
//     );
// }

fn usage() {
    print_long_banner();
}

// Called when markdown file is passed via cli
fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage();
        }
    }

}

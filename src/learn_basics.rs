// specify return values with ->
// -> u16 tells a function to return a 16-bit unsigned integer
/*fn get_version() -> u16 {
    1000
}*/

//fn test_immutability() {
//     let the_version = "0.1";
//     the_version = "0.2";         // breaks our function!
//     println!("The version: {}", the_version);
//
//  Since all variables are inherently immutable, you can't reassign them!
//  So compiling this code returns the following error:
//  =>  error[E0384]: cannot assign twice to immutable variable `the_version`
//
//  Fix this by adding the mut keyword
//    let mut the_version = "0.1";
//  NOTE: Running this will return a warning, but still compile
//  The warning: "value assigned to `the_version` is never read"
//  letting us know that this line is kinda superfluous
//    the_version = "0.2";
//    println!("The version: {}", the_version);
//}

fn usage() {
    // Declare and assign variable; variable type is &str (string slice)
    // let the_version: &str = "0.1";

    // Actually: since Rust knows "0.1" is a string literal and all
    // string literals become static string slices when compiled we
    // can just omit &str. Rust will infer that the var needs to be &str
    // let the_version = "0.1";

    // But why not use the version listed in the manifest?
    // To call environment variables, Rust has a macro for that: env!()
    // string keys we can call:
    //      CARGO_PKG_VERSION
    //      CARGO_PKG_AUTHORS
    //      CARGO_PKG_NAME
    //      CARGO_PKG_DESCRIPTION
    //      CARGO_PKG_HOMEPAGE
    let the_version = env!("CARGO_PKG_VERSION");

    println!("mdcompiler is a markdown compiler written by kyrose");
    println!("Version {}", the_version); // Print value assigned
}

fn main() {
    //test_immutability();
    usage();
}

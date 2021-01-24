// specify return values with ->
// -> u16 tells a function to return a 16-bit unsigned integer
/*fn get_version() -> u16 {
    1000
}*/

fn usage() {
    // Declare and assign variable; variable type is &str (string slice)
    let the_version: &str = "0.1";
    println!("mdcompiler is a markdown compiler written by kyrose");
    println!("Version {}", the_version); // Print value assigned
}

fn main() {
    usage();
}

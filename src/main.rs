use std::env;
mod help;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let mode = &(argv[0].to_lowercase());
    let mode_zip = String::from("zip");
    let mode_unzip = String::from("unzip");
    if !(mode_zip.eq(mode) || mode_unzip.eq(mode)) {
        help::about();
        println!("invalid mode");
        std::process::exit(1);
    } else {
        println!("running in {} mode", mode);
        if mode_zip.eq(mode) {
        } else {
            // unzip
        }
    }
    if argc < 2 {
        help::about();
        println!("Expected at least one argument");
        std::process::exit(1);
    }
    println!("{:?}", argv)
}

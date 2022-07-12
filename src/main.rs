use std::env;

fn main() {
     let argv: Vec<String> = env::args().collect();
     let argc = argv.len();
     if !(String::from("zip").eq_ignore_ascii_case(&argv[0]) || String::from("unzip").eq_ignore_ascii_case(&argv[0])) {
         println!("invalid mode");
     } else {
         println!("running in {} mode", &argv[0]);
     }
     if argc < 2 {
         println!("Zipshim. Copyright 2022 GlassROM and randomhydrosol. Licensed under WTFPL\nExpected at least one argument");
             std::process::exit(1);
     }
}

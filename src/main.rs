use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let mut args = env::args();
    let _ = args.next();

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: dump-syntax path/to/filename.rs");
            process::exit(1);
        }
    };

    let mut file = File::open(&filename).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).unwrap();

    let syntax = syn::parse_file(&src).expect("Unable to parse file");


    println!("{:#?}", syntax);
}

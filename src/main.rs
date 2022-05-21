extern crate elf;


use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path = PathBuf::from(&args[1]);
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };



    let text_section = match file.get_section(".text") {
        Some(s) => s,
        None => panic!("Failed to look up .text section"),
    };
    // println!("{:?}", text_section.data);

    let symbols = match file.get_symbols(text_section) {
        Ok(s) => s,
        Err(_) => panic!("Failed to get symbols"),
    };
    println!("{}",symbols)

    for s in symbols.iter() {
        println!("{}", s)
    }

}


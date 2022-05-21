extern crate elf;
use rustc_demangle::demangle;

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



    let text_section = match file.get_section(".symtab") {
        Some(t) => t,
        None => panic!("Failed to look up section"),
    };
    // println!("{:?}", text_section.data);

    let symbols = match file.get_symbols(text_section) {
        Ok(s) => s,
        Err(_) => panic!("Failed to get symbols"),
    };

    //https://github.com/cole14/rust-elf/blob/master/src/lib.rs#L280
    //     symbols.push(types::Symbol {
    //             name:    try!(utils::get_string(link, name as usize)),
    //             value:   value,
    //             size:    size,
    //             shndx:   shndx,
    //             symtype: types::SymbolType(info[0] & 0xf),
    //             bind:    types::SymbolBind(info[0] >> 4),
    //             vis:     types::SymbolVis(other[0] & 0x3),
    //         });
    //     Ok(())
    // }

    println!("{} symbols found", symbols.len());
    for s in symbols.iter() {
        println!("{}", demangle(&s.name).to_string());

    }

}


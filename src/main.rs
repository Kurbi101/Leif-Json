mod parser;
mod value;

use crate::parser::Parser;
use memmap2::Mmap;
use std::{fs, fs::File, str};

fn main() -> std::io::Result<()> {
    let file = File::open("test.json")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let content = match str::from_utf8(&mmap[..]) {
        Ok(res) => res,
        Err(e) => panic!("File isnt valid utf-8: {}", e),
    };

    let mut parser = Parser::new(content);
    let val = match parser.parse_value() {
        Ok(s) => s,
        Err(e) => panic!("PARSING FAILED: \n {}", e),
    };

    let _ = fs::write("result.json", val.stringify());

    Ok(())
}

extern crate umya_spreadsheet;
use std::path::{Path, PathBuf};
use clap::Parser;
mod circuit;
use circuit::{is_test_sheet, is_version_zero};

// Note umya-spreadsheet rather than calamine as we want random access

fn read_eawr(path: &Path) {
    println!("{:?}", path);
    let workbook = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    for i in 0..(workbook.sheet_count() - 1) {
        let sheet = workbook.sheet(i).unwrap();
        println!("Sheet {}, {}", sheet.name(), is_test_sheet(sheet) && is_version_zero(sheet));
    }
}

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: PathBuf,
}


fn main() {
    let args = Cli::parse();
    let src = &args.path;
    read_eawr(src);
}   

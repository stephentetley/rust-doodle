//    Copyright 2026 Stephen Tetley
//
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

extern crate umya_spreadsheet;
use std::path::{Path, PathBuf};
use clap::Parser;
mod circuit;
mod general_checklist;
use circuit::{is_test_sheet, is_version_zero, read_circuit};
use general_checklist::{get_checklist_data};

// Note umya-spreadsheet rather than calamine as we want random access

fn read_eawr(path: &Path) {
    println!("{:?}", path);
    let workbook = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    let date_string = get_checklist_data(&workbook);
    for i in 0..(workbook.sheet_count() - 1) {
        let sheet = workbook.sheet(i).unwrap();
        if is_test_sheet(sheet) && ! is_version_zero(sheet) {
            let file_name = path.file_name().and_then(|ss| ss.to_str()).unwrap_or("");
            let json = read_circuit(file_name, &date_string.as_str(), sheet);
            println!("{}", json);
        }
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

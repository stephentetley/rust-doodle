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
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use clap::Parser;
use serde::ser::{SerializeSeq, Serializer};
mod circuit;
mod motor;
mod general_checklist;
use circuit::{is_test_sheet, is_version_zero, process_circuits};
use motor::{is_motor_sheet, process_motors};
use general_checklist::{get_checklist_data};
// use serde_json::{Serializer};

// Note use umya-spreadsheet rather than calamine because we want random access

fn read_circuit_sheets(xlsx_path: &Path) {
    if let Ok(workbook) = umya_spreadsheet::reader::xlsx::read(xlsx_path) {
        let file_name = xlsx_path.file_name().and_then(|ss| ss.to_str()).unwrap_or("Unknown");
        let out_path = xlsx_path.with_extension("circuits.json");
        if let Ok(out) = std::fs::File::create(&out_path) {
            let mut writer = BufWriter::new(out);
            let mut serializer = serde_json::Serializer::new(&mut writer);
            if let Ok(mut seq) = serializer.serialize_seq(None) {

                let date_string = get_checklist_data(&workbook);
                for i in 0..(workbook.sheet_count()) {
                    let sheet = workbook.sheet(i).unwrap();
                    if is_test_sheet(sheet) && ! is_version_zero(sheet) {
                        let _ = process_circuits(&mut seq, file_name, &date_string.as_str(), sheet);
                    }
                }
                seq.end().unwrap();
            }
            writer.flush().unwrap();
        } else { println!("Could not create output file {:?}", &out_path) }
    } else { println!("Could not read {:?}", xlsx_path) }
}

fn read_motor_sheets(xlsx_path: &Path) {
    if let Ok(workbook) = umya_spreadsheet::reader::xlsx::read(xlsx_path) {
        let file_name = xlsx_path.file_name().and_then(|ss| ss.to_str()).unwrap_or("Unknown");
        let out_path = xlsx_path.with_extension("motors.json");
        if let Ok(out) = std::fs::File::create(&out_path) {
            let mut writer = BufWriter::new(out);
            let mut serializer = serde_json::Serializer::new(&mut writer);
            if let Ok(mut seq) = serializer.serialize_seq(None) {

                let date_string = get_checklist_data(&workbook);
                for i in 0..(workbook.sheet_count()) {
                    let sheet = workbook.sheet(i).unwrap();
                    if is_motor_sheet(sheet) {
                        let _ = process_motors(&mut seq, file_name, &date_string.as_str(), sheet);
                    }
                }
                seq.end().unwrap();
            }
            writer.flush().unwrap();
        } else { println!("Could not create output file {:?}", &out_path) }
    } else { println!("Could not read {:?}", xlsx_path) }
}


#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: PathBuf,
}


fn main() {
    let args = Cli::parse();
    let src = &args.path;
    read_circuit_sheets(src);
    read_motor_sheets(src);
}   

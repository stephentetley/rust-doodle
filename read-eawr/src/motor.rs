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


use serde_json::json;
use serde_json::Value;
use serde::ser::{SerializeSeq};
use umya_spreadsheet::*;


// https://docs.rs/serde_json/1.0.150/serde_json/#constructing-json-values


pub fn is_motor_sheet(sheet: &Worksheet) -> bool {
    sheet.value("C1") == "MOTOR CHECKLIST"
}


pub fn process_motors<S: SerializeSeq>(seq: &mut S, file_name: &str, date_string: &str, sheet: &Worksheet) -> Result<(), String> {
    // 'D'..'G'
    for col in 4..8 {
        if let Some(json) = read_motor1(col, file_name, date_string, sheet) {
            seq.serialize_element(&json).unwrap();
        }
    }
    Ok(())
}


fn read_motor1(col: u32, file_name: &str, date_string: &str, sheet: &Worksheet) -> Option<Value> {
    let motor_name = sheet.value((col, 11)).trim().to_string();
    let date_string2 = sheet.value("C9").trim().to_string();
    if motor_name != "" { 
        Some(json!({
                "file_name": file_name,
                "sheet_name": sheet.name(),
                // header
                "aib_reference": sheet.value("C2").trim().to_string(),
                "site": sheet.value("C3").trim().to_string(),
                "location": sheet.value("C4").trim().to_string(),
                "section": &sheet.value("C5").trim().to_string(),
                "date_string": if date_string2 != "" {date_string2.as_str()} else {date_string},
                // motor - columns D(4) to G(7)
                "column": col,
                "motor_name": motor_name,
                "initial_inspection": sheet.value((col, 11)).trim().to_string(),
                "periodic_inspection": sheet.value((col, 12)).trim().to_string(),
                "has_name_or_data_plate": sheet.value((col, 20)).trim().to_string(),
            }))
        } else { None }
}


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
use umya_spreadsheet::*;


// https://docs.rs/serde_json/1.0.150/serde_json/#constructing-json-values


pub fn is_test_sheet(sheet: &Worksheet) -> bool {
    sheet.value("F1") == "TEST SHEET"
}

pub fn is_version_zero(sheet: &Worksheet) -> bool {
    sheet.value("B2") == "Site Name:" && sheet.value("B5") == "CIRCUIT/CABLE DETAILS"
}

pub fn read_circuit(file_name: &str, date_string: &str, sheet: &Worksheet) -> serde_json::Value {
    let has_tp_or_sp = sheet.value("E4") == "TP / SP";
    let location_cell = if has_tp_or_sp {"F5".to_string()} else {"E5".to_string()};
    json!({
        "file_name": file_name,
        "date_string": date_string,
        "sheet_name": sheet.name(),
        // header
        "site_name": sheet.value("B3"),
        "db_or_panel_number": sheet.value("E3"),
        "test_date": sheet.value("J3"),
        "sheet_number": sheet.value("K3"),
        "ai_ref": sheet.value("B5"),
        "tp_or_sp": if has_tp_or_sp {sheet.value("E5")} else {"".to_string()},
        "location": sheet.value(location_cell),
        "db_or_panel_incomer_details": sheet.value("B7"),
    })
}

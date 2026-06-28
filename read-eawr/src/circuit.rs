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


pub fn is_test_sheet(sheet: &Worksheet) -> bool {
    sheet.value("F1").trim() == "TEST SHEET"
}

pub fn is_version_zero(sheet: &Worksheet) -> bool {
    sheet.value("B2").trim() == "Site Name:" 
        && sheet.value("B5").trim() == "CIRCUIT/CABLE DETAILS"
}

pub fn process_circuits<S: SerializeSeq>(seq: &mut S, file_name: &str, date_string: &str, sheet: &Worksheet) -> Result<(), String> {
    for col in 3..12 {
        if let Some(json) = read_circuit1(col, file_name, date_string, sheet) {
            seq.serialize_element(&json).unwrap();
        }
    }
    Ok(())
}


fn read_circuit1(col: u32, file_name: &str, date_string: &str, sheet: &Worksheet) -> Option<Value> {
    let has_tp_or_sp = sheet.value("E4").trim() == "TP / SP";
    let location_cell = if has_tp_or_sp {"F5".to_string()} else {"E5".to_string()};

    let cable_num = sheet.value((col, 10)).trim().to_string();
    let fed_from = sheet.value((col, 11)).trim().to_string();
    let circuit_ref_and_phase = sheet.value((col, 12)).trim().to_string();
    if cable_num != "" || fed_from != "" || circuit_ref_and_phase != "" { 
        Some(json!({
                "file_name": file_name,
                "date_string": date_string,
                "sheet_name": sheet.name(),
                // header
                "site_name": sheet.value("B3").trim(),
                "db_or_panel_number": sheet.value("E3").trim(),
                "test_date": sheet.value("J3").trim(),
                "sheet_number": sheet.value("K3").trim(),
                "ai_ref": sheet.value("B5").trim(),
                "tp_or_sp": if has_tp_or_sp {sheet.value("E5").trim().to_string()} else {"".to_string()},
                "location": sheet.value(location_cell).trim(),
                "db_or_panel_incomer_details": sheet.value("B7").trim(),
                // circuit - columns C(3)to K(11)
                "column": col,
                "cable_num": cable_num,
                "fed_from": fed_from,
                "circuit_ref_and_phase": circuit_ref_and_phase,
                "circuit_description": sheet.value((col, 13)).trim().to_string(),
                "circuit_type": sheet.value((col, 14)).trim().to_string(),
                "cable_type": sheet.value((col, 15)).trim().to_string(),
                "installation_method": sheet.value((col, 16)).trim().to_string(), 
                "cable_length": sheet.value((col, 17)).trim().to_string(),
                "num_of_cores_csa": sheet.value((col, 18)).trim().to_string(),
                "circuit_breaker_or_fuse_rating": sheet.value((col, 26)).trim().to_string(),
                "circuit_breaker_bs_and_type_num": sheet.value((col, 27)).trim().to_string(),
                "circuit_breaker_manufacturer_and_ref_num": sheet.value((col, 28)).trim().to_string(),
                "rcd_manufacturer_and_type": sheet.value((col, 31)).trim().to_string(),
                "load": sheet.value((col, 34)).trim().to_string(),
                "rating_kw": sheet.value((col, 35)).trim().to_string(),
                "full_load_current_a": sheet.value((col, 36)).trim().to_string(),
                "circuit_voltage_v": sheet.value((col, 58)).trim().to_string(),
                "circuit_current_a": sheet.value((col, 59)).trim().to_string(),
                "test_date": sheet.value((col, 60)).trim().to_string(),
                "comments": sheet.value((col, 61)).trim().to_string(),
            }))
        } else { None }
}



use serde_json::json;
use umya_spreadsheet::*;


// https://docs.rs/serde_json/1.0.150/serde_json/#constructing-json-values

// #[derive(Serialize, Deserialize)]
// pub struct _AsFittedCircuit {
//     pub file_name: String,
//     pub tab_name: String
// }

pub fn is_test_sheet(sheet: &Worksheet) -> bool {
    sheet.value("F1") == "TEST SHEET"
}

pub fn is_version_zero(sheet: &Worksheet) -> bool {
    sheet.value("B2") == "Site Name:" && sheet.value("B5") == "CIRCUIT/CABLE DETAILS"
}

pub fn read_circuit(file_name: &str, sheet: &Worksheet) -> serde_json::Value {
    json!({
        "file_name": file_name,
        "sheet_name": sheet.name(),
    })
}

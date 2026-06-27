

use umya_spreadsheet::*;




pub struct _AsFittedCircuit {
    pub file_name: String,
    pub tab_name: String
}

pub fn is_test_sheet(sheet: &Worksheet) -> bool {
    sheet.value("F1") == "TEST SHEET"
}

pub fn is_version_zero(sheet: &Worksheet) -> bool {
    sheet.value("B2") == "Site Name:" && sheet.value("B5") == "CIRCUIT/CABLE DETAILS"
}
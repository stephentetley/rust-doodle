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

use umya_spreadsheet::*;


// `date_str` has no checking and can be pretty much anything in the wild...
// This is not the place to try and make it a date, so just return a string.
// Do we need any other data?
pub fn get_checklist_data(workbook: &Workbook) -> String {
    if let Ok(sheet) = workbook.sheet_by_name("General Checklist") {
        let date_str: String = sheet.value("B8").trim().to_string();
        date_str
    } else {
        return "".to_string()
    }  
}


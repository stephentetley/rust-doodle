

use umya_spreadsheet::*;


// `date_str` has no checking and can be pretty much anything in the wild
// so this is not the place to try and make it a date
// Do we need any other data?
pub fn get_checklist_data(workbook: &Workbook) -> String {
    if let Ok(sheet) = workbook.sheet_by_name("General Checklist") {
        let date_str = sheet.value("B8");
        println!("{}", date_str);
        return date_str
    } else {
        return "".to_string()
    }  
}


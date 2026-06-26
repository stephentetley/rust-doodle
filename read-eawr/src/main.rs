extern crate umya_spreadsheet;
mod circuit;
use std::path::Path;


// Note umya-spreadsheet rather than calamine as we want random access

fn read_eawr(path: &Path) {
    println!("{:?}", path);
    let workbook = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    for i in 0..(workbook.sheet_count() - 1) {
        let sheet = workbook.sheet(i).unwrap();
        println!("Sheet {},", sheet.name());
    }
}


fn main() {
    let src = Path::new("/home/stephen/_working/coding/rust/data/As_Fitted1.xlsx");
    read_eawr(src);
}

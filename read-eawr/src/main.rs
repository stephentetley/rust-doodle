use std::path::Path;
use calamine::{Reader, Xlsx, open_workbook};


fn read_eawr(path: &Path) {
    println!("{:?}", path);
    let workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    workbook.sheet_names().iter()
        .for_each(|name| {
            println!("Sheet {},", name);
        });
}


fn main() {
    let src = Path::new("/home/stephen/_working/coding/rust/data/As_Fitted1.xlsx");
    read_eawr(src);
}

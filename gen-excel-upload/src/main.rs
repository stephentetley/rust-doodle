extern crate umya_spreadsheet;
use std::fs;
use std::io;
use std::path::Path;


fn modifies(path: &Path) -> io::Result<bool>{
    // fs::exists(path)?;
    let mut wb = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    let ws = wb.sheet_by_name_mut("Change Request Header").unwrap();
    ws.cell_mut("B6").set_value("Upload1 Batch 1of1");
    let _ = umya_spreadsheet::writer::xlsx::write(&wb, path);
    Ok(true)
}


fn main() {

    let src = Path::new("/home/stephen/_working/coding/rust/_temp/uploader_floc_template.xlsx");
    let dest = Path::new("/home/stephen/_working/coding/rust/_temp/upload1.xlsx");
    match fs::exists(src) {
        Ok(true)  => {
            println!("uploader_floc_template.xlsx exists");
            let _ = fs::copy(src, dest);
            let _ = modifies(dest);
            ()
        },
        Ok(false) => println!("uploader_floc_template.xlsx missing"),
        _ => ()
    };

    
    println!("Done.");
}

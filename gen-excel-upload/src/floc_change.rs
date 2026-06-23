extern crate umya_spreadsheet;
use std::path::Path;
use std::io;

pub fn modifies(path: &Path) -> io::Result<bool>{
    // fs::exists(path)?;
    let mut wb = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    let ws = wb.sheet_by_name_mut("Change Request Header").unwrap();
    ws.cell_mut("B6").set_value("Upload1 Batch 1of1");
    let _ = umya_spreadsheet::writer::xlsx::write(&wb, path);
    Ok(true)
}

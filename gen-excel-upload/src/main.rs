
use std::fs;
use std::path::Path;
mod floc_change;



fn main() {

    let src = Path::new("/home/stephen/_working/coding/rust/data/uploader_floc_template.xlsx");
    let dest = Path::new("/home/stephen/_working/coding/rust/data/upload1.xlsx");
    match fs::exists(src) {
        Ok(true)  => {
            println!("uploader_floc_template.xlsx exists");
            let _ = fs::copy(src, dest);
            let _ = floc_change::modifies(dest);
            ()
        },
        Ok(false) => println!("uploader_floc_template.xlsx missing"),
        _ => ()
    };

    
    println!("Done.");
}

use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {


let  file = OpenOptions :: new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(path);

match file {
    Ok(mut file)=> {
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");
        
    }
    Err(_)=> {
        panic!();
    }
}


}
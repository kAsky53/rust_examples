use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    // get all files in files directory
    let paths = fs::read_dir("./files").unwrap();

    // target extension
    let target_extension = "txt";

    // loop all files in files directory
    for path in paths {
        // get path of file
        let file_path = path.unwrap().path();

        // get file name
        let dir_name = file_path.display().to_string();

        // get extension of file
        let extension = get_extension_from_filename(&dir_name);

        if target_extension == extension.unwrap() {
            // open file
            let file = BufReader::new(fs::File::open(file_path).expect("Unable to open file"));

            // set lines of open file
            let mut cnt = 0;

            // get total lines of open file
            for _ in file.lines() {
                cnt = cnt + 1;
            }

            println!("File name: {} \n {} lines\n", dir_name, cnt);
        }
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

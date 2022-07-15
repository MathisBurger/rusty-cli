use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use platform_dirs::AppDirs;

pub struct FileReader {
    /// The directory all application files are stored in
    platform_dir: PathBuf
}

impl FileReader {

    /// Creates a new file-reader for reading files from
    /// the app dir with the provided name
    pub fn new(app_folder: String) -> Self {
        FileReader {platform_dir: AppDirs::new(Some(app_folder.as_str()), false).unwrap().data_dir}
    }

    /// Reads a string from a file contained in the app directory
    /// of the cli application
    pub fn read_file_to_string(&mut self, file_name: String) -> String {
        let path = PathBuf::as_path(&self.platform_dir);
        if !Path::exists(&path) {
            fs::create_dir(path.clone()).expect("Cannot create root data directory");
        }
        let str_path = path.to_str().unwrap().to_owned() + file_name.as_str();
        let raw_file = File::open(&str_path);
        let mut file = match raw_file {
            Ok(f) => f,
            Err(e) => {
                let mut f = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .read(true)
                    .truncate(true)
                    .open(str_path)
                    .unwrap();
                f.write_all("".as_bytes()).unwrap();
                f
            }
        };
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed reading config");
        if data.is_empty() {
            return String::new();
        }
        data
    }

    /// Writes string content to the file with the provided
    /// file name and closes it after the write process
    pub fn write_string_to_file(&mut self, file_name: String, content: String) {
        let path = PathBuf::as_path(&self.platform_dir);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.to_str().unwrap().to_owned() + file_name.as_str())
            .unwrap();
        file.write_all(content.as_str().as_ref())
            .expect("Cannot write data");
    }
}
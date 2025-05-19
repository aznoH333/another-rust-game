use std::{fs, path::{Path, PathBuf}};

pub struct FileUtils{

}


impl FileUtils {
    pub fn get_files_in_folder(path: &Path, extension: &String) -> Vec<PathBuf>{
        let contents = fs::read_dir(path).unwrap();

        let mut output = Vec::<PathBuf>::new();

        for file in contents {
            let f = file.unwrap();
            
            if f.file_type().unwrap().is_dir() {
                // is folder
                let mut dir_contents = Self::get_files_in_folder(f.path().as_path(), extension);

                output.append(&mut dir_contents);
            }else if f.path().extension().unwrap().to_str().unwrap() == extension{
                // is file
                output.push(f.path());
            }
        }

        return output;
    }
}
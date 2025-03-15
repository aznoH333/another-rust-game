use std::{fs, path::{Path, PathBuf}, string};

pub fn get_files_in_folder(path: &Path) -> Vec<PathBuf>{
    let contents = fs::read_dir(path);

    let output = Vec::<PathBuf>::new();


    output.push(PathBuf::new());

    return output;
}
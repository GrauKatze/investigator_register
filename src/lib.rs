#![warn(missing_docs)]

//! Help to save work file

use std::{ path::Path};

/// reg file in DB
pub fn take_file(file_path: String) -> Result<(), String> {
    if Path::exists(Path::new(&file_path)) {
        if is_file_reg(&file_path){
            Ok(())
        }else {
            Err(format!("Can't reg file, path: \"{}\"", file_path))
        }
    } else {
        Err(format!("path is not valid, path: \"{}\"", file_path))
    }
}

fn is_file_reg(_file_path: &String)->bool{
    true
}
/// give file from DB
pub fn give_file(_file_path: String) -> Result<(), String> {
    Ok(())
}

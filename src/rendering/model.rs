use std::{path::PathBuf, fs::read};

// Imports for FBX Rendering
// use super::Render;

pub struct FBXFile {
    content: String,
}

impl FBXFile {
    pub fn new(content: impl Into<String>) -> FBXFile {
        FBXFile { content: content.into() }
    }

    pub fn load_from_file(path: impl Into<PathBuf>) -> std::io::Result<FBXFile> {
        let path: PathBuf = path.into();

        let content = read(path)?.into_iter().map(|b| b as char).collect::<String>();
        Ok(FBXFile::new(content))
    }
}

pub struct Model {
    file: FBXFile,
    name: String,
    


}

// impl Render for Model {
//     fn render(&self, raylib: &mut raylib::RaylibHandle) {
        
//     }
// }

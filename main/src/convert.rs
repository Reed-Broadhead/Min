//use image::*; // Using image crate: https://github.com/image-rs/image
use webp::*; // Using webp crate: https://github.com/jaredforth/webp
use std::path::Path;
use std::io::Cursor;
use image::ImageReader;
use std::fs;

pub struct ImgFile {
    pub name: String,
    pub format: String,
    pub path: String,
    pub replace: bool,
    pub quality: f32,
}
impl ImgFile {
    pub fn to_webp(&self){

        println!("Converting to PNG {:?}", &self.path);

        let img = ImageReader::open(&self.path).unwrap().decode().unwrap();

        let output_path = Path::new("").join(&self.name).with_extension("webp");
        img.save(output_path).unwrap();
        if self.replace {
            if let Err(e) = fs::remove_file(&self.path){
                println!("Failed to remove files.");
            }
        }
    } 
}

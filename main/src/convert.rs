//use image::*; // Using image crate: https://github.com/image-rs/image
use webp::*; // Using webp crate: https://github.com/jaredforth/webp
use std::path::Path;
use std::io::Cursor;
use image::ImageReader;

pub struct ImgFile {
    pub name: String,
    pub format: String,
    pub path: String,
    pub replace: bool,
    pub quality: f32,
}
impl ImgFile {

    pub fn new_to_webp(&self){

        println!("Converting to PNG {:?}", &self.path);

        //let mut reader = ImageReader::new(Cursor::new(raw_data))
        let img = ImageReader::open(&self.path).unwrap().decode().unwrap();

        let output_path = Path::new("").join(&self.name).with_extension("webp");
        img.save(output_path).unwrap();

    } 
}

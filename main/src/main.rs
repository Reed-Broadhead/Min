use clap::{Parser, Subcommand};
use std::fs;
use std::time::Instant;

mod convert;
/// Min is a command line tool designed to convert png and jpg files to webp format.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands{
    Convert {

        /// The name of the file to convert. Do not include the file extension
        #[arg(short, long)]
        file: Option<String>,

        /// The quality of the file. A number between 1 and 10, 10 being the highest quality. 
        #[arg(short, long)]
        quality: Option<f32>,

        /// Replaces the original file.
        #[arg(short, long)]
        replace: bool
    },
}
fn main() {
    let args = Args::parse();

    match &args.commands {
         Some(Commands::Convert {file, quality, replace}) => {
               process_args( file, quality, replace); 
         },
         None => (),
        }
}
fn process_args(file: &Option<String>, quality: &Option<f32>, replace: &bool) {

    let paths = fs::read_dir("./").unwrap();

    let mut operate: bool = false;
    
    for path in paths {
        let new = Instant::now();

        let path = path.unwrap().path().display().to_string();

        let mut place = (0,0);

        for (i, item) in path.chars().rev().enumerate() {
            match item {
                '.' => place.1 = (path.len() - i).try_into().unwrap(),
                '/' => { place.0 = (path.len() - i).try_into().unwrap(); break},
                _ => continue,
            }
        }
        if place.0 == 0 || place.1 == 0 {continue}; 
        let img = convert::ImgFile{
            name: (path[place.0..place.1-1]).to_string(), 
            format: (path[place.1..path.len()]).to_string(),
            path: path,
            replace: *replace, 
            quality: match quality {
                Some(x) => {
                    if x * 10.0 > 100.0 {
                        println!("{x} is not a valid value. \nplease enter a number between 1 and 10"); 
                        operate = true;
                        break
                    } else {x * 10.0}
                 },
                None => 100.0,
            },
        };

        match &file {
            Some(x) => {
                if &img.name != x {continue};
            },
            None => (),
        }
        if vec!["png", "jpg"].contains(&img.format.to_lowercase().as_str()) { 
            img.to_webp();
            operate = true
        } 
        println!("Time elapsed: {:?}", new.elapsed());
    };
    if operate == false && file != &None {
        println!("No files match {:?}", file); 
    } else if operate == false{ 
        println!("No files to operate on");
    } else {
        println!("Operation complete")
    }; 
}

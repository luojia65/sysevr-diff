mod app;
mod cv;
mod parse;
mod format;

use std::fs::{self, File};
use std::path::Path;
use std::io::{Result, Read};

fn main() -> Result<()> {
    let params = app::params();
    
    println!("Using input : {}", params.input);
    let path = Path::new(&params.input);
    
    if path.is_file() {
        let mut file = File::open(path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;
    
        let code_slices = parse::code_slices(&content);
    
        for a in code_slices {
            println!("new code slice!");
            for i in format::add_if_return(a) {
                println!("{}", i.cond);
            }
        }
    } else {
        println!("input is a folder");
        for dirent in fs::read_dir(path)? {
            let dirent = dirent?;
            dbg!(dirent.path());
                
            let mut file = File::open(dirent.path())?;

            let mut content = String::new();
            file.read_to_string(&mut content)?;
        
            let code_slices = parse::code_slices(&content);
        
            for a in code_slices {
                println!("new code slice!");
                for i in format::add_if_return(a) {
                    let cv_list = cv::gen_add_if_return(&i);
                    dbg!(cv_list);
                }
                for i in format::modify_while_for(a) {
                    cv::gen_while_for(&i);
                    dbg!(i);
                }
                for i in format::modify_if(a) {
                    cv::gen_modify_if(&i);
                    dbg!(i);
                }
                for i in format::modify_value_assign(a) {
                    // dbg!(i);
                }
            }
        }
    }
    Ok(())
}



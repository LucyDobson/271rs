use std::*;
use std::fs;
use serde_json::{json, Value};

fn inv() -> Vec<String> {
    let entries = fs::read_dir("./")
        .expect("Failed to read directory");
   
    let mut files: Vec<String> = Vec::new();
   
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
       
        let file_name = path.file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
        if !file_name.starts_with(".") {
            files.push(file_name)
        }
    }
   
    files.sort();
    files
}

fn data() -> Vec<String> {
    
}

fn main() {
    println!();
    
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    println!();

    let input = args[1].trim().to_lowercase();
    
    if input.trim() == "help".to_string(){
        println!("List of commands: \n  . commit \n  . revert");
    }


    else if input.trim() == "commit".to_string(){
        if !fs::exists(".scm").expect("Can't check existence of file does_not_exist.txt") {
            println!("No .scm file found!");
            fs::write(".scm","").expect("Failed to make .scm file");
            
            let file_names: Vec<String> = inv();
            let mut file_data : Vec<String> = Vec::new();
            println!("Files in Directory: ");
            
            for file in file_names {
                println!("{}", file);
                let contents = fs::read_to_string(file)
                    .expect("Should have been able to read the file");
                file_data.push(contents.clone());
               // println!("{}", contents);
            }
        }
        else {
            println!(".scm file found! ");
        }

    }



    else if input.trim() == "revert".to_string(){ 
        println!("You would like to REVERT?");
    }


    else{
       println!("Invalid entry, try again."); 
    }


    println!();
  
    //let paths = fs::read_dir("./").unwrap();
    //println!("Files in directory: ");
    //for path in paths {
        //println!("- {}", path.unwrap().path().display())
   // }
}

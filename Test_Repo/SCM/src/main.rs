use std::*;
use std::fs;
use std::env;
use serde_json::{json, Value};
use std::collections::HashMap;
use diffy::create_patch;

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


fn main() {
    println!();
    let mut latest_stuff: HashMap<String, String> = HashMap::new();
    let file_names: Vec<String> = inv();
    let args: Vec<String> = env::args().collect();
    
    dbg!(&args);
    println!();
    
    if args.len() < 2 {
        println!("Comands: help, commit, revert");
        return;
    }

    let input = args[1].trim().to_lowercase();
    if input == "help".to_string(){
        println!("List of commands: \n  . commit \n  . revert");
    }

    else if input.trim() == "commit".to_string(){
        if !fs::exists(".scm").expect("Can't check existence of file does_not_exist.txt") {
            println!("No .scm file found!");
            fs::write(".scm","").expect("Failed to make .scm file");
          
            println!("Files in Directory: ");
            
            for file in file_names {
                println!("{}", file);
                let contents = fs::read_to_string(&file)
                    .expect("Should have been able to read the file");
                latest_stuff.insert(file, contents);
            }

            let commit_stuff = json!({"init": latest_stuff, "diff": "{}"});
            let json_stuff = json!({"latest": latest_stuff, "commit": commit_stuff});
            let pretty_json: String = serde_json::to_string_pretty(&json_stuff).expect("String is not pretty");
            println!("Pretty json : {}", pretty_json);
            fs::write(".scm", pretty_json).expect("Can not write json")
        }
        else {
            println!(".scm file found! ");
            let scm_content: String = fs::read_to_string(".scm").expect("Can't read file to string");
            println!("scm: {}",scm_content);
            let json_thingy: Value = 
            match fs::read_to_string(".scm") {

                Ok(json_content) => {
                    match serde_json::from_str::<Value>(&json_content) {
                        Ok(parsed) => parsed,
                        Err(e) => {
                            eprintln!("failed to parse:{}", e);
                            return;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read file: {}", e);
                    return;
                }
            };

            //original == scm_content
            //modified == current file
            //file name = "file"
            
            //let patch = create_patch(&scm_content, latest_stuff);
            for file in file_names {
                if(".scm"[file])
                //let modified: String = commit_stuff[file];
               // how do you find out 
                let contents = fs::read_to_string(&file)
                    .expect("Should have been able to read the file");
                latest_stuff.insert(file, contents);
            }
            

            //for each file in directory compare to .scm version
            //diff the two verions
            //append that output into "diff"
        

            // rotate through all files
            // see if there are any not in scm
            // add an x to all files
            // use diffy to get difference and see if it works


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

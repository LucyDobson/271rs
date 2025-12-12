use std::*;
use std::fs;
use std::env;
use serde_json::{json, Value};
use std::collections::HashMap;
use diffy::{create_patch, apply, Patch};

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
    let mut diff_stuff: HashMap<String, String> = HashMap::new();
    let file_names: Vec<String> = inv();
    let args: Vec<String> = env::args().collect();
    let diff_global = "{}";

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
                latest_stuff.insert(file.clone(), contents);
                diff_stuff.insert(file.clone(),"[]".to_string());
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

            for file in file_names {
                let contents = fs::read_to_string(&file)
                    .expect("Should have been able to read the file");
                let old_files = json_thingy["latest"][file.clone()].as_str().unwrap();
               //println!("Old: {}", old_files); 
               //println!("New: {}", contents.clone());
               let mut all_diffs: Vec<String> = Vec::new(); 
               all_diffs.push(json_thingy["commit"]["diff"][file.clone()].to_string());
               println!("LENGTH OF ALL DIFFS \n {}", all_diffs.len());

                let mut applied_diffs = &json_thingy["commit"]["init"][file.clone()].to_string();
                for change in &all_diffs{
                    let patch = Patch::from_str(change.as_str()).unwrap();
               //     let mut temporary =  &serde_json::Value::String(apply(applied_diffs.as_str().unwrap_or_default(), &patch).unwrap());
                    let applied_diffs = apply(applied_diffs.as_str(),&patch).unwrap();
                }
                println!("APPLIED DIFFS PRINT:\n{}", applied_diffs);
                let final_diff = create_patch(applied_diffs.as_str(),contents.as_str());
                all_diffs.push(final_diff.to_string());
                (json_thingy["commit"]["diff"][file]) = all_diffs;// THIS SPOT IS BROKEN
                
                let patch_test = apply(applied_diffs.as_str(),&final_diff);
                println!("-------------------------------\nPatch Test: {}", patch_test.unwrap());
                let mut stuffing = fs::read_to_string(&file).unwrap_or_default();
                println!("This is diff: {}", final_diff);
                stuffing.push_str ("X\n");
                fs::write(file, stuffing).expect("Failed to write file");
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

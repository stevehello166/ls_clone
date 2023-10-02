
use std::{fs,env, path::PathBuf};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    //include options for all and directory
    
    args.push("./".to_string());
    args.push("ndbg".to_string());

    let path = &args[1];
    let mode = &args[2];

    if mode == "dbg" {
        dbg!(&args);
         println!("PATH: {}", path);

    }

    
    print_path(path);
}

fn print_path(path: &String) { 
    let my_path_buf: PathBuf = PathBuf::from(path); 
    if my_path_buf.is_file() {
        println!("Not a directory");
        return();
    }       
    let paths = fs::read_dir(path.clone()).expect("test");

    let mut paths2 = paths        
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        }).collect::<Vec<String>>();

    paths2.sort();
        
    println!(
        "{}", paths2.iter()
        .fold(String::new(),
        |acc, num| acc + &num.to_string()
        + " ")
    );
}



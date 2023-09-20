
use std::{fs,env};

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

    
    print_path(path.to_string());
}

fn print_path(path: String) {         
    let paths = fs::read_dir(path.clone()).unwrap();

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



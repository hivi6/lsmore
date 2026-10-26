use std::env;
use std::fs;
use std::path::Path;

struct Config {
    help: bool,
    paths: Vec<String>,
    depth: Option<i64>,
}

fn help() {
    println!("Usage:");
    println!("    lsmore [FLAGS] <path>");
    println!();
    println!("About:");
    println!("    This is a program that is like ls, but shows files in tree");
    println!();
    println!("FLAGS:");
    println!("    --help          This screen");
    println!("    --depth <size>  Depth of the tree");
    println!();
}

fn parse_arguments(args: &[String]) -> Result<Config, String> {
    let mut help_flag = false;

    let mut paths = Vec::new();

    let mut depth_found = false;
    let mut depth_value = 0;

    let mut iter = args.into_iter().peekable();
    while let Some(arg) = iter.next() {
        let is_flag = arg.starts_with("--");
        if is_flag {
            match arg.as_str() {
                "--help" => help_flag = true,
                "--depth" => {
                    depth_found = true;

                    match iter.next() {
                        Some(next_arg) => match next_arg.parse::<i64>() {
                            Ok(num) => {
                                if num < 0 {
                                    return Err(String::from("depth cannot be negetive"));
                                }
                                depth_value = num
                            }
                            Err(e) => return Err(format!("Failed parsing: {}", e)),
                        },
                        None => return Err(String::from("Expected size for the depth")),
                    }
                }
                _ => return Err(format!("Invalid flag {}", arg)),
            }
        } else {
            paths.push(arg.to_string());
        }
    }

    return Ok(Config {
        help: help_flag,
        paths: paths,
        depth: if depth_found { Some(depth_value) } else { None },
    });
}

fn lsmore_helper(path_str: String, prefix: String, is_last: bool, depth: i64) {
    if depth == 0 {
        return;
    }

    let path = Path::new(&path_str);

    if !path.is_file() && !path.is_dir() {
        println!(
            "{} Doesnot exists or is a special type(like symlink)",
            path_str
        );
        return;
    }

    if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
        println!("{}+- {}", prefix, filename);
    }

    if path.is_file() {
        return;
    }

    let mut file_entries = Vec::new();
    let mut dir_entries = Vec::new();

    match fs::read_dir(&path_str) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        let path = e.path();
                        if let Some(path_str) = path.to_str() {
                            let path_string = path_str.to_string();
                            if path.is_file() {
                                file_entries.push(path_string.clone());
                            }
                            if path.is_dir() {
                                dir_entries.push(path_string.clone());
                            }
                        }
                    }
                    Err(err) => panic!("Error reading entry: {}", err),
                }
            }
        }
        Err(err) => panic!("Error reading directory: {}", err),
    }

    for (i, path) in dir_entries.iter().enumerate() {
        let temp = if is_last { "   " } else { "|  " };
        let is_last = i == dir_entries.len() - 1 && file_entries.is_empty();
        let depth = if depth > 0 { depth - 1 } else { depth };

        lsmore_helper(path.clone(), format!("{}{}", prefix, temp), is_last, depth);
    }

    for (i, path) in file_entries.iter().enumerate() {
        let temp = if is_last { "   " } else { "|  " };
        let is_last = i == file_entries.len() - 1;
        let depth = if depth > 0 { depth - 1 } else { depth };

        lsmore_helper(path.clone(), format!("{}{}", prefix, temp), is_last, depth);
    }
}

fn lsmore(paths: Vec<String>, depth: i64) {
    for path_str in paths.clone().into_iter() {
        let path = Path::new(&path_str);

        if !path.is_file() && !path.is_dir() {
            println!(
                "{} Doesnot exists or is a special type(like symlink)",
                path_str
            );
            return;
        }
    }

    println!("OUTPUT");
    for (i, path_str) in paths.iter().enumerate() {
        let is_last = i == paths.len() - 1;
        lsmore_helper(path_str.to_string(), String::new(), is_last, depth);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match parse_arguments(&args[1..]) {
        Ok(val) => val,
        Err(err) => {
            println!("Error: {}", err);
            help();
            return;
        }
    };

    if config.help {
        help();
        return;
    }

    lsmore(
        config.paths,
        match config.depth {
            Some(val) => val,
            None => -1,
        },
    );
}

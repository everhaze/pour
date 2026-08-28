use std::{
    env::home_dir,
    fmt::Display,
    fs::{DirEntry, File, create_dir_all, read_dir, read_to_string},
    io::{Read, Result},
    path::PathBuf,
    process::exit,
};

const ERRMSG: &str = "Clears the terminal and prints your ASCII art.

Usage: pour [OPTIONS]

Options:
    -a, --available         Shows all available files in .config/pour
    -p, --print <FILENAME>  Prints the desired file
    -r, --random            Picks and prints a random file from .config/pour

Note: Drop your ASCII art in .config/pour";

fn main() {
    let home = home_dir().unwrap_or_else(|| die("invalid home folder"));
    let config_path = home.join(".config").join("pour");
    create_config(&config_path).unwrap_or_else(|e| die(e));

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        println!("{}", ERRMSG);
        exit(0)
    }

    let dir = read_dir(&config_path).unwrap_or_else(|e| die(e));

    match args[1].trim() {
        "-p" | "--print" => {
            if args.len() != 3 {
                die("missing file name");
            }
            let f = args[2].clone();
            let p = config_path.join(f);
            print_file(p).unwrap_or_else(|_| die("file not found"));
        }
        "-a" | "--available" => {
            println!("** Available files **");
            for entry in dir {
                let e = entry.unwrap_or_else(|e| die(e));
                println!("{}", e.file_name().to_string_lossy());
            }
        }
        "-r" | "--random" => {
            let mut files: Vec<DirEntry> = vec![];
            for entry in dir {
                let e = entry.unwrap_or_else(|e| die(e));
                files.push(e);
            }
            if files.is_empty() {
                die(".config/pour is empty")
            }
            let num = random_num(files.len()).unwrap_or_else(|e| die(e));
            let p = files[num].path();
            print_file(p).unwrap_or_else(|_| die("file not found"));
        }
        _ => {
            println!("{}", ERRMSG);
            exit(0)
        }
    }
}

fn random_num(len: usize) -> Result<usize> {
    let mut buf = [0u8; 8];
    File::open("/dev/urandom")?.read_exact(&mut buf)?;
    let result = (u64::from_ne_bytes(buf) % len as u64) as usize;
    Ok(result)
}

fn create_config(config_path: &PathBuf) -> Result<()> {
    create_dir_all(config_path)?;
    Ok(())
}

fn print_file(file: PathBuf) -> Result<()> {
    let f = read_to_string(file)?;
    print!("\x1b[2J\x1b[3J\x1b[H");
    println!("{}", f);
    Ok(())
}

fn die(msg: impl Display) -> ! {
    eprintln!("{}", msg);
    exit(1)
}

use std::io;
use std::fs::{self};
use std::env;
use std::process::{self, Command};
use std::path;

enum PmType {
    Npm,
    Yarn,
    Pnpm,
}

fn get_pm_name(pm_type: PmType) -> String {
    match pm_type {
        PmType::Npm => String::from("npm"),
        PmType::Yarn => String::from("yarn"),
        PmType::Pnpm => String::from("pnpm"),
    }
}

fn detect_pm(filename: String) -> Option<PmType> {
    match filename.as_ref() {
        "yarn.lock" => Some(PmType::Yarn),
        "pnpm-lock.yaml" => Some(PmType::Pnpm),
        "package-lock.json" => Some(PmType::Npm),
        _ => None,
    }
}

fn get_pm_type(dir: fs::ReadDir) -> io::Result<Option<PmType>> {
    for entry in dir {
        let entry = entry?;
        match entry.file_name().to_str() {
            Some(filename) => {
                let pm = detect_pm(String::from(filename));
                match pm {
                    Some(_) => return Ok(pm),
                    None => continue,
                }
            },
            None => continue,
        };
    }
    Ok(None)
}

fn run_pm(pm_name: String, args: Vec<String>, cwd: path::PathBuf) -> process::ExitStatus {
    match which::which(&pm_name) {
        Ok(exec_name) => {
            let mut child = Command::new(exec_name)
                                    .args(args)
                                    .current_dir(cwd)
                                    .spawn()
                                    .expect("Failed to execute process");
            child.wait().expect("Failed to wait for child")
        },
        Err(_) => {
            eprintln!("Couldn't find {} on the path. Exiting.", pm_name);
            process::exit(1);
        }
    }
    
}

fn main() -> io::Result<()> {
    let cwd = env::current_dir()?;
    let dir = cwd.as_path();
    let dir_it = fs::read_dir(dir)?;
    match get_pm_type(dir_it)? {
        Some(pm_type) => {
            let args: Vec<String> = env::args().collect();
            let ecode = run_pm(get_pm_name(pm_type), args[1..].to_vec(), cwd);
            process::exit(ecode.code().expect("Failed to get child exit code"));
        },
        None => {
            eprintln!("Couldn't detect package manager. Exiting.");
            process::exit(1);
        },
    }
}

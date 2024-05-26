use chrono::prelude::*;

fn main() {
    cls()
}

fn run(command: &str, arguments: &[&str]) -> Result<std::process::Output, std::io::Error> {
    std::process::Command::new(command).args(arguments).output()
}

#[cfg(target_os = "windows")]
fn cls() {
    let c = "cls";
    let args = [];

    let output = run(c, &args);

    match output {
        Ok(_output) => {print!("{}",String::from_utf8_lossy(&_output.stdout));}
        Err(_e) => eprintln!("Error: cls did not run properly.")
    }

    println!("last login: {}", Local::now());
}

#[cfg(target_os = "macos")]
fn cls() {
    let c = "clear";
    let args = [];

    let output = run(c, &args);

    match output {
        Ok(_output) => {
            print!("{}",String::from_utf8_lossy(&_output.stdout));
        }
        Err(_e) => eprintln!("Error: cls did not run properly.")
    }
    
    println!("last login: {}", Local::now());
}

#[cfg(target_os = "linux")]
fn cls(){
    let c = "clear";
    let args = [];

    let output = run(c, &args);

    match output {
        Ok(_output) => {print!("{}",String::from_utf8_lossy(&_output.stdout));}
        Err(_e) => eprintln!("Error: cls did not run properly.")
    }
    
    println!("last login: {}", Local::now());
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux"
)))]
fn cls() {
    println!("Os not supported.");
}
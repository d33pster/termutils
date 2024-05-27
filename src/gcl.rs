use colored::*;
use rustypath::RPath;
use std::{fs::{self, File}, io::{Read, Write}, process};

mod analyzer;
use analyzer::Arguments;

fn helper() {
    println!("{} {}", "gcl".blue(), "v1.0".bright_purple().blink());
    println!("Help Text\n");
    println!("{} $ gcl [options] username repo <optional-destination-dir> [flags]\n", "syntax:".bold());
    println!("  | [Options]");
    println!("  | help -> show help and exit.");
    println!("  | version -> show version and exit.\n");
    println!("  | [Flags]");
    println!("  | --priv -> for private repos, add this flag.");
    process::exit(0);
}

fn version() {
    println!("{} {}", "gcl".blue(), "v1.0".bright_purple().blink());
    process::exit(0);
}

use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks, Repository};

fn main() -> Result<(), git2::Error> {
    let mut args = Arguments::new(std::env::args().skip(1).collect());
    args.add("--priv");
    // args.add("--reset-priv");
    
    args.analyze();

    let mut private = false;
    if args.arguments.contains(&"--priv".to_string()) {
        private = true;
    }

    if !private {
        // if --priv flag is not present
        if args.gotlen() == 1 {
            // if only one arg is present
            if args.fetch(1) == "help".to_string() {
                // if help is passed as an argument
                helper();
            } else if args.fetch(1) == "version".to_string() {
                // if version is passed as an argument
                version();
            } else {}
        } else if args.gotlen() >= 2 {
            // if username and repo are the only args
            let username = args.fetch(1);
            let repo = args.fetch(2);

            let url = String::from("https://github.com/") + &username + "/" + &repo + ".git";

            let clonepath:RPath;
            // check for path
            if args.gotlen() == 3 {
                clonepath = RPath::new_from(&args.fetch(3));
            }else {
                clonepath = RPath::pwd().join(&repo);
            }

            let repository = Repository::clone(&url, clonepath.convert_to_pathbuf().as_path())?;
            
            println!("{} ! {}", "@cloned".blue().blink(), RPath::new_from_path(repository.workdir().unwrap_or(repository.path())).convert_to_string());
        } else {}
    } else {
        // if private repository needs to be cloned.

        // get the first value that is not "--priv" as username
        let mut username = Default::default();
        for x in 1..=3 {
            if args.fetch(x) != "--priv".to_string() {
                username = args.fetch(x);
                break;
            }
        }

        // get the remaining arg as repo
        let mut repo = Default::default();
        for x in 1..=3 {
            if args.fetch(x) != "--priv".to_string() && args.fetch(x) != username {
                repo = args.fetch(x);
                break;
            }
        }

        let url = String::from("https://github.com/") + &username + "/" + &repo + ".git";

        // if len is 4 for args, then path is also provided
        let mut _clonepath = RPath::pwd().join(&repo).convert_to_string();
        if args.gotlen() == 4 {
            for x in 1..=3 {
                if args.fetch(x) != repo && args.fetch(x) != "--priv".to_string() && args.fetch(x) != username {
                    _clonepath = args.fetch(x);
                    break;
                }
            }
        }

        // check for ~/.gcl file
        let gclfile = RPath::gethomedir().join(".gcl");
        let mut gcl_existence = false;
        if let Ok(meta) = fs::metadata(gclfile.convert_to_pathbuf().as_path()) {
            if meta.is_file() {
                gcl_existence = true;
            } else {}
        } else {}

        // if exists, then read it to find the github token
        let mut __token = String::new();
        if gcl_existence {
            __token = read_file(gclfile);
        } else {
            println!("@First time {} invoke.", "--priv".bold().yellow());
            println!("{} a GitHub API token (classic) to continue. {}", "@Paste".blue(), "(one-time)".blink());
            std::io::stdin()
                .read_line(&mut __token)
                .expect("Unable to Read __token");

            // save to ~/.gcl
            write_file(gclfile, __token.replace("\n", ""));
        }

        // once token is read, create repo builder.
        let mut rbuilder = RepoBuilder::new();
        let mut cbs = RemoteCallbacks::new();
        let mut opts = FetchOptions::new();

        __token = __token.replace("\n", "");
        cbs.credentials(|_,_,_| {
            let credentials = Cred::userpass_plaintext(&username, &__token);
            Ok(credentials.expect("Credentials mismatch"))
        });

        opts.remote_callbacks(cbs);
        rbuilder.fetch_options(opts);

        let repository = rbuilder.clone(&url, &RPath::new_from(&_clonepath).convert_to_pathbuf().as_path())?;

        println!("{} ! {}", "@cloned".blue().blink(), RPath::new_from_path(repository.workdir().unwrap_or(repository.path())).convert_to_string());
    }

    Ok(())
}

fn read_file(path: RPath) -> String {
    let mut file = File::open(path.convert_to_pathbuf()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read .gcl file.");
    contents
}

fn write_file(path: RPath, content: String) {
    let mut file = File::create(path.convert_to_pathbuf()).unwrap();
    file.write_all(content.as_bytes()).expect("Failed to write to .gcl file");
}
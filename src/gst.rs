use git2::Repository;
use rustypath::RPath;
use colored::*;

fn main() {
    let path = RPath::pwd();

    let repo = match Repository::open(path.convert_to_pathbuf()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open repository: {}", e);
            return;
        }
    };

    // get status
    let statuses = match repo.statuses(None) {
        Ok(statuses) => statuses,
        Err(e) => {
            eprintln!("Failed to get status: {}", e);
            return;
        }
    };

    if statuses.len() == 0 {
        println!("Up to date.");
        std::process::exit(0);
    }

    let mut count = 0;
    // new files
    for entry in statuses.iter() {
        let status = entry.status();
        let epath = entry.path().unwrap_or("unknown");

        if status.is_index_new() {
            println!("{}: {}","New".color(Color::Blue), epath);
            count += 1;
        }
    }

    // modified files
    if count!=0 {
        println!("");
        count = 0;
    }
    for entry in statuses.iter() {
        let status = entry.status();
        let epath = entry.path().unwrap_or("unknown");

        if status.is_index_modified()  || status.is_wt_modified() {
            println!("{}: {}","Modified".color(Color::Cyan), epath);
            count += 1;
        }
    }

    // deleted file
    if count!=0 {
        println!("");
        count = 0;
    }
    for entry in statuses.iter() {
        let status = entry.status();
        let epath = entry.path().unwrap_or("unknown");
        
        if status.is_index_deleted() || status.is_wt_deleted() {
            println!("{}: {}", "Deleted".color(Color::Red), epath);
            count +=1;
        }
    }

    // untracked files
    if count!=0 {
        println!("");
        count = 0;
    }
    for entry in statuses.iter() {
        let status = entry.status();
        let epath = entry.path().unwrap_or("unknown");

        if status.is_wt_new() {
            println!("{}: {}","Untracked".color(Color::Yellow), epath);
            count +=1;
        }
    }

    // conflicted files
    if count!=0 {
        println!("");
        count = 0;
    }
    for entry in statuses.iter() {
        let status = entry.status();
        let epath = entry.path().unwrap_or("unknown");

        if status.is_conflicted() {
            println!("{}: {}","Conflicted".color(Color::Red), epath);
            count += 1;
        }
    }

}
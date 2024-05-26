use git2::Repository;
use rustypath::RPath;
use colorized::*;

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

    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => {
            eprintln!("Failed to fetch head: {}", e);
            return;
        }
    };

    if head.is_branch() {
        match head.shorthand() {
            Some(branchname) => println!("on branch: {}", branchname.color(Colors::GreenFg)),
            None => println!("HEAD is not pointing to any branch."),
        }
    } else {
        println!("HEAD is detached.");
    }

    let mut count_untracked = 0;
    let mut count_deleted = 0;
    let mut count_added = 0;
    let mut count_conflicted = 0;
    let mut count_modified = 0;
    let mut count_renamed = 0;
    let mut count_ignored = 0;

    for entry in statuses.iter() {
        let status = entry.status();

        if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() || status.is_index_renamed() {
            count_added += 1;
        } else if status.is_wt_modified() {
            count_modified += 1;
        } else if status.is_wt_deleted() {
            count_deleted += 1;
        } else if status.is_wt_new() {
            count_untracked += 1;
        } else if status.is_conflicted() {
            count_conflicted += 1;
        } else if status.is_wt_renamed() {
            count_renamed += 1;
        } else if status.is_ignored() {
            count_ignored += 1;
        }
    }

    if count_untracked==0 && count_added==0 && count_deleted==0 && count_modified==0 && count_renamed==0 && count_conflicted==0 {
        println!("{}", "up-to-date.".color(Colors::BlueFg));
        std::process::exit(0);
    }

    // sequence: untracked -> added -> modified -> renamed -> deleted -> conflicted -> ignored.

    // untracked files
    if count_untracked > 0 {
        println!("\n{}", "untracked\\".color(Colors::RedFg));
        for entry in statuses.iter() {
            let status = entry.status();
            let p = entry.path().unwrap_or("{unknown}");
            if status.is_wt_new() {
                println!("    {}", p);
            }
        }
    }

    // added files
    if count_added > 0 {
        if count_untracked > 0 {
            println!("");
        }

        println!("{}", "Added\\".color(Colors::GreenFg));
        for entry in statuses.iter() {
            let status = entry.status();
            let p = entry.path().unwrap_or("{unknown}");
            if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() || status.is_index_renamed() {
                println!("    {}", p);
            }
        }
    }

    // modified files
    if count_modified > 0 {
        if count_untracked > 0 || count_added > 0 {
            println!("");
        }

        println!("{}", "Modified\\".color(Colors::WhiteBg).color(Colors::BlueFg));
        for entry in statuses.iter() {
            let status = entry.status();
            let p = entry.path().unwrap_or("{unknown}");
            if status.is_wt_modified() {
                println!("    {}", p);
            }
        }
    }

    // renamed files
    if count_renamed > 0 {
        if count_untracked > 0 || count_added > 0 || count_modified > 0 {
            println!("");
        }

        println!("{}", "Renamed\\".color(Colors::WhiteBg).color(Colors::YellowFg));
        for entry in statuses.iter() {
            let status = entry.status();
            let p = entry.path().unwrap_or("{unknown}");
            if status.is_wt_renamed() {
                println!("    {}", p);
            }
        }
    }

    // deleted files
    if count_deleted > 0 {
        if count_untracked > 0 || count_added > 0 || count_modified > 0 || count_renamed > 0 {
            println!("");
        }

        println!("{}", "Deleted\\".color(Colors::BlackBg).color(Colors::RedFg));
        for entry in statuses.iter() {
            let status = entry.status();
            let p = entry.path().unwrap_or("{unknown}");
            if status.is_wt_deleted() {
                println!("    {}", p);
            }
        }
    }

    // conflicted files
    if count_conflicted > 0 {
        if count_untracked > 0 || count_added > 0 || count_modified > 0 || count_renamed > 0 || count_deleted > 0 {
            println!("");
        }
        
        println!("{}", "Conflicted\\".color(Colors::BlackBg).color(Colors::WhiteFg));
        for entry in statuses.iter() {
            let status = entry.status();
            let p = entry.path().unwrap_or("{unknown}");
            if status.is_conflicted() {
                println!("    {}", p);
            }
        }
    }

    // ignored files
    if count_ignored > 0 {
        if count_untracked > 0 || count_added > 0 || count_modified > 0 || count_renamed > 0 || count_deleted > 0 || count_conflicted > 0 {
            println!("");
        }

        let mut ignored: Vec<String> = Vec::new();

        for entry in statuses.iter() {
            if entry.status().is_ignored() {
                ignored.push(entry.path().unwrap_or("{unknown}").to_string());
            }
        }
        let mut count = 0;
        let mut ign = ignored[0].clone();
        for entry in &ignored {
            if count == 0 {
                count += 1;
                continue;
            }
            ign = ign + ", " + entry;
        }

        println!("{}", "Ignored\\".color(Colors::BlackBg).color(Colors::GreenFg));
        println!("    {}", ign);
    }

    // if statuses.len() == 0 {
    //     println!("Up to date.");
    //     std::process::exit(0);
    // }

    // let mut count = 0;
    // // new files
    // for entry in statuses.iter() {
    //     let status = entry.status();
    //     let epath = entry.path().unwrap_or("unknown");

    //     if status.is_index_new() {
    //         println!("{}: {}","New".color(Color::Blue), epath);
    //         count += 1;
    //     }
    // }

    // // modified files
    // if count!=0 {
    //     println!("");
    //     count = 0;
    // }
    // for entry in statuses.iter() {
    //     let status = entry.status();
    //     let epath = entry.path().unwrap_or("unknown");

    //     if status.is_index_modified()  || status.is_wt_modified() {
    //         println!("{}: {}","Modified".color(Color::Cyan), epath);
    //         count += 1;
    //     }
    // }

    // // deleted file
    // if count!=0 {
    //     println!("");
    //     count = 0;
    // }
    // for entry in statuses.iter() {
    //     let status = entry.status();
    //     let epath = entry.path().unwrap_or("unknown");
        
    //     if status.is_index_deleted() || status.is_wt_deleted() {
    //         println!("{}: {}", "Deleted".color(Color::Red), epath);
    //         count +=1;
    //     }
    // }

    // // untracked files
    // if count!=0 {
    //     println!("");
    //     count = 0;
    // }
    // for entry in statuses.iter() {
    //     let status = entry.status();
    //     let epath = entry.path().unwrap_or("unknown");

    //     if status.is_wt_new() {
    //         println!("{}: {}","Untracked".color(Color::Yellow), epath);
    //         count +=1;
    //     }
    // }

    // // conflicted files
    // if count!=0 {
    //     println!("");
    //     count = 0;
    // }
    // for entry in statuses.iter() {
    //     let status = entry.status();
    //     let epath = entry.path().unwrap_or("unknown");

    //     if status.is_conflicted() {
    //         println!("{}: {}","Conflicted".color(Color::Red), epath);
    //         count += 1;
    //     }
    // }

}
use std::io::Write;

use argrust::{Arguments, ArgumentDescription, FetchTypes};
use rustypath::RPath;

struct Version{
    name: String,
    version: String,
}

fn main() {
    // version info
    let version = Version{
        name: "termutils".to_string(),
        version: "v0.1.0".to_string(),
    };

    // catch arguments
    let mut args = Arguments::new(std::env::args().skip(1).collect());
    
    // help text
    args.add("help",
    ArgumentDescription::new()
        .name("help")
        .description("show help text")
        .short("-h")
    );

    // license
    args.add("license",
        ArgumentDescription::new()
            .name("license")
            .description("show license")
            .short("-l")
    );

    // version
    args.add(
        "version",
        ArgumentDescription::new()
            .name("version info")
            .description("print version info")
            .short("-v")
    );

    // sub option - create
    args.add(
        "create",
        ArgumentDescription::new()
            .name("create")
            .description("suboption - create")
            .short("-c")
    );

    // parse
    args.analyse();

    // check arguments
    // - mutually exclusive args
    if args.ifarg("help") {
        helper(&version);
        std::process::exit(0);
    } else if args.ifarg("license"){
        license();
        std::process::exit(0);
    } else if args.ifarg("version") {
        version_info(&version);
        std::process::exit(0);
    
    // "create" sub option
    } else if args.ifarg("create") {
        // create new parser for create
        let mut create_args = Arguments::new(args.fetch("create", FetchTypes::TillLast).get());

        // define create arguments

        // callable and init -> macos, linux
        if cfg!(target_os = "macos") || cfg!(target_os="linux") {
            create_args.add(
                "callable",
                ArgumentDescription::new()
                    .name("callable")
                    .description("create callable command")
                    .short("-cal")
            );

            create_args.add(
                "init",
                ArgumentDescription::new()
                    .name("initialise")
                    .description("initialise callable commands")
                    .short("-i")
            );

            create_args.add("system", ArgumentDescription::new().short("-sys"));
        }

        // other general args

        create_args.analyse();

        // for callable
        // syntax -> term create callable [OPTIONS]
        // [OPTIONS] (macos)
        // 1. init, else
        // 2. <appname>
        // [OPTIONS] (linux)
        // 1. init


        if cfg!(target_os = "macos") || cfg!(target_os="linux") {
            if create_args.ifarg("init") && create_args.ifarg("callable") {
                // if init, initialise the command file
                // and add entry in the $SHELL.basename_rc file
                let mut shell_file = ".".to_string();
                if let Ok(shell) = std::env::var("SHELL") {
                    let shell_path = std::path::Path::new(&shell);

                    // get basename
                    if let Some(basename) = shell_path.file_name() {
                        if let Some(basename_str) = basename.to_str() {
                            shell_file = shell_file + basename_str + "rc";
                        } else {
                            eprintln!("Error: Could not resolve shell");
                            print!("Enter shell: ");
                            std::io::stdout().flush().unwrap_or_else(|error| {
                                eprintln!("Failed to flush: {}", error);
                                std::process::exit(0);
                            });
                            std::io::stdin().read_line(&mut shell_file).unwrap_or_else(|error| {
                                eprintln!("Failed to read shell: {}", error);
                                std::process::exit(1);
                            });

                            shell_file = shell_file.trim().to_string();
                            shell_file = ".".to_owned() + &shell_file + "rc";
                        }
                    }
                }

                // add to home dir -> home/shell_file
                shell_file = RPath::gethomedir().join(&shell_file).convert_to_string();

                let mut shellfile = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&shell_file)
                    .unwrap_or_else(|error| {
                        eprintln!("Failed to open {} file: {}", &shell_file, error);
                        std::process::exit(1);
                    });
                
                let init_content = "\nsource ".to_owned() + &RPath::gethomedir().join(".termutils").convert_to_string() + "\n";

                shellfile.write_all(init_content.as_bytes())
                    .unwrap_or_else(|error| {
                        eprintln!("Failed to append to {}: {}", &shell_file, error);
                        std::process::exit(0);
                    });

                let mut commandfile = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(RPath::gethomedir().join(".termutils").convert_to_string())
                    .unwrap_or_else(|error| {
                        eprintln!("Failed to initialise {}: {}", shell_file, error);
                        std::process::exit(1);
                    });
                
                commandfile.write_all("#!/bin/bash\n\n".as_bytes())
                    .unwrap_or_else(|error| {
                        eprintln!("Failed to initialise {}: {}", shell_file, error);
                        std::process::exit(1);
                    });
                
                let _output = std::process::Command::new("chmod")
                    .arg("a+x")
                    .arg(RPath::gethomedir().join(".termutils").convert_to_string())
                    .output()
                    .unwrap_or_else(|error| {
                        eprintln!("Failed to provide permissions to {}: {}", shell_file, error);
                        println!("run $ chmod a+x ~/.termutils");
                        std::process::exit(0);
                    });
                println!("Initialised termutils for {}", shell_file);
            } else if create_args.ifarg_force("callable")==false && create_args.ifarg_force("init") {
                eprintln!("Did you mean CALLABLE INIT?");
                std::process::exit(1);
            } else if create_args.ifarg_force("callable") && create_args.ifarg_force("init")==false {
                // if only callable is there, look for args in macos
                if cfg!(target_os = "macos") {
                    let mut callable_args = Arguments::new(create_args.fetch("callable", FetchTypes::TillLast).get());
                    callable_args.add("application", ArgumentDescription::new().short("-app"));
                    callable_args.add("system", ArgumentDescription::new().short("-sys"));


                    // check for callable args
                    callable_args.analyse();
                    // syntax -> | term create callable | <appflag> <appname> <nickname> <sysflag> {if macos}
                    
                    if callable_args.ifarg("application"){
                        let values = callable_args.fetch("application", FetchTypes::TillNext).get();
                        // if values.len == 1, then only appname provided
                        // else values.len == 2, then nickname provided.
                        let mut appname = String::new();
                        let mut nickname = String::new();
                        if values.len() == 1 {
                            appname = values[0].clone();
                            // read nick name
                            print!("Enter a nickname for the app: ");
                            std::io::stdout().flush().unwrap_or_else(|error| {
                                eprintln!("Failed to flush: {}", error);
                                std::process::exit(0);
                            });
                            std::io::stdin().read_line(&mut nickname).unwrap_or_else(|error| {
                                eprintln!("Failed to read nickname: {}", error);
                                std::process::exit(1);
                            });

                            nickname = nickname.trim().to_string();

                        } else if values.len() == 2 {
                            appname = values[0].clone();
                            nickname = values[1].clone();
                        }

                        let mut file = std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(RPath::gethomedir().join(".termutils").convert_to_string())
                            .unwrap_or_else(|error| {
                                eprintln!("Failed to append to .termutils file: {}", error);
                                std::process::exit(1);
                            });
                        
                        // fix appname if there is more than one word for bash code -> word1\ word2\ ...
                        let appname_temp = appname.clone();
                        let appname_words: Vec<&str> = appname_temp.split(" ").collect();
                        let _appname_backup = appname.clone();
                        if appname_words.len() > 1 {
                            appname = appname_words[0].to_string();
                            let mut count = 0;
                            for x in appname_words{
                                if count == 0 {
                                    count += 1;
                                    continue;
                                }
                                appname = appname + "\\ " + x;
                            }
                        }

                        // fix nickname if there is more than one word -> word1_word2_...
                        let nickname_temp = nickname.clone();
                        let nickname_words: Vec<&str> = nickname_temp.split(" ").collect();
                        let _nickname_backup = nickname.clone();
                        if nickname_words.len() > 1 {
                            nickname = nickname_words[0].to_string();
                            let mut count = 0;
                            for x in nickname_words {
                                if count == 0 {
                                    count += 1;
                                    continue;
                                }
                                nickname = nickname + "_" + x;
                            }
                        }
                        
                        let mut content = "\nfunction ".to_owned() + &nickname + "() {\n";
                        content = content + "    echo \"Opening " + &nickname + "...\"\n    ";
                        if create_args.ifarg("system") {
                            content = content + "open /System/Applications/" + &appname + ".app\n    ";
                        } else {
                            content = content + "open /Applications/" + &appname + ".app\n    ";
                        }

                        content = content + "echo \"done.\"\n}";

                        file.write_all(content.as_bytes())
                            .unwrap_or_else(|error| {
                                eprintln!("Failed to append to .termutils file: {}", error);
                                std::process::exit(1);
                            });
                        
                        println!("Created Trigger for {} as {}", appname.to_uppercase().replace("\\", ""), nickname.to_uppercase());
                        println!("Restart terminal!");
                    } else {
                        // normal callable
                        // ask for name, nickname, description
                        // ask for bash code
                        // create callable.

                        print!("Command name: ");
                        std::io::stdout().flush().unwrap_or_else(|error| {
                            eprintln!("Failed to fetch command name: {}", error);
                            std::process::exit(1);
                        });

                        let mut command_name: String = String::new();
                        std::io::stdin().read_line(&mut command_name).unwrap_or_else(|error| {
                            eprintln!("Failed to fetch command_name: {}", error);
                            std::process::exit(1);
                        });

                        print!("Command nickname: ");
                        std::io::stdout().flush().unwrap_or_else(|error| {
                            eprintln!("Failed to fetch command nickname: {}", error);
                            std::process::exit(1);
                        });

                        let mut command_nickname: String = String::new();
                        std::io::stdin().read_line(&mut command_nickname).unwrap_or_else(|error| {
                            eprintln!("Failed to fetch command_name: {}", error);
                            std::process::exit(1);
                        });

                        print!("Command Description (optional, press ENTER/RETURN for default): ");
                        std::io::stdout().flush().unwrap_or_else(|error| {
                            eprintln!("Failed to fetch command description: {}", error);
                            std::process::exit(1);
                        });

                        let mut _command_desc = String::new();
                        std::io::stdin().read_line(&mut _command_desc).unwrap_or_else(|error| {
                            eprintln!("Failed to fetch command_name: {}", error);
                            std::process::exit(1);
                        });

                        if _command_desc == "".to_string() {
                            _command_desc = "Callable Command".to_string();
                        }

                        // modify nickname
                        let nickname_temp = command_nickname.clone();
                        let nickname_words: Vec<&str> = nickname_temp.split(" ").collect();
                        if nickname_words.len() > 1 {
                            command_nickname = nickname_words[0].to_string();
                            let mut count = 0;
                            for entry in nickname_words {
                                if count == 0 {
                                    count += 1;
                                    continue;
                                }

                                command_nickname = command_nickname + "_" + entry;
                            }
                        }

                        println!("Enter your command body here (shell):");
                        let mut content = String::new();
                        content = content + "\n# command: " + &command_name + "# nickname: " + &command_nickname + "# description: " + &_command_desc + "function " + &command_nickname.replace("\n", "") + "() {\n";
                        let mut count = 1;
                        loop {
                            let mut buffer = String::new();
                            std::io::stdin().read_line(&mut buffer).unwrap_or_else(|error| {
                                eprint!("Failed to print line {}: {}", count, error);
                                std::process::exit(1);
                            });
                            if buffer == "END\n" {
                                break;
                            }
                            content = content + "    " + &buffer;
                            count += 1;
                        }

                        content = content + "}\n";

                        let mut commandfile = std::fs::OpenOptions::new()
                            .append(true)
                            .open(RPath::gethomedir().join(".termutils").convert_to_string())
                            .unwrap_or_else(|error| {
                                eprintln!("Failed to append to {}: {}", RPath::gethomedir().join(".termutils").convert_to_string(), error);
                                std::process::exit(1);
                            });
                        
                        commandfile.write_all(content.as_bytes())
                            .unwrap_or_else(|error| {
                                eprintln!("Failed to write data to {}: {}", RPath::gethomedir().join(".termutils").convert_to_string(), error);
                                std::process::exit(1);
                            });
                        
                        println!("Created command {} -> {}", command_nickname, RPath::gethomedir().join(".termutils").convert_to_string());
                        println!("Restart Terminal!");
                    }
                } else if cfg!(target_os = "linux"){
                    // normal callable
                    // ask for name, nickname, description
                    // ask for bash code
                    // create callable.

                    print!("Command name: ");
                    std::io::stdout().flush().unwrap_or_else(|error| {
                        eprintln!("Failed to fetch command name: {}", error);
                        std::process::exit(1);
                    });

                    let mut command_name: String = String::new();
                    std::io::stdin().read_line(&mut command_name).unwrap_or_else(|error| {
                        eprintln!("Failed to fetch command_name: {}", error);
                        std::process::exit(1);
                    });

                    print!("Command nickname: ");
                    std::io::stdout().flush().unwrap_or_else(|error| {
                        eprintln!("Failed to fetch command nickname: {}", error);
                        std::process::exit(1);
                    });

                    let mut command_nickname: String = String::new();
                    std::io::stdin().read_line(&mut command_nickname).unwrap_or_else(|error| {
                        eprintln!("Failed to fetch command_name: {}", error);
                        std::process::exit(1);
                    });

                    // modify nickname
                    let nickname_temp = command_nickname.clone();
                    let nickname_words: Vec<&str> = nickname_temp.split(" ").collect();
                    if nickname_words.len() > 1 {
                        command_nickname = nickname_words[0].to_string();
                        let mut count = 0;
                        for entry in nickname_words {
                            if count == 0 {
                                count += 1;
                                continue;
                            }

                            command_nickname = command_nickname + "_" + entry;
                        }
                    }

                    print!("Command Description (optional, press ENTER/RETURN for default): ");
                    std::io::stdout().flush().unwrap_or_else(|error| {
                        eprintln!("Failed to fetch command description: {}", error);
                        std::process::exit(1);
                    });

                    let mut _command_desc = String::new();
                    std::io::stdin().read_line(&mut _command_desc).unwrap_or_else(|error| {
                        eprintln!("Failed to fetch command_name: {}", error);
                        std::process::exit(1);
                    });

                    if _command_desc == "".to_string() {
                        _command_desc = "Callable Command".to_string();
                    }

                    println!("Enter your command body here (shell):");
                    let mut content = String::new();
                    content = content + "\n# command: " + &command_name + "# nickname: " + &command_nickname + "# description: " + &_command_desc + "function " + &command_nickname.replace("\n", "") + "() {\n";
                    let mut count = 1;
                    loop {
                        let mut buffer = String::new();
                        std::io::stdin().read_line(&mut buffer).unwrap_or_else(|error| {
                            eprint!("Failed to print line {}: {}", count, error);
                            std::process::exit(1);
                        });
                        if buffer == "END\n" {
                            break;
                        }
                        content = content + "    " + &buffer;
                        count += 1;
                    }

                    content = content + "}\n";

                    let mut commandfile = std::fs::OpenOptions::new()
                        .append(true)
                        .open(RPath::gethomedir().join(".termutils").convert_to_string())
                        .unwrap_or_else(|error| {
                            eprintln!("Failed to append to {}: {}", RPath::gethomedir().join(".termutils").convert_to_string(), error);
                            std::process::exit(1);
                        });
                    
                    commandfile.write_all(content.as_bytes())
                        .unwrap_or_else(|error| {
                            eprintln!("Failed to write data to {}: {}", RPath::gethomedir().join(".termutils").convert_to_string(), error);
                            std::process::exit(1);
                        });
                    
                    println!("Created Command {} -> {}", command_nickname, RPath::gethomedir().join(".termutils").convert_to_string());
                    println!("Restart Terminal");
                }
            }
        }
    }
}

fn helper(version: &Version) {
    println!("{} {}", version.name, version.version);
    println!("HELP");
    println!("  | OPTIONS |");
    println!("      | help or -h     : show this help text and exit");
    println!("      | license or -l  : show license and exit");
    println!("      | version or -v  : show version and exit");
    println!("      | create or -c   : create terminal utilities");
    println!("          | SUBOPTIONS (for create) |");
    if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        println!("               | callable or -cal  : create a callable command for any application");
        println!("                    | SUBOPTIONS (for callable) |");
        println!("                         | init or -i          : initialise callables");
        println!("                         | application or -app : callable command for an application");
        println!("                         | system or sys       : to specify if the app is a system app.");
        println!("                    | SYNTAX |");
        println!("                         $ term create callable init # need to be run before callables can be created.");
        println!("                         $ term create callable -app <appname> <nickname> <optional-system-flag>");
    } else if cfg!(target_os = "linux") {
        println!("               | callable or -cal  : create a callable command (shell)");
        println!("                    | SUBOPTIONS (for callable) |");
        println!("                         | init or -i          : initialise callables");
    }

    println!("\n[COMMANDS]");
    println!("cls    : clear the screen");
    println!("gst    : git status");
    println!("gcl    : git clone");
}

fn license() {
    let licensetext = r###"    MIT License

    Copyright (c) 2024 Soumyo Deep Gupta
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE."###;

    println!("{}", licensetext);
}

fn version_info(version: &Version) {
    println!("{} {} (c) d33pster <deep.main.ac@gmail.com, Soumyo Deep Gupta>", version.name, version.version);
}
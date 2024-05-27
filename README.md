# Overview

`termutils` brings all your favourite command line tools in one place.

## Table of Contents

- [Features](#features)
- [Feature Description](#feature-description)
  - [`cls`](#cls)
  - [`gst`](#gst)
  - [`gcl`](#gcl)
  - [Custom Callable command creation (in shell)](#custom-callable-command-creation-in-shell)
  - [Custom Callable command creation for macOS Applications (both external and system)](#custom-callable-command-creation-for-macos-applications-both-external-and-system)

## Features

- `cls`
- `gst`
- `gcl`
- Custom callable command creation
- Custom Callable command creation for macOS Applications (both external and system)

## Feature Description

### `cls`

The windows `cls` command can now be used in Linux/MacOS without ruining the vibe and feel of Linux/MacOS.

#### Usage

```bash
$ cls
last login: 2024-05-27 14:22:40.473684 +05:30
(@_@) XXXXX@XXXX termutils % 
```

### `gst`

`gst` is a short form for git status coded in rust with better formatting and focusing mainly on the files.

#### usage

```bash
$ gst
on branch: main
Modified\
    Cargo.toml
    README.md

Ignored\
    Cargo.lock, target/
```

### `gcl`

`gcl` being an upgrade to the classic `git clone` command is a great tool for cloning repositories.  
Documentation of `gcl` can be found [here](https://github.com/d33pster/gcl).

#### usage

```bash
$ gcl help
gcl v1.0
Help Text

syntax: $ gcl [options] username repo <optional-destination-dir> [flags]

  | [Options]
  | help -> show help and exit.
  | version -> show version and exit.

  | [Flags]
  | --priv -> for private repos, add this flag.
```

### Custom callable command creation (in shell)

Custom callable commands can be created using termutils. The code for the same can be written in the terminal with shell code.

#### usage - help

```bash
$ term help
termutils v0.1.0
HELP
  | OPTIONS |
      | help or -h     : show this help text and exit
      | license or -l  : show license and exit
      | version or -v  : show version and exit
      | create or -c   : create terminal utilities
          | SUBOPTIONS (for create) |
               | callable or -cal  : create a callable command for any application
                    | SUBOPTIONS (for callable) |
                         | init or -i          : initialise callables

...
```

#### usage - example

```bash
# initialise callables
$ term create callable init
Initialised termutils for /Users/XXXX/.zshrc
```

```bash
$ term create callable
Command name: print hello world
Command nickname: hello world
Command Description (optional, press ENTER/RETURN for default): prints hello world in the terminal
Enter your command body here (shell):
echo "hello, world!"
END
Created command hello_world
 -> /Users/d33pster/.termutils
Restart Terminal!
```

```bash
$ hello_world
hello, world!
```

#### usage - description

- step 0.

> Initialize callables (one time only).

- step 1.

> Enter the name of the command.

- step 2.

> Enter the nickname or the name you want to call from the terminal. NOTE: if there are more than two words, it `_` will be added in between them.

- step 3.

> Add an optional description or just press RETURN/ENTER for default description.

- step 4.

> You need to type the shell code for the command line by line and once you are done, type `END` in a new line and press ENTER/RETURN.

> NOTE: Do not add `function function_name() {}`. Just write the body.

- step 5.

> All done. Restart the terminal to start using your command.

### Custom Callable command creation for macOS Applications (both external and system)

MacOS applications cannot be called directly from the terminal, but a callable command can be created so that they can be opened via terminal.

Here's how:

#### usage - help

```bash
$ term help
termutils v0.1.0
HELP
  | OPTIONS |
      | help or -h     : show this help text and exit
      | license or -l  : show license and exit
      | version or -v  : show version and exit
      | create or -c   : create terminal utilities
          | SUBOPTIONS (for create) |
               | callable or -cal  : create a callable command for any application
                    | SUBOPTIONS (for callable) |
                         | init or -i          : initialise callables
                         | application or -app : callable command for an application
                         | system or sys       : to specify if the app is a system app.
                    | SYNTAX |
                         $ term create callable init # need to be run before callables can be created.
                         $ term create callable -app <appname> <nickname> <optional-system-flag>

[COMMANDS]
cls    : clear the screen
gst    : git status
gcl    : git clone
```

#### usage - example

```bash
# first usage only
$ term create callable init
Initialised termutils for /Users/XXXX/.zshrc
```

```bash
# general syntax
term create callable <app-flag> <app-name> <nickname> <sys-flag>
# Here, the app-flag tells the termutils you are creating a callable command for an Application.

# app-name should be same as the one you can see in the /Applications folder.

# nickname arg is optional, if not passed, it will be prompted anyway. This nickname will be the command that you can call in the terminal. NOTE: if more than one word is there `_` will be added in place of a white space.

# finally, if you the Application is a system app like the App Store, you need to add a sys-flag. else no need.


# example
# suppose you need to create a callable command for Google Chrome App.
$ term create callable application "Google Chrome" "chrome"
Created Trigger for GOOGLE CHROME as CHROME
Restart terminal!

# example
# suppose its a system app:
$ term create callable application "App Store" "store" system
Created Trigger for APP STORE as STORE
Restart terminal!
```

```bash
$ chrome
Opening chrome...
done.
```

```bash
$ store
Opening store...
done.
```
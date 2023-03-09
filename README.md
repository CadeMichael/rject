# R-Ject

- I was a longtime Emacs user and really miss the project management that came with **projectile.el** and **project.el**
- at the same time I was looking for a rust toy project that impliments a simple TUI
- the solution seems to be a command line project manager that allows you to store (add, remove) project paths in a local file that the program can source to present as options to the user. 

# Supported 'Backends'

## Tmux
- this is the default way of opening a new project. The reasoning here is that it allows you to preserve the window you are in and that to cd in the current shell I would need to write a shell script that the rust code calls and my goal with this project is 100% rust. 

## Vs Code
- if anything other than 'code' is passed to the command it will default to Tmux but you can pass 
    - "code" -> open a **new** vscode window
    - "code add" -> add the project folder to an open vscode window

# Installing

- clone the [repo](github.com/cademichael/rject/) and run

```sh
# build binary
cargo build

# create a simlink to a folder in $Path to call the command from anywhere
ln -s [path to binary] [path to your .local/bin/]
```

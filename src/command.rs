use cursive::Cursive;
use cursive::views::{TextView, Dialog};
use std::env;
use std::process::Command;

enum Action {
    Code,
    CodeOss,
    Tmux,
}

pub fn execute_command(s: &mut Cursive, path: &str) {
    // get user args
    let args: Vec<String> = env::args().collect();
    // add to current vscode window
    let mut code_add = false;

    // parse commands
    let command: Action = match &args[..] {
        // no commands default to tmux
        [] => Action::Tmux,
        // one command check for code
        // any other command default to tmux
        [_, cmd] => {
            if cmd == "code" {
                Action::Code
            } else if cmd == "code-oss" {
                Action::CodeOss
            } else {
                Action::Tmux
            }
        }
        // check for "code add"
        // default to tmux
        [_, cmd, arg] => {
            if cmd == "code" && arg == "add" {
                code_add = true;
                Action::Code
            } else if cmd == "code-oss" && arg == "add" {
                code_add = true;
                Action::CodeOss
            } else {
                Action::Tmux
            }
        }
        // any other incorrect combo default to tmux
        _ => Action::Tmux,
    };
    match command {
        Action::Tmux => {
            match Command::new("tmux")
                .arg("new-window")
                .arg("-c")
                .arg(path)
                .output()
            {
                Ok(res) => {
                    // tmux not started
                    if res.stderr.len() != 0 {
                        s.add_layer(
                            Dialog::around(TextView::new("start tmux!"))
                                .button("exit", |s| s.quit()),
                        );
                    // no errors, go to dir
                    } else {
                        s.quit();
                    }
                }
                Err(_) => {
                    // error executing command
                    s.add_layer(TextView::new("Error!"));
                }
            };
        }
        Action::Code => {
            let win_type = if code_add { "-a" } else { "-n" };

            match Command::new("code").arg(win_type).arg(path).output() {
                Ok(_) => {
                    s.quit();
                }
                Err(_) => {
                    // error executing command
                    s.add_layer(TextView::new("Error!"));
                }
            };
        }
        Action::CodeOss => {
            let win_type = if code_add { "-a" } else { "-n" };

            match Command::new("code-oss").arg(win_type).arg(path).output() {
                Ok(_) => {
                    s.quit();
                }
                Err(_) => {
                    // error executing command
                    s.add_layer(TextView::new("Error!"));
                }
            };
        }
    };
}

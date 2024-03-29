use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use std::env;
use std::process::Command;
use crossterm::terminal::disable_raw_mode;

enum Action {
    Code,
    CodeOss,
    Nvim,
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
        // check for code
        [_, cmd] => {
            // vscode
            if cmd == "code" {
                Action::Code
            // cooler vscode
            } else if cmd == "code-oss" {
                Action::CodeOss
            // neovim
            } else if cmd == "nvim" {
                Action::Nvim
            // default
            } else {
                Action::Tmux
            }
        }
        // check for "code add"
        [_, cmd, arg] => {
            // regular vscode
            if cmd == "code" && arg == "add" {
                code_add = true;
                Action::Code
            // based OSS vscode
            } else if cmd == "code-oss" && arg == "add" {
                code_add = true;
                Action::CodeOss
            // any other input default
            } else {
                Action::Tmux
            }
        }
        // any other incorrect combo default to tmux
        _ => Action::Tmux,
    };
    match command {
        Action::Tmux => {
            // open new window at 'path'
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
                    s.add_layer(
                        Dialog::around(TextView::new("start tmux!")).button("exit", |s| s.quit()),
                    );
                }
            };
        }
        Action::Nvim => {
            // give nvim the right cwd
            let _ = env::set_current_dir(path);
            // open nvim
            match Command::new("nvim").arg("./").status() {
                Ok(_) => {
                    let _ = disable_raw_mode();
                    s.quit();
                },
                Err(_) => {
                    s.add_layer(
                        Dialog::around(TextView::new("neovim not found")).button("exit", |s| s.quit()),
                    );
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
                    s.add_layer(
                        Dialog::around(TextView::new("Code not found!")).button("exit", |s| s.quit()),
                    );
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
                    s.add_layer(
                        Dialog::around(TextView::new("Code not found!")).button("exit", |s| s.quit()),
                    );
                }
            };
        }
    };
}

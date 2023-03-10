use std::env;
use std::process::Command;

use cursive::event::{EventResult, Key};
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, ListView, NamedView, OnEventView, SelectView, TextView,
    ViewRef,
};
use cursive::Cursive;

use crate::command;
use crate::file;

pub fn create_base_view(siv: &mut Cursive, select: NamedView<OnEventView<SelectView>>) {
    // get user args
    let args: Vec<String> = env::args().collect();
    // parse user args for 'backend' to show user
    let open_ins = match &args[..] {
        // no input
        [_] => "Tmux",
        // one cmd
        [_, cmd] => {
            if cmd == "code" || cmd == "code-oss" {
                "Code"
            } else {
                "Tmux"
            }
        }
        // more than one command
        [_, cmd, ..] => {
            // vscode
            if cmd == "code" || cmd == "code-oss" {
                "Code"
            // tmux
            } else {
                "Tmux"
            }
        }
        // default to tmux
        _ => "Tmux",
    };

    // create base layer
    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            // select list
            .child(Dialog::around(select.scrollable()).title("Projects"))
            // instructions
            .child(TextView::new(" | h:help | n:new | D:del | Esc:exit "))
            // backend
            .child(TextView::new(format!(" Using: {}", open_ins))),
    ));
}

pub fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();
    // use current terminal theme
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    theme
}

pub fn new_proj_popup(s: &mut Cursive) {
    s.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(
                // dialog for adding projects
                Dialog::new()
                    .title("New Project Path:")
                    .padding_lrtb(1, 1, 1, 0)
                    .content(
                        EditView::new()
                            .on_submit(created_new_popup)
                            .with_name("new proj")
                            .full_width()
                            .max_width(50),
                    )
                    // paste pwd to 'EditView'
                    .button("pwd", |s| {
                        // get dir
                        let dir = env::current_dir();
                        // use let outside of match to bind variable
                        let string;
                        // get path as &String
                        let path = match dir {
                            Ok(s) => {
                                string = format!("{}/", s.to_str().unwrap());
                                &string
                            }
                            Err(e) => panic!("error retrieving dir {e}"),
                        };
                        // set editable text to 'pwd'
                        s.call_on_name("new proj", |view: &mut EditView| view.set_content(path));
                    })
                    // add project to ~/.proj and make popup
                    .button("add", |s| {
                        // get path from input
                        let path = s
                            .call_on_name("new proj", |view: &mut EditView| view.get_content())
                            .unwrap();
                        // create popup
                        created_new_popup(s, &path);
                    })
                    .button("cancel", |s| {
                        // remove popup
                        s.pop_layer();
                        // rebuild base view
                        create_base_view(s, create_select_list());
                    }),
            )
            // show user pwd
            .child(TextView::new(format!(
                "pwd:\n{}/",
                env::current_dir().unwrap().to_str().unwrap()
            ))),
    ));
}

fn create_preview(s: &mut Cursive, path: &str) {
    // define command
    let mut cmd_ls = Command::new("ls");
    // execute from 'current_dir'
    let out = cmd_ls.current_dir(path).output();

    // match against output
    let output = match out {
        Ok(res) => String::from_utf8(res.stdout).unwrap(),
        Err(e) => panic!("failed at: {}", e),
    };

    // convert output to Vector
    let mut res_vec = output.split("\n").collect::<Vec<&str>>().clone();
    // remove blank ""
    res_vec.pop();
    // instantiate blank view
    let mut l_view = ListView::new();
    // populate view with files in project
    for f in res_vec {
        l_view.add_child("|-", TextView::new(f))
    }

    // compose everything into a 'Dialog'
    s.add_layer(Dialog::around(
        LinearLayout::vertical()
            // title and files
            .child(Dialog::new().title(path).content(l_view).scrollable())
            // button to exit to main menu
            .child(
                Dialog::around(Button::new("back", |s| {
                    s.pop_layer();
                }))
                .max_width("<back>".len() + 4),
            ),
    ));
}

pub fn create_select_list() -> NamedView<OnEventView<SelectView>> {
    // create base select
    let mut select = SelectView::new()
        .h_align(cursive::align::HAlign::Center)
        .autojump();
    // add projects
    select.add_all_str(file::read_proj());
    // set keybindings
    select.set_on_submit(|s, path: &str| {
        command::execute_command(s, path);
    });

    // set events for select list
    let select = OnEventView::new(select)
        // -- exit --
        .on_event(Key::Esc, |s| s.quit())
        // -- help menu --
        .on_event('h', |s| {
            s.add_layer(Dialog::around(
                Dialog::new()
                    .title("Help Menu")
                    .content(TextView::new(" p:preview | r:refresh | j/k: up down "))
                    .button("back", |s| {
                        s.pop_layer();
                    }),
            ));
        })
        // -- preview current project --
        .on_event('p', |s| {
            let select: ViewRef<OnEventView<SelectView>> = s.find_name("Selection").unwrap();
            let sel = select.get_inner();
            let path = sel.selection().unwrap();
            create_preview(s, &path)
        })
        // -- moving around list --
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        // -- refresh list --
        .on_event_inner('r', |s, _| {
            s.clear();
            s.add_all_str(file::read_proj());
            None
        })
        // -- adding / removing projects --
        .on_event_inner('D', |s, _| {
            // remove selected path from ~/.proj
            file::remove_project(&s.selection().unwrap(), file::read_proj());
            // graphically remove item from list
            let cb = s.remove_item(s.selected_id().unwrap());
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_event('n', move |s| {
            // pop select list view
            s.pop_layer();
            // popup to create new project
            new_proj_popup(s);
        })
        // set name for referencing
        .with_name("Selection");

    select
}

pub fn created_new_popup(s: &mut Cursive, path: &str) {
    // no user data
    if path.is_empty() {
        s.add_layer(Dialog::info("Enter Project Path:"));
    } else {
        // determine if path is correct or if project already exists
        let content = match file::add_project(path, &file::read_proj()) {
            None => format!("Project {path}!"),
            Some(s) => s,
        };

        // remove popup
        s.pop_layer();
        // allert that new path has been added
        s.add_layer(Dialog::around(TextView::new(content)).button("Ok", |s| {
            s.pop_layer();
            // remake base view with new select list
            create_base_view(s, create_select_list());
        }));
    }
}

use std::env;
use std::process::Command;

use cursive::event::{EventResult, Key};
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, OnEventView, SelectView, TextView};
use cursive::Cursive;

// I think this import is correct
use crate::proj_file;

pub fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme
}

pub fn new_proj_popup(s: &mut Cursive) {
    s.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(
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
                    .button("add", |s| {
                        let path = s
                            .call_on_name("new proj", |view: &mut EditView| view.get_content())
                            .unwrap();
                        created_new_popup(s, &path);
                    })
                    .button("cancel", |s| {
                        s.pop_layer();
                    }),
            )
            // show user pwd
            .child(TextView::new(format!(
                "pwd:\n{}/",
                env::current_dir().unwrap().to_str().unwrap()
            ))),
    ));
}

pub fn create_select_list() -> OnEventView<SelectView> {
    // create base select
    let mut select = SelectView::new()
        .h_align(cursive::align::HAlign::Center)
        .autojump();
    // add projects
    select.add_all_str(proj_file::read_proj());
    // set keybindings
    select.set_on_submit(|s, path: &str| {
        // open new tmux window at given path
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
                        Dialog::around(TextView::new("start tmux!")).button("exit", |s| s.quit()),
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
    }); //created_new_popup(s, name));
        // s in 'event_inner' is the select
        // s in 'event' is cursive instance
        // the _ i'm using represents an 'Event' not sure if needs to be delt with
        // for what we're doing
    let select = OnEventView::new(select)
        .on_event(Key::Esc, |s| s.quit())
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
            s.add_all_str(proj_file::read_proj());
            None
        })
        // -- adding / removing projects --
        .on_event_inner('D', |s, _| {
            // cannot borrow so just re read file, optimize later
            proj_file::remove_project(&s.selection().unwrap(), proj_file::read_proj());
            // add popup to confirm here
            // -- TODO
            // graphically remove item from list
            let cb = s.remove_item(s.selected_id().unwrap());
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_event('n', |s| {
            // popup to create new project
            new_proj_popup(s);
        });

    select
}

pub fn created_new_popup(s: &mut Cursive, path: &str) {
    if path.is_empty() {
        s.add_layer(Dialog::info("Enter Project Path:"));
    } else {
        // determine if path is correct or if project already exists
        let content = match proj_file::add_project(path, &proj_file::read_proj()) {
            None => format!("Project {path}!"),
            Some(s) => s,
        };
        s.pop_layer();
        s.add_layer(Dialog::around(TextView::new(content)).button("Ok", |s| {
            s.pop_layer();
        }));
        println!("{}", &path);
    }
}

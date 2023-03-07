use cursive::event::EventResult;
// use cursive::event::EventResult;
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::traits::*;
// use cursive::view::Selector;
use cursive::views::{Dialog, EditView, LinearLayout, OnEventView, SelectView, TextView};
use cursive::Cursive;

mod proj_file;

// fn change(siv: &mut Cursive) -> EventResult {
// let name = siv.focus(&Selector::Name("eview"));
// return name.unwrap_or_else(|e| panic!("{}", e));
// }

fn main() {
    let mut select = SelectView::new()
        .h_align(cursive::align::HAlign::Center)
        .autojump();
    select.add_all_str(proj_file::read_proj());
    let mut siv = cursive::default();
    select.set_on_submit(|s, name: &str| created_new_popup(s, name));
    // s in 'event_inner' is the select
    // s in 'event' is cursive instance
    // the _ i'm using represents an 'Event' not sure if needs to be delt with
    // for what we're doing
    let select = OnEventView::new(select)
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

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);
    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
        .child(Dialog::around(select.scrollable()).title("Projects"))
        .child(TextView::new("r: refresh n: new D: delete"))
        ));

    siv.run();
}

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme
}

fn new_proj_popup(s: &mut Cursive) {
    s.add_layer(
        Dialog::new()
            .title("New Project Path:")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .on_submit(created_new_popup)
                    .with_name("new proj")
                    .fixed_width(20)
                    .with_name("eview"),
            )
            .button("Ok", |s| {
                let path = s
                    .call_on_name("new proj", |view: &mut EditView| view.get_content())
                    .unwrap();
                created_new_popup(s, &path);
            }),
    )
}

fn created_new_popup(s: &mut Cursive, path: &str) {
    if path.is_empty() {
        s.add_layer(Dialog::info("Enter Project Path:"));
    } else {
        let content = format!("Project {path}!");
        proj_file::add_project(path, &proj_file::read_proj());
        s.pop_layer();
        s.add_layer(Dialog::around(TextView::new(content)).button("Ok", |s| {
            s.pop_layer();
        }));
    }
}

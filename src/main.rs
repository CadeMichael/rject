use cursive::event::EventResult;
// use cursive::event::EventResult;
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::traits::*;
use cursive::view::Selector;
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
    let content = proj_file::read_proj();
    select.add_all_str(content);
    let mut siv = cursive::default();
    select.set_on_submit(|s, name: &str| show_popup(s, name));
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
        // -- adding / removing projects --
        .on_event('n', |s| {
            s.focus(&Selector::Name("eview")).unwrap();
        });

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);
    siv.add_layer(Dialog::around(
        LinearLayout::horizontal()
            .child(Dialog::around(select.scrollable()))
            .child(
                Dialog::new()
                    .title("New Project Path:")
                    .padding_lrtb(1, 1, 1, 0)
                    .content(
                        EditView::new()
                            .on_submit(show_popup)
                            .with_name("new proj")
                            .fixed_width(20)
                            .with_name("eview"),
                    )
                    .button("Ok", |s| {
                        let path = s
                            .call_on_name("new proj", |view: &mut EditView| view.get_content())
                            .unwrap();
                        show_popup(s, &path);
                    }),
            ),
    ));

    siv.run();
}

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme
}

fn show_popup(s: &mut Cursive, path: &str) {
    if path.is_empty() {
        s.add_layer(Dialog::info("Enter Project Path:"));
    } else {
        let content = format!("Project {path}!");
        proj_file::add_project(path, &proj_file::read_proj());
        s.pop_layer();
        s.add_layer(Dialog::around(TextView::new(content)).button("Quit", |s| s.quit()));
    }
}

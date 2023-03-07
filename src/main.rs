use cursive::view::Scrollable;
use cursive::views::{Dialog, LinearLayout, TextView};

mod proj_file;
mod proj_tui;

// fn change(siv: &mut Cursive) -> EventResult {
// let name = siv.focus(&Selector::Name("eview"));
// return name.unwrap_or_else(|e| panic!("{}", e));
// }

fn main() {
    let mut siv = cursive::default();
    let select = proj_tui::create_select_list();
    let theme = proj_tui::custom_theme_from_cursive(&siv);

    siv.set_theme(theme);
    siv.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(Dialog::around(select.scrollable()).title("Projects"))
            .child(TextView::new("r: refresh n: new D: delete Esc: exit")),
    ));

    siv.run();
}

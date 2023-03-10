mod file;
mod tui;
mod command;

fn main() {
    // create TUI instance
    let mut siv = cursive::default();
    // make select list
    let select = tui::create_select_list();
    // make theme
    let theme = tui::custom_theme_from_cursive(&siv);

    // populate TUI
    siv.set_theme(theme);
    tui::create_base_view(&mut siv, select);

    // run UI
    siv.run();
}

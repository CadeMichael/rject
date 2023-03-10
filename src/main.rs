mod file;
mod tui;

fn main() {
    let mut siv = cursive::default();
    let select = tui::create_select_list();
    let theme = tui::custom_theme_from_cursive(&siv);

    siv.set_theme(theme);
    tui::create_base_view(&mut siv, select);
    siv.run();
}

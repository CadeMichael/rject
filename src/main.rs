mod proj_file;
mod proj_tui;

fn main() {
    let mut siv = cursive::default();
    let select = proj_tui::create_select_list();
    let theme = proj_tui::custom_theme_from_cursive(&siv);

    siv.set_theme(theme);
    proj_tui::create_base_view(&mut siv, select);
    siv.run();
}

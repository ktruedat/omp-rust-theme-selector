mod bashrc;
mod errors;
mod themes;

use crate::themes::{list_themes, select_theme};
use std::env;

fn main() {
    let user_dir = env::var("HOME").unwrap();

    let theme_dir = env::var("OMP_THEME_DIR")
        .unwrap_or_else(|_| format!("{}/.terminal_themes", user_dir).to_string());

    let themes = list_themes(&theme_dir).unwrap();
    let theme_name = select_theme(&themes).unwrap();

    bashrc::modify_bashrc(
        &theme_name,
        &theme_dir,
        format!("{}/.bashrc", user_dir).as_str(),
    )
    .unwrap();
}

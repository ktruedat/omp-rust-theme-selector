use crate::errors::ThemeError;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn update_bashrc(theme_name: &str, theme_dir: &str, bashrc_file: &str) -> Result<(), ThemeError> {
    let bashrc_path = Path::new(bashrc_file);
    let mut file = OpenOptions::new().append(true).open(bashrc_path)?;

    writeln!(file, "\n# oh-my-posh theme selected by Theme Selector app")?;
    writeln!(
        file,
        "eval \"$(oh-my-posh init bash --config {}/{})\"",
        theme_dir, theme_name
    )?;

    Ok(())
}

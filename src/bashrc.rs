use crate::errors::ThemeError;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

pub fn modify_bashrc(theme_name: &str, theme_dir: &str, file_path: &str) -> Result<(), ThemeError> {
    let keyword = "oh-my-posh";
    let new_line = format!(
        "eval \"$(oh-my-posh init bash --config {}/{})\"",
        theme_dir, theme_name
    );
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut found = false;
    for line in lines.iter_mut() {
        if line.contains(keyword) {
            *line = new_line.to_string();
            found = true;
            break;
        }
    }

    if !found {
        lines.push(new_line.to_string());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

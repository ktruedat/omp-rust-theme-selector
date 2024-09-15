use crate::errors::ThemeError;
use std::fs;
use std::io::{self, Write};

fn list_themes(directory: &str) -> Result<Vec<String>, ThemeError> {
    let paths = fs::read_dir(directory)?;
    let mut themes = Vec::new();

    for path in paths {
        let entry = path?;
        let theme_name = entry
            .file_name()
            .into_string()
            .map_err(|e| ThemeError::Utf8(e.into_string().unwrap_err()))?;
        themes.push(theme_name);
    }

    Ok(themes)
}

fn select_theme(themes: &[String]) -> Result<String, ThemeError> {
    println!("Available themes:");
    for (i, theme) in themes.iter().enumerate() {
        println!("{}: {}", i + 1, theme);
    }

    print!("Enter the number of the theme you want to select: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let selected = input.trim().parse::<usize>()?;

    if selected > 0 && selected <= themes.len() {
        Ok(themes[selected - 1].clone())
    } else {
        Err(ThemeError::InvalidChoice)
    }
}

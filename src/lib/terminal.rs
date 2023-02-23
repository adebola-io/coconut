use std::{fmt::Display, io::Error, path::PathBuf};

pub struct Fmt {
    text: String,
    codes: Vec<u8>,
}

pub trait FmtTrait {
    fn fmt(&self) -> Fmt;
}

impl Fmt {
    pub fn from<T>(value: T) -> Self
    where
        T: Display,
    {
        Fmt {
            text: value.to_string(),
            codes: vec![],
        }
    }
    pub fn apply(&self, codes: Vec<u8>) -> Self {
        let mut init_codes = self.codes.clone();
        for code in codes {
            init_codes.push(code);
        }
        Fmt {
            text: self.text.to_owned(),
            codes: init_codes,
        }
    }
    pub fn bold(&self) -> Self {
        add_format(self, 1)
    }
    pub fn underline(&self) -> Self {
        add_format(self, 4)
    }
    pub fn black(&self) -> Self {
        add_format(self, 30)
    }
    pub fn red(&self) -> Self {
        add_format(self, 31)
    }
    pub fn green(&self) -> Self {
        add_format(self, 32)
    }
    pub fn yellow(&self) -> Self {
        add_format(self, 33)
    }
    pub fn blue(&self) -> Self {
        add_format(self, 34)
    }
    pub fn pink(&self) -> Self {
        add_format(self, 35)
    }
    pub fn cyan(&self) -> Self {
        add_format(self, 36)
    }
    pub fn white(&self) -> Self {
        add_format(self, 37)
    }
    pub fn bg_black(&self) -> Self {
        add_format(self, 40)
    }
    pub fn bg_red(&self) -> Self {
        add_format(self, 41)
    }
    pub fn bg_green(&self) -> Self {
        add_format(self, 42)
    }
    pub fn bg_yellow(&self) -> Self {
        add_format(self, 43)
    }
    pub fn bg_blue(&self) -> Self {
        add_format(self, 44)
    }
    pub fn bg_purple(&self) -> Self {
        add_format(self, 45)
    }
    pub fn bg_cyan(&self) -> Self {
        add_format(self, 46)
    }
    pub fn bg_white(&self) -> Self {
        add_format(self, 47)
    }
}

fn add_format(text: &Fmt, code: u8) -> Fmt {
    let mut codes = text.codes.clone();
    codes.push(code);
    Fmt {
        text: text.text.to_owned(),
        codes,
    }
}

impl Display for Fmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rendered = String::from("\x1b[");
        let mut i = 0;
        let code_length = self.codes.len();
        while i < code_length {
            rendered.push_str(&self.codes[i].to_string());
            if i < code_length - 1 {
                rendered.push(';');
            }
            i += 1;
        }
        write!(f, "{}m{}\x1b[0m", rendered, self.text)
    }
}

/// Logs a success message to the terminal.
pub fn success<T>(message: T)
where
    T: Display,
{
    print!(
        "{} {}",
        Fmt::from(" SUCCESS: ").bg_green().white().bold(),
        Fmt::from(message).green()
    )
}

/// Log an error to the console.
pub fn error<T>(error: T)
where
    T: Display,
{
    eprintln!(
        "{} {}",
        Fmt::from(" ERROR: ").bg_red().black().bold(),
        Fmt::from(error.to_string()).red()
    )
}

/// Logs an error to the console and close the command.
pub fn fatal_error<T>(message: T) -> !
where
    T: Display,
{
    eprintln!("{}", Fmt::from(message).red());
    std::process::exit(0);
}

/// Logs a warning to the console.
pub fn warn<T>(error: T)
where
    T: Display,
{
    println!(
        "{} {}",
        Fmt::from(" WARNING: ").bg_yellow().black().bold(),
        Fmt::from(error).yellow()
    )
}

pub fn inform<T>(message: T)
where
    T: Display,
{
    println!("{}", Fmt::from(message).blue())
}

impl FmtTrait for Error {
    fn fmt(&self) -> Fmt {
        Fmt::from(self.to_string())
    }
}

impl FmtTrait for String {
    fn fmt(&self) -> Fmt {
        Fmt::from(&self)
    }
}

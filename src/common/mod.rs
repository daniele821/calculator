use std::{
    fmt::{self, Display},
    io::{self, IsTerminal},
};

pub mod algs;

pub enum Color {
    /// success
    SUC,
    /// failure
    FAI,
    /// title
    TIT,
    /// sub-title
    SUB,
    /// other
    OTH,
}

pub fn color<T: Display + ?Sized>(color: &Color, str: &T) -> String {
    let str = str.to_string();
    if !io::stdout().is_terminal() {
        return str;
    }
    match color {
        Color::SUC => format!("\x1b[1;32m{str}\x1b[0m"),
        Color::FAI => format!("\x1b[1;31m{str}\x1b[0m"),
        Color::TIT => format!("\x1b[1;34m{str}\x1b[0m"),
        Color::SUB => format!("\x1b[1;36m{str}\x1b[0m"),
        Color::OTH => format!("\x1b[1;33m{str}\x1b[0m"),
    }
}

pub fn fmt<T: fmt::Display>(items: &[T], sep: Option<&str>) -> String {
    items
        .iter()
        .map(T::to_string)
        .collect::<Vec<_>>()
        .join(sep.unwrap_or(" "))
}

pub fn convert<T: Clone, F: From<T>>(items: &[T]) -> Vec<F> {
    items.iter().map(|t| F::from(t.clone())).collect()
}

#![allow(dead_code)]

use std::fmt;

pub fn fmt<T: fmt::Display>(items: &[T], sep: Option<&str>) -> String {
    items
        .iter()
        .map(T::to_string)
        .collect::<Vec<_>>()
        .join(sep.unwrap_or(" "))
}

pub fn fmt_dbg<T: fmt::Debug>(items: &[T], sep: Option<&str>) -> String {
    items
        .iter()
        .map(|t| format!("{t:?}"))
        .collect::<Vec<_>>()
        .join(sep.unwrap_or(" "))
}

pub fn convert<T: Clone, F: From<T>>(items: &[T]) -> Vec<F> {
    items.iter().map(|t| F::from(t.clone())).collect()
}

use std::cell::LazyCell;

use clap::Error;
use regex::Regex;
use yew::{Html, html};

const COLOR_REGEX: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"\u{1b}\[(?P<fg>\d{1,2})(;\d{1,2})?m").unwrap());

#[derive(Debug, Clone, Copy)]
enum Color {
    None,
    Red,
    Green,
    Yellow,
}

impl<'a> From<&'a str> for Color {
    fn from(text: &str) -> Self {
        match text {
            "31" => Color::Red,
            "32" => Color::Green,
            "33" => Color::Yellow,
            _ => Color::None,
        }
    }
}

pub fn convert(err: &Error) -> Html {
    let ansi_str = format!("{}", err.render().ansi());
    let mut chunks: Vec<(Color, &str)> = Vec::new();
    let mut last_text = 0;
    let mut last_color = Color::None;

    COLOR_REGEX
        .captures_iter(&ansi_str)
        .zip(COLOR_REGEX.find_iter(&ansi_str))
        .for_each(|(capture, m)| {
            if m.start() > last_text {
                chunks.push((last_color, &ansi_str[last_text..m.start()]));
            }
            let color = Color::from(capture.name("fg").unwrap().as_str());

            last_text = m.end();
            last_color = color;
        });
    // append remainder of string
    if last_text < ansi_str.len() {
        chunks.push((last_color, &ansi_str[last_text..]));
    }

    // get start and end ranges for colors

    chunks
        .iter()
        .map(|(color, text)| {
            let text_html = text.chars().map(|c| match c {
                '\n' => html! {<br />},
                '\t' => html! {"\u{a0}\u{a0}\u{a0}\u{a0}"},
                ' ' => html! {"\u{a0}"},
                _ => html! {c},
            });
            match color {
                Color::None => html! {for text_html},
                Color::Red => html! {<span class="highlight-red">{for text_html}</span>},
                Color::Green => html! {<span class="highlight-green">{for text_html}</span>},
                Color::Yellow => html! {<span class="highlight-yellow">{for text_html}</span>},
            }
        })
        .collect()
}

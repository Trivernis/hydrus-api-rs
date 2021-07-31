use crate::wrapper::tag::Tag;
use chrono::{Datelike, Duration};

pub fn string_list_to_json_array(l: Vec<String>) -> String {
    format!("[\"{}\"]", l.join("\",\""))
}

pub fn number_list_to_json_array<T: ToString>(l: Vec<T>) -> String {
    format!(
        "[{}]",
        l.into_iter()
            .fold(String::from(""), |acc, val| format!(
                "{},{}",
                acc,
                val.to_string()
            ))
            .trim_start_matches(",")
    )
}

/// Converts a list of tags into a list of string tags
pub fn tag_list_to_string_list(tags: Vec<Tag>) -> Vec<String> {
    tags.into_iter().map(|t| t.to_string()).collect()
}

pub fn format_datetime<D: Datelike>(datetime: D) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        datetime.year(),
        datetime.month(),
        datetime.day()
    )
}

pub fn format_duration(duration: Duration) -> String {
    let mut expression = String::new();
    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    if days > 0 {
        expression.push_str(&days.to_string());
        expression.push_str(" days ");
    }
    if hours > 0 {
        expression.push_str(&hours.to_string());
        expression.push_str(" hours ")
    }
    if minutes > 0 {
        expression.push_str(&minutes.to_string());
        expression.push_str(" minutes ");
    }
    expression.push_str(&seconds.to_string());
    expression.push_str(" seconds");

    expression
}

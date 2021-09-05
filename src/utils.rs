use crate::api_core::common::FileIdentifier;
use crate::api_core::searching_and_fetching_files::SearchQueryEntry;
use crate::wrapper::tag::Tag;
use chrono::{Datelike, Duration};

/// Converts a list of Search Query entries into a json array
pub(crate) fn search_query_list_to_json_array(l: Vec<SearchQueryEntry>) -> String {
    let entry_list: Vec<String> = l
        .into_iter()
        .map(|e| match e {
            SearchQueryEntry::Tag(t) => format!("\"{}\"", t),
            SearchQueryEntry::OrChain(c) => string_list_to_json_array(c),
        })
        .collect();

    format!("[{}]", entry_list.join(","))
}

pub(crate) fn string_list_to_json_array(l: Vec<String>) -> String {
    format!("[\"{}\"]", l.join("\",\""))
}

pub(crate) fn number_list_to_json_array<T: ToString>(l: Vec<T>) -> String {
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

pub(crate) fn format_datetime<D: Datelike>(datetime: D) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        datetime.year(),
        datetime.month(),
        datetime.day()
    )
}

pub(crate) fn format_duration(duration: Duration) -> String {
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

pub(crate) fn split_file_identifiers_into_hashes_and_ids(
    files: Vec<FileIdentifier>,
) -> (Vec<u64>, Vec<String>) {
    let mut ids = Vec::new();
    let mut hashes = Vec::new();

    for file in files {
        match file {
            FileIdentifier::ID(id) => ids.push(id),
            FileIdentifier::Hash(hash) => hashes.push(hash),
        }
    }
    (ids, hashes)
}

use crate::wrapper::tag::Tag;

pub fn string_list_to_json_array(l: Vec<String>) -> String {
    format!("[\"{}\"]", l.join("\",\""))
}

pub fn number_list_to_json_array<T: ToString>(l: Vec<T>) -> String {
    format!(
        "[{}]",
        l.into_iter().fold(String::from(""), |acc, val| format!(
            "{},{}",
            acc,
            val.to_string()
        ))
    )
}

/// Converts a list of tags into a list of string tags
pub fn tag_list_to_string_list(tags: Vec<Tag>) -> Vec<String> {
    tags.into_iter().map(|t| t.to_string()).collect()
}

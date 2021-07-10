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

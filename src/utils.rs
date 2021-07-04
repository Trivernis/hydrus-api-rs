pub fn string_list_to_json_array(l: Vec<String>) -> String {
    format!("[\"{}\"]", l.join("\",\""))
}

use std::collections::HashMap;

pub fn parse_data(data: String) -> Option<HashMap<String, String>> {
    let mut result: HashMap<String, String> = HashMap::new();

    let mut current_field_name: Option<&str> = None;

    for line in data.lines() {
        let line = line.trim();

        if line.trim().ends_with(":") {
            current_field_name = Some(line.get(..line.len() - 1)?);
            result.insert(current_field_name?.to_string(), "".to_string());
        } else if let Some(field_name) = current_field_name {
            if let Some(field) = result.get_mut(field_name) {
                *field = format!("{}\n{}", field, line).trim().to_string();
            }
        }
    }

    Some(result)
}

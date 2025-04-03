use serde_json::{Value, json};

pub fn generate_map(content: &str, language: Option<String>) -> Option<Value> {
    match language {
        Some(language) if language[.. 2].chars().all(|char| char.is_ascii_lowercase()) =>
            Some(json!({
                language[..2]: content
            })),
        _ => None,
    }
}

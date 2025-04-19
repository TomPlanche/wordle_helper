use crate::{
    data::{convert_word_data, WordData},
    game_logic::filter_words,
    load_words, Word,
};

#[tauri::command]
pub fn filter_word_list(patterns: Vec<WordData>) -> Result<Vec<String>, String> {
    // Convert all pattern words to our internal Word type
    let converted_patterns: Result<Vec<Word>, String> =
        patterns.iter().map(convert_word_data).collect();

    match converted_patterns {
        Ok(patterns) => {
            // Load all words and filter them
            let all_words = load_words();
            Ok(filter_words(&all_words, &patterns))
        }
        Err(e) => Err(e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![filter_word_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;

    // Helper function to parse JSON into WordData
    fn parse_word_data(json_data: Value) -> Vec<WordData> {
        serde_json::from_value(json_data).expect("Failed to parse JSON data")
    }

    #[test]
    fn test_filter_word_list_basic() {
        // Single pattern with one correct letter
        let json_patterns = json!([
            {
                "letters": [
                    {"character": "p", "state": "correct"},
                    {"character": "a", "state": "unknown"},
                    {"character": "i", "state": "unknown"},
                    {"character": "n", "state": "unknown"},
                    {"character": "t", "state": "unknown"}
                ]
            }
        ]);

        let patterns = parse_word_data(json_patterns);
        let result = filter_word_list(patterns).unwrap();

        // Results should contain words starting with 'p'
        assert!(result.iter().all(|w| w.starts_with('p')));
        assert!(result.contains(&"paint".to_string()));
        assert!(result.contains(&"place".to_string()));
    }

    #[test]
    fn test_filter_word_list_multiple_patterns() {
        // Test with multiple patterns
        let json_patterns = json!([
            {
                "letters": [
                    {"character": "t", "state": "unknown"},
                    {"character": "r", "state": "unknown"},
                    {"character": "a", "state": "correct"},
                    {"character": "i", "state": "unknown"},
                    {"character": "n", "state": "unknown"}
                ]
            },
            {
                "letters": [
                    {"character": "p", "state": "unknown"},
                    {"character": "l", "state": "unknown"},
                    {"character": "a", "state": "unknown"},
                    {"character": "n", "state": "correct"},
                    {"character": "e", "state": "unknown"}
                ]
            }
        ]);

        let patterns = parse_word_data(json_patterns);
        let result = filter_word_list(patterns).unwrap();

        // Results should have 'a' at position 2 and 'n' at position 3
        assert!(result.iter().all(|w| {
            let chars: Vec<char> = w.chars().collect();
            chars[2] == 'a' && chars[3] == 'n'
        }));
    }

    #[test]
    fn test_filter_word_list_misplaced() {
        // Test with misplaced letters
        let json_patterns = json!([
            {
                "letters": [
                    {"character": "r", "state": "misplaced"},
                    {"character": "e", "state": "unknown"},
                    {"character": "a", "state": "unknown"},
                    {"character": "c", "state": "unknown"},
                    {"character": "h", "state": "unknown"}
                ]
            }
        ]);

        let patterns = parse_word_data(json_patterns);
        let result = filter_word_list(patterns).unwrap();

        // Results should contain 'r' but not at first position
        assert!(result.iter().all(|w| {
            let chars: Vec<char> = w.chars().collect();
            w.contains('r') && chars[0] != 'r'
        }));
    }

    #[test]
    fn test_filter_word_list_absent() {
        // Test with absent letters
        let json_patterns = json!([
            {
                "letters": [
                    {"character": "q", "state": "absent"},
                    {"character": "w", "state": "absent"},
                    {"character": "e", "state": "absent"},
                    {"character": "r", "state": "absent"},
                    {"character": "t", "state": "absent"}
                ]
            }
        ]);

        let patterns = parse_word_data(json_patterns);
        let result = filter_word_list(patterns).unwrap();

        // Results should not contain any of the letters q, w, e, r, t
        assert!(result.iter().all(|w| {
            !w.contains('q')
                && !w.contains('w')
                && !w.contains('e')
                && !w.contains('r')
                && !w.contains('t')
        }));
    }

    #[test]
    fn test_filter_word_list_invalid_input() {
        // Test with invalid input (too few letters)
        let json_patterns = json!([
            {
                "letters": [
                    {"character": "h", "state": "unknown"},
                    {"character": "i", "state": "unknown"}
                ]
            }
        ]);

        let patterns = parse_word_data(json_patterns);
        let result = filter_word_list(patterns);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must have exactly 5 letters"));
    }

    #[test]
    fn test_filter_word_list_mixed_constraints() {
        // Test with a mix of constraints
        let json_patterns = json!([
            {
                "letters": [
                    {"character": "b", "state": "unknown"},
                    {"character": "l", "state": "correct"},
                    {"character": "a", "state": "correct"},
                    {"character": "c", "state": "unknown"},
                    {"character": "k", "state": "absent"}
                ]
            }
        ]);

        let patterns = parse_word_data(json_patterns);
        let result = filter_word_list(patterns).unwrap();

        // Results should have 'l' at position 1, 'a' at position 2, and no 'k' at the end
        assert!(result.iter().all(|w| {
            let chars: Vec<char> = w.chars().collect();
            chars[1] == 'l' && chars[2] == 'a' && chars[4] != 'k'
        }));
    }

    #[test]
    fn test_direct_json_string() {
        // Test with direct JSON string parsing
        let json_str = r#"[
                {
                    "letters": [
                        {"character": "s", "state": "correct"},
                        {"character": "t", "state": "correct"},
                        {"character": "o", "state": "unknown"},
                        {"character": "n", "state": "unknown"},
                        {"character": "e", "state": "unknown"}
                    ]
                }
            ]"#;

        let patterns: Vec<WordData> = serde_json::from_str(json_str).unwrap();
        let result = filter_word_list(patterns).unwrap();

        // Results should start with "st"
        assert!(result.iter().all(|w| w.starts_with("st")));
    }
}

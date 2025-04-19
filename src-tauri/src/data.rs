//! Module containing data structures and conversion functions for frontend data.

use serde::{Deserialize, Serialize};

use crate::{LetterState, Word};

// Serializable struct to represent letter data from frontend
#[derive(Serialize, Deserialize)]
pub struct LetterData {
    pub character: char,
    pub state: String, // "unknown", "correct", "misplaced", "absent"
}

// Serializable struct to represent word data from frontend
pub type WordData = Vec<LetterData>;

// Convert frontend LetterState string to backend LetterState enum
fn convert_letter_state(state: &str) -> LetterState {
    match state {
        "correct" => LetterState::Correct,
        "misplaced" => LetterState::Misplaced,
        "absent" => LetterState::Absent,
        _ => LetterState::Unknown,
    }
}

// Convert WordData from frontend to Word struct in backend
pub fn convert_word_data(word_data: &WordData) -> Result<Word, String> {
    // Ensure we have exactly 5 letters
    if word_data.len() != 5 {
        return Err("Word must have exactly 5 letters".to_string());
    }

    // Create a Word with the right characters
    let word_str: String = word_data.iter().map(|l| l.character).collect();

    let mut word = Word::new(&word_str).map_err(|e| e.to_string())?;

    // Set the states for each letter
    for (i, letter_data) in word_data.iter().enumerate() {
        word.letter_at_mut(i)
            .set_state(convert_letter_state(&letter_data.state));
    }

    Ok(word)
}

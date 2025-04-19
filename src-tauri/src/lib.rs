pub mod data;
pub mod game_logic;
pub mod tauri;

pub use tauri::run;

const WORDS_FILE: &str = "assets/all_words.json";

/// # `LetterState`
/// Represents the state of a letter in a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterState {
    Unknown,   // Initial state
    Correct,   // Green - right letter, right position
    Misplaced, // Yellow - right letter, wrong position
    Absent,    // Gray - letter not in word
}

/// # `Letter`
/// Represents a letter in a word with its state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Letter {
    pub character: char,
    pub state: LetterState,
}

impl Letter {
    /// Create a new Letter with Unknown state
    pub fn new(c: char) -> Result<Self, &'static str> {
        if !c.is_ascii_alphabetic() {
            return Err("Character must be an ASCII letter");
        }
        Ok(Self {
            character: c.to_ascii_lowercase(),
            state: LetterState::Unknown,
        })
    }

    /// Create a new Letter with a specific state
    pub fn with_state(c: char, state: LetterState) -> Result<Self, &'static str> {
        if !c.is_ascii_alphabetic() {
            return Err("Character must be an ASCII letter");
        }
        Ok(Self {
            character: c.to_ascii_lowercase(),
            state,
        })
    }

    /// Update the state of the letter
    pub fn set_state(&mut self, state: LetterState) {
        self.state = state;
    }
}

/// # `Word`
/// Represents a word with its letters and their states.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Word {
    letters: [Letter; 5],
}

impl Word {
    /// Create a new Word from a string.
    pub fn new(word: &str) -> Result<Self, &'static str> {
        if word.len() != 5 {
            return Err("Word must be exactly 5 letters");
        }

        let letters: Result<[Letter; 5], &'static str> = word
            .chars()
            .map(Letter::new)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "Failed to convert to array");

        // Map the successful array into a Word struct
        letters.map(|l| Word { letters: l })
    }

    /// # `letter_at`
    /// Returns a reference to the letter at the given position.
    ///
    /// ## Arguments
    /// * `pos` - The position of the letter to retrieve.
    ///
    /// ## Returns
    /// * `&Letter` - A reference to the letter at the given position.
    #[must_use]
    pub fn letter_at(&self, pos: usize) -> &Letter {
        &self.letters[pos]
    }

    /// # `letter_at_mut`
    /// Returns a mutable reference to the letter at the given position.
    ///
    /// ## Arguments
    /// * `pos` - The position of the letter to retrieve.
    ///
    /// ## Returns
    /// * `&mut Letter` - A mutable reference to the letter at the given position.
    pub fn letter_at_mut(&mut self, pos: usize) -> &mut Letter {
        &mut self.letters[pos]
    }
}

/// # `load_words`
/// Loads the list of words from the JSON file.
///
/// ## Returns
/// * `Vec<String>` - A vector of words loaded from the JSON file.
#[must_use]
pub fn load_words() -> Vec<String> {
    let file = std::fs::File::open(WORDS_FILE).expect("Failed to open words file");
    let reader = std::io::BufReader::new(file);

    serde_json::from_reader(reader).expect("Failed to parse words file")
}

/// # `create_pattern`
/// Helper function to create a pattern with specific states
///
/// ## Arguments
/// * `word` - The word to create the pattern for.
/// * `states` - The states of each letter in the word.
///
/// ## Returns
/// * `Word` - The created pattern.
#[must_use]
pub fn create_pattern(word: &str, states: Vec<(usize, LetterState)>) -> Word {
    let mut pattern = Word::new(word).unwrap();
    for (pos, state) in states {
        pattern.letter_at_mut(pos).set_state(state);
    }

    pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_creation() {
        assert!(Word::new("hello").is_ok());
        assert!(Word::new("hi").is_err());
        assert!(Word::new("toolong").is_err());
        assert!(Word::new("12345").is_err());
    }

    #[test]
    fn test_letter_states() {
        let mut word = Word::new("hello").unwrap();
        word.letter_at_mut(0).set_state(LetterState::Correct);
        word.letter_at_mut(1).set_state(LetterState::Misplaced);
        word.letter_at_mut(2).set_state(LetterState::Absent);

        assert_eq!(word.letter_at(0).state, LetterState::Correct);
        assert_eq!(word.letter_at(1).state, LetterState::Misplaced);
        assert_eq!(word.letter_at(2).state, LetterState::Absent);
    }
}

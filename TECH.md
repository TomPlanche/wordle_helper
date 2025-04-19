# Technical Details: Wordle Helper

## 🏗️ Architecture

This project demonstrates a modern desktop application architecture using Tauri, combining:

- **Frontend**: SvelteKit 2.x with TypeScript and SCSS
- **Backend**: Rust for efficient word filtering algorithms
- **Bridge**: Tauri's IPC system connecting the two worlds

## 💻 Frontend Implementation

### Technology Stack

- **SvelteKit 2.x**: For the UI framework with static adapter
- **TypeScript**: For type safety
- **Svelte 5 Features**: Using new reactivity primitives ($state, $derived)
- **SCSS**: For styling with a Solarized Dark theme

### Key Components

The `WordGuess.svelte` component is the core UI element that:
- Handles user input for 5-letter words
- Manages the cycling of letter states (unknown → correct → misplaced → absent)
- Provides keyboard navigation between letter boxes
- Updates the parent component with new letter states

### State Management

All state is managed using Svelte 5's new reactivity primitives:

```typescript
// Main page state
let guesses = $state<TWord[]>([/* Initial state */]);
let possibleMatches = $state<string[]>([]);

// Derived calculations
let allFilled = $derived(guesses.every(/*...*/));
```

## 🦀 Backend Implementation

### Core Data Model

The backend defines a domain-specific model for the Wordle game:

```rust
// Letter with its state
pub struct Letter {
    pub character: char,
    pub state: LetterState,
}

// Letter state (unknown, correct, misplaced, absent)
pub enum LetterState {
    Unknown, Correct, Misplaced, Absent
}

// A 5-letter word
pub struct Word {
    letters: [Letter; 5],
}
```

### Word Filtering Algorithm

The core algorithm in `game_logic.rs` handles:

1. **Pattern Matching**: Determining if a word matches a given pattern with:
   - Correct letters in the right positions
   - Misplaced letters that exist somewhere else
   - Absent letters that shouldn't appear

2. **Special Cases**:
   - Handling duplicate letters correctly (matching Wordle's rules)
   - All-misplaced patterns
   - All-absent patterns

3. **Filtering**: Applying multiple patterns to filter the word dictionary efficiently

### Word Dictionary

The application includes a pre-loaded dictionary of valid 5-letter words in JSON format:

```rust
pub fn load_words() -> Vec<String> {
    let file = std::fs::File::open(WORDS_FILE).expect("Failed to open words file");
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse words file")
}
```

## 🔄 Frontend-Backend Communication

Tauri bridges the frontend and backend through command invocation:

```typescript
// Frontend (TypeScript)
invoke('filter_word_list', {patterns: guesses})
  .then((possibleWords) => {
    possibleMatches = possibleWords as string[];
  });
```

```rust
// Backend (Rust)
#[tauri::command]
pub fn filter_word_list(patterns: Vec<WordData>) -> Result<Vec<String>, String> {
    // Convert frontend data to backend models
    let converted_patterns = patterns.iter().map(convert_word_data).collect();

    // Process and return filtered words
    let all_words = load_words();
    Ok(filter_words(&all_words, &patterns))
}
```

Data conversion handles the mapping between frontend and backend representations:

```rust
// Convert frontend LetterState string to backend LetterState enum
fn convert_letter_state(state: &str) -> LetterState {
    match state {
        "correct" => LetterState::Correct,
        "misplaced" => LetterState::Misplaced,
        "absent" => LetterState::Absent,
        _ => LetterState::Unknown,
    }
}
```

## 🧪 Testing Approach

The project includes comprehensive tests for the Rust backend:

1. **Unit Tests**: Test individual functions and methods
2. **Pattern Matching Tests**: Verify the pattern matching logic
3. **Edge Cases**: Test special cases like duplicate letters
4. **Filter Tests**: Verify the word filtering functionality

Example test:
```rust
#[test]
fn test_pattern_matching_duplicate_letters() {
    // Test duplicate letters with different states
    let word = Word::new("paper").unwrap();
    let pattern = create_pattern(
        "happy",
        vec![
            (0, LetterState::Absent),  // 'h' absent
            (1, LetterState::Correct), // 'a' correct
            (2, LetterState::Correct), // 'p' correct
            (3, LetterState::Absent),  // 'p' absent
            (4, LetterState::Absent),  // 'y' absent
        ],
    );
    assert!(word.matches_pattern(&pattern));
}
```

## 🛠️ Build and Development Tools

- **Tauri**: For creating the cross-platform desktop app
- **pnpm**: Package management with workspaces
- **Biome**: Code formatting and linting (JavaScript/TypeScript)
- **Vite**: Frontend bundling and development server
- **Tauri CLI**: Building and packaging the application

## 📝 Data Validation

- **Frontend**: Uses Zod schemas for runtime validation
- **Backend**: Uses Rust's type system and explicit validation

```typescript
// Frontend validation with Zod
import {z} from "zod";

const LetterStateSchema = z.enum(["unknown", "correct", "misplaced", "absent"]);
const CharacterSchema = z
  .string()
  .min(1, {message: "Letter must be at least 1 character"})
  .max(1, {message: "Letter must be at most 1 character"})
  .transform((val) => val.toLowerCase())
  .regex(/^[a-z]$/, {message: "Letter must be a single letter"});

const FinalLetterStateSchema = z.enum(["correct", "misplaced", "absent"]);
export const FinalLetterSchema = z.object({
  character: CharacterSchema,
  state: FinalLetterStateSchema,
});
```

```rust
// Backend validation
pub fn new(word: &str) -> Result<Self, &'static str> {
    if word.len() != 5 {
        return Err("Word must be exactly 5 letters");
    }
    // Additional validation...
}
```

## 🚀 Performance Considerations

- **Efficient Filtering**: The Rust backend provides fast word filtering
- **Minimal Dependencies**: Keeping the bundle size small
- **Static Generation**: Frontend is pre-rendered for instant loading

# 🎮 Wordle Helper

A modern desktop application to help you solve Wordle puzzles. Enter your guesses and get suggestions for your next move!

## ✨ Features

- 🔍 Get suggestions for your next Wordle guess
- 🎯 Input your current guesses with color coding
- 🔄 Toggle letter states (correct, misplaced, absent) with a simple click
- 📊 See all possible matching words based on your current knowledge
- 🖥️ Cross-platform desktop app (Windows, macOS, Linux)

## 🛠️ Technologies

- **[Tauri](https://tauri.app/)**: Creating efficient, secure, and cross-platform desktop apps
- **[SvelteKit](https://kit.svelte.dev/)**: Frontend framework
- **[TypeScript](https://www.typescriptlang.org/)**: Type-safe JavaScript
- **[Rust](https://www.rust-lang.org/)**: Backend logic and word filtering
- **[Biome](https://biomejs.dev/)**: Code formatting and linting

> [!NOTE]
> See the [TECH.md](TECH.md) for more details on the technologies and the implementation details.

## 🚀 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (v16 or later)
- [pnpm](https://pnpm.io/installation)
- Platform-specific dependencies for Tauri (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/wordle-helper.git
   cd wordle-helper
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Run in development mode:
   ```bash
   pnpm tauri dev
   ```

4. Build for production:
   ```bash
   pnpm tauri build
   ```

## 📖 How to Use

1. **Enter Your Guesses**: Type your Wordle guesses in the input fields.

2. **Set Letter States**: Click on a letter to cycle through states:
   - **Absent (Grey)**: Letter is not in the word
   - **Misplaced (Yellow)**: Letter is in the word but in the wrong position  
   - **Correct (Green)**: Letter is in the correct position

3. **Add More Guesses**: Click "Add Guess" to input additional guesses (up to 4).

4. **Get Suggestions**: Click "Search" to see possible matching words based on your guesses.

5. **Review Results**: The app will show you all possible words that match your constraints, along with the total count.

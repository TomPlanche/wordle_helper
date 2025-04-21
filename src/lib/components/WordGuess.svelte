<script lang="ts">
  // Props
  import type {TWord} from "$lib/types";

  const {key, word = $bindable<TWord>()} = $props<{
    key: number;
    word: TWord;
  }>();

  // States
  const letters = $state<TWord>(word);

  // Handle input in letter boxes
  const handleInput = (index: number, event: Event) => {
    const input = event.target as HTMLInputElement;
    const value = input.value.toUpperCase();

    // Make sure we only get a single letter
    if (value.length > 0) {
      const letter = value[value.length - 1];
      if (/^[A-Z]$/.test(letter)) {
        word[index].character = letter; // Update the bound word

        // Auto-advance to the next input if possible
        if (index < 4) {
          const nextInput = document.querySelector(
            `.letter-input[data-index="${index + 1}"][data-key="${key}"]`,
          ) as HTMLInputElement;
          if (nextInput) nextInput.focus();
        }
      } else {
        // Not a valid letter, reset
        input.value = "";
      }
    }
  };

  // Cycle through letter states on click
  const cycleLetterState = (index: number) => {
    const stateOrder: Array<"unknown" | "correct" | "misplaced" | "absent"> = [
      "unknown",
      "correct",
      "misplaced",
      "absent",
    ];

    const currentState = word[index].state;
    const currentIndex = stateOrder.indexOf(currentState);
    const nextIndex = (currentIndex + 1) % stateOrder.length;

    word[index].state = stateOrder[nextIndex]; // Update the bound word
  };

  // Handle keyboard navigation
  const handleKeyDown = (index: number, event: KeyboardEvent) => {
    if (event.key === "ArrowRight" && index < 4) {
      const nextInput = document.querySelector(
        `.letter-input[data-index="${index + 1}"][data-key="${key}"]`,
      ) as HTMLInputElement;
      if (nextInput) nextInput.focus();
    } else if (event.key === "ArrowLeft" && index > 0) {
      const prevInput = document.querySelector(
        `.letter-input[data-index="${index - 1}"][data-key="${key}"]`,
      ) as HTMLInputElement;
      if (prevInput) prevInput.focus();
    } else if (
      event.key === "Backspace" &&
      index > 0 &&
      !letters[index].character
    ) {
      // If the current box is empty and backspace was pressed, go to the previous box
      const prevInput = document.querySelector(
        `.letter-input[data-index="${index - 1}"][data-key="${key}"]`,
      ) as HTMLInputElement;
      if (prevInput) {
        prevInput.focus();
        // Optionally clear the previous input
        // letters[index - 1].character = '';
      }
    }
  };
</script>

<div class="word-guess">
  {#each word as letter, index}
    <div class="letter-box"
         class:correct={letter.state === 'correct'}
         class:misplaced={letter.state === 'misplaced'}
         class:absent={letter.state === 'absent'}
    >
      <input
          type="text"
          class="letter-input"
          maxlength="1"
          value={letter.character}
          oninput={(e) => handleInput(index, e)}
          onkeydown={(e) => handleKeyDown(index, e)}
          onclick={() => cycleLetterState(index)}
          data-index={index}
          data-key={key}
      />
    </div>
  {/each}
</div>

<style lang="scss">
  .word-guess {
    display: flex;
    gap: 0.5rem;
  }

  .letter-box {
    width: 3.5rem;
    height: 3.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #073642;
    border: 2px solid #586e75;
    border-radius: 4px;
    transition: all 0.2s;

    &.correct {
      background-color: #2aa198;
      border-color: #2aa198;
    }

    &.misplaced {
      background-color: #b58900;
      border-color: #b58900;
    }

    &.absent {
      background-color: #657b83;
      border-color: #657b83;
    }
  }

  .letter-input {
    width: 100%;
    height: 100%;
    background: transparent;
    border: none;
    color: #eee8d5;
    font-size: 1.8rem;
    text-align: center;
    text-transform: uppercase;
    font-weight: bold;
    cursor: pointer;

    &:focus {
      outline: none;
    }
  }
</style>
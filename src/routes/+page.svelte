<script lang="ts">
  import "../main.scss";
  import WordGuess from "../lib/components/WordGuess.svelte";
  import {invoke} from "@tauri-apps/api/core";
  import {GuessesSchema, type TWord} from "$lib/types";

  // States
  let guesses = $state<TWord[]>([
    [
      {character: "", state: "unknown"},
      {character: "", state: "unknown"},
      {character: "", state: "unknown"},
      {character: "", state: "unknown"},
      {character: "", state: "unknown"},
    ],
  ]);
  let possibleMatches = $state<string[]>([]);

  const allFilled: boolean = $derived(
    guesses.every((guess) => guess.every((letter) => letter.character !== "")),
  );
  const allAreNotUnknown: boolean = $derived(
    guesses.every((guess) => guess.every((letter) => letter.state !== "unknown")),
  );

  const addGuess = () => {
    if (!allFilled) {
      alert("Please fill all letter boxes before adding a new guess.");
      return;
    }

    if (!allAreNotUnknown) {
      alert("Please set all letter states before adding a new guess.");
      return;
    }

    guesses = [
      ...guesses,
      [
        {character: "", state: "unknown"},
        {character: "", state: "unknown"},
        {character: "", state: "unknown"},
        {character: "", state: "unknown"},
        {character: "", state: "unknown"},
      ],
    ];
  };

  // Delete a guess by index
  const deleteGuess = (index: number) => {
    guesses = guesses.filter((_, i) => i !== index);
  };

  const handleSubmit = () => {
    if (!allFilled) {
      alert("Please fill all letter boxes before submitting.");
      return;
    }

    if (!allAreNotUnknown) {
      alert("Please set all letter states before submitting.");
      return;
    }

    const verif = GuessesSchema.safeParse(guesses);

    if (verif.error) {
      console.error("Invalid guesses:", verif.error);
      alert("Invalid guesses. Please check your input.");
    }

    invoke("filter_word_list_command", {patterns: verif.data}).then(
      (possibleWords) => {
        possibleMatches = possibleWords as string[];
      },
    );
  };

  const addWordAsGuess = (word: string) => {
    if (guesses.length >= 4) {
      alert("Maximum number of guesses reached (4).");
      return;
    }

    const newGuess: TWord = word.split("").map((char) => ({
      character: char,
      state: "unknown",
    }));

    guesses = [...guesses, newGuess];
  };
</script>

<main>
  <h1>Welcome to Tom's Wordle helper!</h1>

  <div class="guesses">
    {#each guesses as _, index}
      <div class="guess-container">
        <WordGuess key={index} bind:word={guesses[index]}/>

        {#if index !== 0 && guesses.length < 1}
          <button
              class="delete-guess-btn"
              onclick={() => deleteGuess(index)}
              aria-label="Delete guess"
          >
            <svg
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
              <path
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                  d="M17 5V4C17 2.89543 16.1046 2 15 2H9C7.89543 2 7 2.89543 7 4V5H4C3.44772 5 3 5.44772 3 6C3 6.55228 3.44772 7 4 7H5V18C5 19.6569 6.34315 21 8 21H16C17.6569 21 19 19.6569 19 18V7H20C20.5523 7 21 6.55228 21 6C21 5.44772 20.5523 5 20 5H17ZM15 4H9V5H15V4ZM17 7H7V18C7 18.5523 7.44772 19 8 19H16C16.5523 19 17 18.5523 17 18V7Z"
                  fill="currentColor"
              />
              <path d="M9 9H11V17H9V9Z" fill="currentColor"/>
              <path d="M13 9H15V17H13V9Z" fill="currentColor"/>
            </svg>
          </button>
        {/if}
      </div>
    {/each}

    {#if guesses.length < 4}
      <button
          class="add-guess-btn"
          onclick={addGuess}
      >Add Guess
      </button>
    {/if}

    <button
        class="search-btn"
        onclick={handleSubmit}
    >
      Search
    </button>
  </div>

  {#if possibleMatches.length > 0}
    <div class="results">
      <h2>Possible matches:</h2>
      <ul>
        {#each possibleMatches as word}
          <li>
            <button
                class="add-proposed-guess-btn"
                onclick={() => addWordAsGuess(word)}
            >
              {word}
            </button>
          </li>
        {/each}
      </ul>
    </div>
  {/if}
</main>

<style lang="scss">
  :global(*) {
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }

  .guesses {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin: 2rem 0;
    align-items: center;
  }

  .guess-container {
    display: flex;
    align-items: center;
    gap: 1rem;
    position: relative;
  }


  button:not(.add-proposed-guess-btn) {
    font-family: "Monorama", monospace;
    font-size: 1rem;
    font-weight: 100;
    text-transform: uppercase;
    cursor: pointer;
    transition: background-color 0.2s;
    padding: 0.75rem 1.5rem;
    border-radius: 4px;
    border: none;

    &.delete-guess-btn {
      padding: .5rem;
      background-color: #dc322f;
      color: white;
      border-radius: 8px;
      font-size: 1.5rem;

      &:hover {
        background-color: #cb3837;

        svg {
          scale: 1.2;
        }
      }

      svg {
        width: 1.5rem;
        height: 1.5rem;

        transition: background-color 0.2s, scale 0.2s;
      }
    }

    &.add-guess-btn {
      background-color: #268bd2;
      color: white;

      &:hover {
        background-color: #1e6ea7;
      }
    }

    &.search-btn {
      background-color: #2aa198;
      color: white;
      border: none;

      &:hover {
        background-color: #219186;
      }
    }
  }

  .results {
    margin-top: 2rem;
    text-align: center;

    ul {
      list-style: none;
      padding: 0;
      display: flex;
      flex-wrap: wrap;
      gap: 0.5rem;
      justify-content: center;
      margin-top: 1rem;

      li {
        background-color: #073642;
        color: #eee8d5;
        padding: 0.5rem 1rem;
        border-radius: 4px;
        cursor: pointer;
        transition: all 0.2s;
        user-select: none;

        &:hover {
          background-color: #268bd2;
          transform: scale(1.05);
        }

        &:active {
          transform: scale(0.95);
        }
      }
    }
  }
</style>
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use wordle_helper_lib::load_words;

fn main() {
    let words = load_words();
    println!("Loaded {} words", words.len());

    wordle_helper_lib::run();
}

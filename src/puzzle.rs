use std::iter::Iterator;
use std::collections::HashMap;
use std::hash::Hash;
use rand::Rng;

#[derive(Debug)]
pub struct Puzzle {
    pub base_word : String,
    pub letter : char,
    pub grid : [[char;3];3],
    pub words : Vec<String>
}

impl Puzzle {
    pub fn grid_string(self : &Puzzle) -> String {
        format!("{}{}{}\n{}{}{}\n{}{}{}",
            self.grid[0][0], self.grid[0][1], self.grid[0][2],
            self.grid[1][0], self.grid[1][1], self.grid[1][2],
            self.grid[2][0], self.grid[2][1], self.grid[2][2]
            )
    }
}

pub fn create(words : &[&str], puzzle_word : &str, letter : &char) -> Puzzle {
    let base_word = String::from(puzzle_word);
    let letter = *letter;

    let puzzle_freq = char_freq(&puzzle_word);

    let final_words = words
        .iter()
        .filter(|word| word.chars().any(|c| c == letter))
        .filter(|&word| is_frequency_subset(&puzzle_freq, &char_freq(&word)))
        .map(|&str| String::from(str))
        .collect::<Vec<_>>();

    let mut other_letters = puzzle_word
        .chars()
        .filter(|&c| c != letter)
        .collect::<Vec<_>>();

    while other_letters.len() < 8 {
        other_letters.push(letter);
    }

    ::rand::thread_rng().shuffle(&mut other_letters);

    Puzzle {
        base_word,
        letter,
        grid: [
            [other_letters[0], other_letters[1], other_letters[2]],
            [other_letters[3], letter, other_letters[4]],
            [other_letters[5], other_letters[6], other_letters[7]]],
        words: final_words
    }
}

fn char_freq(string : &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for char in string.chars() {
        *map.entry(char).or_insert(0) += 1;
    }

    map
}

fn is_frequency_subset<T : Eq + Hash>(superset : &HashMap<T, usize>, compared : &HashMap<T, usize>) -> bool {
    for key in compared.keys() {
        if !superset.contains_key(key) {
            return false;
        }
    }

    for key in compared.keys() {
        if superset[key] < compared[key] {
            return false;
        }
    }

    true
}
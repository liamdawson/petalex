#![feature(ascii_ctype)]
#[macro_use] extern crate clap;
extern crate rand;

use rand::Rng;

mod dict;
mod random;
mod puzzle;

fn main() {
    let app = clap_app!(app =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (name: crate_name!())
        (@setting SubcommandRequiredElseHelp)
        (@subcommand check => 
            (about: "Check if a word is in the puzzle")
            (@group puzzle_source => 
                (@attributes +required)
                (@arg SEED: -s --seed +takes_value "puzzle seed for this wordlist version")
                (@arg LETTERS: -l --letters +takes_value "puzzle letters, where the first is the center"))
            (@arg WORD: +required "word to check"))
        (@subcommand generate => 
            (about: "Generate a new puzzle")
            (@arg SEED: -s --seed +takes_value "previous seed to reuse"))
        (@subcommand answers => 
            (about: "Provide answers for an existing puzzle")
            (@group puzzle_source => 
                (@attributes +required)
                (@arg SEED: -s --seed +takes_value "puzzle seed for this wordlist version")
                (@arg LETTERS: -l --letters +takes_value "puzzle letters, where the first is the center")))
    );

    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("generate") => {
            let submatches = matches.subcommand_matches("generate").unwrap();
            let seed = value_t!(submatches, "SEED", u64).unwrap_or(random::make_seed());

            let puzzle = puzzle_from_seed(seed);

            println!("\n{}\n", puzzle.grid_string());
            println!("total words: {}", puzzle.words.len());
            println!("seed: {}", seed);
        },
        Some("answers") => {
            let submatches = matches.subcommand_matches("answers").unwrap();

            let puzzle = match submatches.is_present("SEED") {
                true => puzzle_from_seed(value_t!(submatches, "SEED", u64).unwrap_or_else(|e| e.exit())),
                false => {
                    let letters = submatches.value_of("LETTERS").unwrap().to_lowercase();
                    if letters.chars().count() != 9 {
                        panic!("expected letters to be 9 characters");
                    }

                    puzzle_from_letters(letters.as_str())}
            };

            println!("words:\n");

            for word in puzzle.words {
                println!("{}", word);
            }
        },
        Some("check") => {
            let submatches = matches.subcommand_matches("check").unwrap();
            let check_word = submatches.value_of("WORD").unwrap().to_lowercase();

            let puzzle = match submatches.is_present("SEED") {
                true => puzzle_from_seed(value_t!(submatches, "SEED", u64).unwrap_or_else(|e| e.exit())),
                false => {
                    let letters = submatches.value_of("LETTERS").unwrap().to_lowercase();
                    if letters.chars().count() != 9 {
                        panic!("expected letters to be 9 characters");
                    }

                    puzzle_from_letters(letters.as_str())}
            };

            let matched = puzzle.words.into_iter().any(|w| w.to_lowercase() == check_word);

            println!("{} was{} in the puzzle", check_word, if matched { "" } else { " not" });
        },
        _ => unreachable!()
    }
}

fn puzzle_from_letters(letters : &str) -> puzzle::Puzzle {
    let letter : char = letters.chars().take(1).last().unwrap();
    puzzle::create(&dict::british_words().collect::<Vec<_>>(), letters, &letter)
}

fn puzzle_from_seed(seed : u64) -> puzzle::Puzzle {
    let mut rand = random::make_random(seed);

    let puzzle_words = dict::british_words().filter(|w| w.chars().count() == 9).collect::<Vec<_>>();
    let words = dict::british_words().collect::<Vec<_>>();

    let word_count = puzzle_words.len();
    let word_index = rand.gen_range(0, word_count);
    let letter_index = rand.gen_range(0, 8);

    let word = puzzle_words[word_index];
    let letter = word.chars().skip(letter_index).take(1).last().unwrap();

    puzzle::create(&words, word, &letter)
}

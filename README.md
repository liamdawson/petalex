# petalex (PETAL lEXicography)

Builds petal puzzles, also know as wheel words, target words puzzles, or
nine letter block puzzles.

## Puzzle Details

Petal puzzles, like the one shown below, are formed as an anagram of a
nine letter word, with one letter shown centrally. The puzzle is considered
"solved" when the player finds the nine letter word, as well as a certain
number of other words that meet the following criteria:

* minimum of four letters long
* only use letters from the puzzle
* do not use any letter in the puzzle more times than it appears
* use the central letter

## Sample Puzzle

|     |     |     |
|-----|-----|-----|
|  E  |  I  |  B  |
|  O  |**R**|  S  |
|  F  |  T  |  T  |

Target: 50 words, good; 75 words, very good; 90 words, excellent; 101 words, perfect;

### Examples

Example valid words:

* FROST
* TORT

Example invalid words:

* SHORT
    The puzzle does not contain a "H"
* SORTS
    The puzzle only contains one "S"
* SOFT
    The central letter "R" must be used
* ROT
    Minimum length of 4 letters

Word list from Scowl 7.1 (British + English words + most common variants, up to 60)
pub const BRITISH : &'static str = include_str!("words/british");

pub fn british_words() -> ::std::str::Split<'static, &'static str> {
    BRITISH
        .split("\n")
}
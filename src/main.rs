use std::collections::HashSet;
use rayon::prelude::*;

fn main() {
    let words = include_str!("../words_alpha.txt");

    let dict = words
        .par_lines()
        .collect::<HashSet<_>>();
    
    let mut results = words
        .par_lines()
        .filter(|word| word.len() & 1 == 0)
        .map(deinterleave)
        .filter(|(wr, od, _)| dict.contains(wr.as_str()) && dict.contains(od.as_str()))
        .collect::<Vec<_>>();

    results.par_sort_by(|(a, _, _), (b, _, _)| a.len().cmp(&b.len()));

    for (wr, od, word) in results {
        println!("{} + {} = {}", wr, od, word);
    }
}

fn deinterleave(word: &str) -> (String, String, &str) {
    (word.chars().step_by(2).collect(),
     word.chars().skip(1).step_by(2).collect(),
     word)
}
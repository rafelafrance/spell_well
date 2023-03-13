use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct WordFreq {
    word: String,
    freq: u32,
}

pub struct WordFreqs {
    pub word_freqs: Vec<WordFreq>,
}

impl WordFreqs {
    pub fn new() -> Self {
        WordFreqs {
            word_freqs: Vec::new(),
        }
    }

    pub fn read_csv(&mut self, vocab_csv: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut reader =
            csv::Reader::from_path(vocab_csv).expect("Could not read the vocabulary CSV file.");

        for row in reader.deserialize() {
            let freq: WordFreq = row.expect("Could not parse the vocabulary CSV file.");

            self.word_freqs.push(freq);
        }
        Ok(())
    }
}

struct Spelling<'a> {
    word: &'a WordFreq,
    dist: u8,
}

impl<'a> Spelling<'a> {
    pub fn new(word: &'a WordFreq, dist: u8) -> Self {
        Spelling { word, dist }
    }
}

pub struct SpellWell<'a> {
    spell: HashMap<String, Spelling<'a>>,
}

impl<'a> SpellWell<'a> {
    pub fn new() -> Self {
        SpellWell {
            spell: HashMap::new(),
        }
    }

    pub fn add_word_list(&'a mut self, word_freqs: &'a WordFreqs, deletes: u8) {
        for word_freq in word_freqs.word_freqs.iter() {
            self.add_spelling(&word_freq.word, word_freq, 0);
            self.delete_chars(&word_freq.word, word_freq, deletes, 1);
        }
    }

    fn delete_chars(&mut self, word: &str, word_freq: &'a WordFreq, deletes: u8, dist: u8) {
        if dist > deletes {
            return;
        }
        for (i, ch) in word.char_indices() {
            let deleted = delete_char(word, i, ch);
            self.add_spelling(&word, word_freq, dist);
            self.delete_chars(&deleted, word_freq, deletes, dist + 1);
        }
    }

    fn add_spelling(&mut self, word: &str, word_freq: &'a WordFreq, dist: u8) {
        match self.spell.get(word) {
            Some(old) => {
                if old.dist > dist || (old.dist == dist && old.word.freq < word_freq.freq) {
                    self.spell
                        .insert(word.to_string(), Spelling::new(word_freq, dist));
                }
            }
            None => {
                self.spell
                    .insert(word.to_string(), Spelling::new(word_freq, dist));
            }
        }
    }

    pub fn correct(&self, word: &'a str) -> &str {
        word
    }
}

fn delete_char(word: &str, i: usize, ch: char) -> String {
    let mut delete = word.to_string();
    delete.replace_range(i..i + ch.len_utf8(), "");
    delete
}

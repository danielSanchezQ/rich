use lazy_static::lazy_static;
use regex::{Match, Regex};

use super::iter::loop_last;
use cells::{cell_len, chop_cells, DEFAULT_CELL_LEN_CACHE};

lazy_static! {
    pub static ref WORDS: Regex = Regex::new(r#"\s*\S+\s*"#).unwrap();
}

struct WordsMatchIterator<'a> {
    text: &'a str,
    current_position: usize,
}

impl<'a> WordsMatchIterator<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            current_position: 0,
        }
    }
}

impl<'a> Iterator for WordsMatchIterator<'a> {
    type Item = (usize, usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        match WORDS.find_at(self.text, self.current_position) {
            None => None,
            Some(word) => {
                self.current_position = word.end();
                Some((word.start(), word.end(), word.as_str()))
            }
        }
    }
}

pub fn words<'a>(text: &'a str) -> impl Iterator<Item = (usize, usize, &'a str)> + 'a {
    WordsMatchIterator::new(text)
}

pub fn divide_line(text: &str, width: usize, fold: Option<bool>) -> Vec<usize> {
    let mut divides: Vec<usize> = Vec::new();
    let fold = fold.unwrap_or(true);
    let mut line_position = 0;
    for (mut start, end, word) in words(text) {
        let mut len_cache = DEFAULT_CELL_LEN_CACHE.lock().unwrap();
        let word_len = cell_len(word.trim_end(), &mut len_cache);
        if line_position + word_len > width {
            if word_len > width {
                if fold {
                    for (last, line) in loop_last(chop_cells(word, width, line_position)) {
                        if last {
                            line_position = cell_len(&line, &mut len_cache);
                        } else {
                            start += line.len();
                            divides.push(start);
                        }
                    }
                } else {
                    if start > 0 {
                        divides.push(start);
                    }
                    line_position = cell_len(word, &mut len_cache);
                }
            } else if line_position > 0 && start > 0 {
                divides.push(start);
                line_position = cell_len(word, &mut len_cache);
            }
        } else {
            line_position += cell_len(word, &mut len_cache);
        }
        std::mem::drop(len_cache);
    }
    divides
}

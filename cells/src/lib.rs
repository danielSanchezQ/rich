mod cell_widths;

use std::ops::Div;
use std::sync::Mutex;

use lazy_static::lazy_static;
use lru::LruCache;

pub use cell_widths::CELL_WIDTHS;

lazy_static! {
    static ref CODEPOINT_CELL_SIZE_CACHE: Mutex<LruCache<u8, usize>> =
        Mutex::new(LruCache::new(4096));
    static ref DEFAULT_CELL_LEN_CACHE: Mutex<LruCache<String, usize>> =
        Mutex::new(LruCache::new(4096));
}

/// Get the number of cells required to display text
pub fn cell_len(text: &str, cache: &mut LruCache<String, usize>) -> usize {
    let text = text.to_string();

    if let Some(cached_result) = cache.get(&text) {
        return *cached_result;
    }
    let total_size = text.as_str().chars().map(get_character_cell_size).sum();
    if text.len() <= 64 {
        cache.put(text, total_size);
    }
    total_size
}

/// Get the cell size of a character
pub fn get_character_cell_size(char: char) -> usize {
    if char.is_ascii() {
        1
    } else {
        get_codepoint_cell_size(char as u8)
    }
}

/// Get the cell size of a character
fn get_codepoint_cell_size(codepoint: u8) -> usize {
    let mut cache = CODEPOINT_CELL_SIZE_CACHE.lock().unwrap();
    if let Some(result) = cache.get(&codepoint) {
        return *result;
    }
    let table = &CELL_WIDTHS;
    let (mut lower_bound, mut upper_bound): (i32, i32) = (0, table.len() as i32 - 1);
    let mut index = (lower_bound + upper_bound).div(2);
    loop {
        let (start, end, width) = table[index as usize];
        if (codepoint as i32) < start {
            upper_bound = index - 1;
        } else if (codepoint as i32) > end {
            lower_bound = index + 1;
        } else {
            let result = if width == -1 { 0 } else { width as usize };
            cache.put(codepoint, result);
            return result;
        }
        if upper_bound < lower_bound {
            break;
        }
        index = (lower_bound + upper_bound).div(2);
    }
    1
}

/// Set the length of a string to fit within given number of cells
pub fn set_cell_size(text: &str, total: usize) -> String {
    let cell_size = cell_len(text, &mut DEFAULT_CELL_LEN_CACHE.lock().unwrap());
    if cell_size == total {
        return text.to_string();
    }
    if cell_size < total {
        return format!("{}{}", text, " ".repeat(total - cell_size));
    }

    let mut character_sizes: Vec<usize> = text.chars().map(get_character_cell_size).collect();
    let mut excess = cell_size - total;
    while excess > 0 && character_sizes.len() > 0 {
        excess -= character_sizes.pop().unwrap();
    }
    let mut text = text[..character_sizes.len()].to_string();
    if excess < 0 {
        text.push(' ');
    }
    text
}

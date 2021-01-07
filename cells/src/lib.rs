mod cell_widths;

use std::ops::Div;
use std::sync::Mutex;

use lazy_static::lazy_static;
use lru::LruCache;

pub use cell_widths::CELL_WIDTHS;

lazy_static! {
    static ref CODEPOINT_CELL_SIZE_CACHE: Mutex<LruCache<u32, usize>> =
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
        get_codepoint_cell_size(char as u32)
    }
}

/// Get the cell size of a character
fn get_codepoint_cell_size(codepoint: u32) -> usize {
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
    let mut excess = cell_size as i32 - total as i32;
    while excess > 0 && character_sizes.len() > 0 {
        excess -= character_sizes.pop().unwrap() as i32;
    }
    let mut text = text
        .chars()
        .take(character_sizes.len())
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");
    if excess < 0 {
        text.push(' ');
    }
    text
}

/// Break text in to equal (cell) length strings
pub fn chop_cells(text: &str, max_size: usize, position: usize) -> Vec<String> {
    let mut characters = text
        .chars()
        .rev()
        .map(|c| (c, get_character_cell_size(c)))
        .peekable();
    let mut total_size = position;
    let mut lines: Vec<Vec<char>> = Vec::new();
    while let Some(_) = characters.peek() {
        let (character, size) = characters.next().unwrap();
        if (total_size + size) > max_size {
            lines.push(vec![character]);
            total_size = size
        } else {
            total_size = size;
            let len = lines.len();
            lines.get_mut(len - 1).unwrap().push(character);
        }
    }
    lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_codepoint_cell_size() {
        let codepoint = '😽' as u32;
        assert_eq!(codepoint, 128573);
        assert_eq!(get_codepoint_cell_size(codepoint), 2);
    }

    #[test]
    fn test_get_character_cell_size() {
        assert_eq!(get_character_cell_size('A'), 1);
        assert_eq!(get_character_cell_size('😽'), 2);
    }

    #[test]
    fn test_set_cell_size() {
        assert_eq!(set_cell_size("foo", 2), "fo");
        assert_eq!(set_cell_size("foo", 3), "foo");
        assert_eq!(set_cell_size("foo", 4), "foo ");
        assert_eq!(set_cell_size("😽😽", 4), "😽😽");
        assert_eq!(set_cell_size("😽😽", 3), "😽 ");
        assert_eq!(set_cell_size("😽😽", 2), "😽");
        assert_eq!(set_cell_size("😽😽", 1), " ");
    }
}

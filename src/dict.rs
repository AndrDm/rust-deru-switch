use std::io::Read;

use flate2::read::GzDecoder;

use crate::keymap::Script;

/// German abbreviations are stored as they are written (`TV`, `kB`, `GmbH`)
/// and match only in that case. Every other entry is lowercase.
pub trait HasWords {
    fn has(&self, script: Script, word: &str) -> bool;
}

pub struct MapDict {
    de: Vec<String>,
    ru: Vec<String>,
}

impl MapDict {
    pub fn new(de: &[&str], ru: &[&str]) -> Self {
        Self {
            de: prepare(de),
            ru: prepare(ru),
        }
    }
}

impl HasWords for MapDict {
    fn has(&self, script: Script, normalized: &str) -> bool {
        let list = match script {
            Script::De => &self.de,
            Script::Ru => &self.ru,
        };
        list.binary_search_by(|item| item.as_str().cmp(normalized))
            .is_ok()
    }
}

pub struct StaticDict {
    de: Vec<&'static str>,
    ru: Vec<&'static str>,
}

impl StaticDict {
    pub fn load() -> Self {
        let mut de = collect(inflate(include_bytes!(concat!(env!("OUT_DIR"), "/de.txt.gz"))), is_de_word);
        let mut ru = collect(inflate(include_bytes!(concat!(env!("OUT_DIR"), "/ru.txt.gz"))), is_ru_word);
        de.extend(load_extra(
            include_str!("../dictionaries/extra-de.txt"),
            is_extra_de,
        ));
        ru.extend(load_extra(
            include_str!("../dictionaries/extra-ru.txt"),
            is_ru_word,
        ));
        de.sort_unstable();
        de.dedup();
        ru.sort_unstable();
        ru.dedup();
        Self { de, ru }
    }
}

fn inflate(bytes: &[u8]) -> &'static str {
    let mut text = String::new();
    GzDecoder::new(bytes)
        .read_to_string(&mut text)
        .expect("embedded dictionary");
    Box::leak(text.into_boxed_str())
}

impl HasWords for StaticDict {
    fn has(&self, script: Script, normalized: &str) -> bool {
        let list: &[&str] = match script {
            Script::De => &self.de,
            Script::Ru => &self.ru,
        };
        list.binary_search(&normalized).is_ok()
    }
}

fn prepare(words: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = words
        .iter()
        .map(|word| {
            if is_abbreviation(word) {
                word.to_string()
            } else {
                normalize(word)
            }
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

fn collect(text: &'static str, keep: fn(&str) -> bool) -> Vec<&'static str> {
    text.lines()
        .map(str::trim)
        .filter(|word| keep(word))
        .collect()
}

fn load_extra(text: &str, keep: fn(&str) -> bool) -> Vec<&'static str> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let word = normalize(line);
            if keep(&word) {
                Some(&*Box::leak(word.into_boxed_str()))
            } else {
                None
            }
        })
        .collect()
}

/// `build.rs` already filtered and lowercased the German list. German
/// compounds run long, so the limit is 30 letters.
fn is_de_word(word: &str) -> bool {
    (2..=30).contains(&word.chars().count()) && word.chars().all(is_de_letter)
}

/// Letters, or a run of periods. A period is the German face of `ю`.
fn is_extra_de(word: &str) -> bool {
    is_de_word(word) || ((2..=30).contains(&word.len()) && word.bytes().all(|b| b == b'.'))
}

fn is_de_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || matches!(ch, 'ä' | 'ö' | 'ü' | 'ß' | 'Ä' | 'Ö' | 'Ü')
}

/// A capital after the first letter: `TV`, `kB`, `GmbH`. Such a word is kept
/// in its own case, so `tv` typed in lowercase is not a German word.
pub fn is_abbreviation(word: &str) -> bool {
    word.chars().skip(1).any(char::is_uppercase)
}

fn is_ru_word(word: &str) -> bool {
    let len = word.chars().count();
    (2..=20).contains(&len) && word.chars().all(|ch| ('а'..='я').contains(&ch))
}

pub fn normalize(word: &str) -> String {
    let mut out = String::with_capacity(word.len());
    for ch in word.chars() {
        for lower in ch.to_lowercase() {
            out.push(if lower == 'ё' { 'е' } else { lower });
        }
    }
    out
}

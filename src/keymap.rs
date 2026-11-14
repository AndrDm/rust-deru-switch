//! Physical German QWERTZ (T1) ↔ Russian ЙЦУКЕН keys.
//!
//! Characters are paired by the key that produces them, not by alphabetical order.
//! Shifted letters follow Unicode case. Shifted punctuation is listed explicitly
//! because the two layouts disagree (`§` versus `№` and so on).
//!
//! `ß` shares its key with the Russian hyphen, so a hyphen typed in Russian is
//! part of the word. `^`, `´` and `` ` `` are dead keys in German: nothing
//! appears on screen until the next key, so they drop the unfinished word.
//! AltGr combinations never reach this table.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    De,
    Ru,
}

impl Script {
    pub fn other(self) -> Self {
        match self {
            Self::De => Self::Ru,
            Self::Ru => Self::De,
        }
    }
}

/// How a finished word was terminated. The platform replays this after a rewrite
/// so the character the user pressed still appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Term {
    Space,
    Enter,
    Tab,
    Mark(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyClass {
    /// Part of the current word. Passed through to the focused application.
    Glyph(char),
    /// Ends the word. May be swallowed and replayed after a correction.
    End(Term),
    /// Navigation and unknown keys. The unfinished word is dropped.
    Other,
}

struct KeyDef {
    scan: u16,
    de: char,
    ru: char,
    /// Shift on a letter key is the capital unless this is set (`ß` → `?`).
    de_shift: Option<char>,
    ru_shift: Option<char>,
}

struct NumDef {
    scan: u16,
    de: char,
    ru: char,
    de_shift: char,
    ru_shift: char,
}

/// Set 1 make-codes for the keys that carry a letter in at least one of the
/// two layouts.
const KEYS: &[KeyDef] = &[
    KeyDef {
        scan: 0x29,
        de: '^',
        ru: 'ё',
        de_shift: Some('°'),
        ru_shift: None,
    },
    KeyDef {
        scan: 0x10,
        de: 'q',
        ru: 'й',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x11,
        de: 'w',
        ru: 'ц',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x12,
        de: 'e',
        ru: 'у',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x13,
        de: 'r',
        ru: 'к',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x14,
        de: 't',
        ru: 'е',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x15,
        de: 'z',
        ru: 'н',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x16,
        de: 'u',
        ru: 'г',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x17,
        de: 'i',
        ru: 'ш',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x18,
        de: 'o',
        ru: 'щ',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x19,
        de: 'p',
        ru: 'з',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x1A,
        de: 'ü',
        ru: 'х',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x1B,
        de: '+',
        ru: 'ъ',
        de_shift: Some('*'),
        ru_shift: None,
    },
    KeyDef {
        scan: 0x1E,
        de: 'a',
        ru: 'ф',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x1F,
        de: 's',
        ru: 'ы',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x20,
        de: 'd',
        ru: 'в',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x21,
        de: 'f',
        ru: 'а',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x22,
        de: 'g',
        ru: 'п',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x23,
        de: 'h',
        ru: 'р',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x24,
        de: 'j',
        ru: 'о',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x25,
        de: 'k',
        ru: 'л',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x26,
        de: 'l',
        ru: 'д',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x27,
        de: 'ö',
        ru: 'ж',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x28,
        de: 'ä',
        ru: 'э',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x2C,
        de: 'y',
        ru: 'я',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x2D,
        de: 'x',
        ru: 'ч',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x2E,
        de: 'c',
        ru: 'с',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x2F,
        de: 'v',
        ru: 'м',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x30,
        de: 'b',
        ru: 'и',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x31,
        de: 'n',
        ru: 'т',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x32,
        de: 'm',
        ru: 'б',
        de_shift: None,
        ru_shift: None,
    },
    KeyDef {
        scan: 0x33,
        de: ',',
        ru: ',',
        de_shift: Some(';'),
        ru_shift: Some(';'),
    },
    KeyDef {
        scan: 0x34,
        de: '.',
        ru: '.',
        de_shift: Some(':'),
        ru_shift: Some(':'),
    },
    KeyDef {
        scan: 0x35,
        de: '-',
        ru: '-',
        de_shift: Some('_'),
        ru_shift: Some('_'),
    },
    KeyDef {
        scan: 0x0C,
        de: 'ß',
        ru: 'ь',
        de_shift: Some('?'),
        ru_shift: Some('?'),
    },
];

const NUMS: &[NumDef] = &[
    NumDef {
        scan: 0x02,
        de: '1',
        ru: '1',
        de_shift: '!',
        ru_shift: '!',
    },
    NumDef {
        scan: 0x03,
        de: '2',
        ru: '2',
        de_shift: '"',
        ru_shift: '"',
    },
    NumDef {
        scan: 0x04,
        de: '3',
        ru: '3',
        de_shift: '§',
        ru_shift: '№',
    },
    NumDef {
        scan: 0x05,
        de: '4',
        ru: '4',
        de_shift: '$',
        ru_shift: ';',
    },
    NumDef {
        scan: 0x06,
        de: '5',
        ru: '5',
        de_shift: '%',
        ru_shift: '%',
    },
    NumDef {
        scan: 0x07,
        de: '6',
        ru: '6',
        de_shift: '&',
        ru_shift: ':',
    },
    NumDef {
        scan: 0x08,
        de: '7',
        ru: '7',
        de_shift: '/',
        ru_shift: '/',
    },
    NumDef {
        scan: 0x09,
        de: '8',
        ru: '8',
        de_shift: '(',
        ru_shift: '(',
    },
    NumDef {
        scan: 0x0A,
        de: '9',
        ru: '9',
        de_shift: ')',
        ru_shift: ')',
    },
    NumDef {
        scan: 0x0B,
        de: '0',
        ru: '0',
        de_shift: '=',
        ru_shift: '=',
    },
    NumDef {
        scan: 0x0D,
        de: '´',
        ru: 'ю',
        de_shift: '`',
        ru_shift: 'Ю',
    },
];

/// A physical key and whether Shift must be held to produce a character.
/// `code` is the AT set-1 scan code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chord {
    pub code: u16,
    pub shift: bool,
}

pub fn chord_for(script: Script, ch: char) -> Option<Chord> {
    let lower = ch.to_lowercase().next()?;
    let letter = ch.is_alphabetic();
    for key in KEYS {
        let (base, shifted) = match script {
            Script::De => (key.de, key.de_shift),
            Script::Ru => (key.ru, key.ru_shift),
        };
        if letter && base == lower {
            return Some(Chord {
                code: key.scan,
                shift: ch.is_uppercase(),
            });
        }
        if !letter && base == ch {
            return Some(Chord {
                code: key.scan,
                shift: false,
            });
        }
        if !letter && shifted == Some(ch) {
            return Some(Chord {
                code: key.scan,
                shift: true,
            });
        }
    }
    for num in NUMS {
        let (plain, shifted) = match script {
            Script::De => (num.de, num.de_shift),
            Script::Ru => (num.ru, num.ru_shift),
        };
        if plain == ch {
            return Some(Chord {
                code: num.scan,
                shift: false,
            });
        }
        if shifted == ch {
            return Some(Chord {
                code: num.scan,
                shift: true,
            });
        }
    }
    None
}

pub fn classify(scan: u16, shift: bool, caps: bool, script: Script) -> KeyClass {
    let scan = scan & 0xFF;
    if script == Script::De && is_dead_key(scan, shift) {
        return KeyClass::Other;
    }
    if let Some(def) = KEYS.iter().find(|k| k.scan == scan) {
        return class_letter_key(def, shift, caps, script);
    }
    if let Some(def) = NUMS.iter().find(|k| k.scan == scan) {
        let ch = match (script, shift) {
            (Script::De, false) => def.de,
            (Script::De, true) => def.de_shift,
            (Script::Ru, false) => def.ru,
            (Script::Ru, true) => def.ru_shift,
        };
        return KeyClass::End(Term::Mark(ch));
    }
    KeyClass::Other
}

fn class_letter_key(def: &KeyDef, shift: bool, caps: bool, script: Script) -> KeyClass {
    let base = match script {
        Script::De => def.de,
        Script::Ru => def.ru,
    };
    let shifted = match script {
        Script::De => def.de_shift,
        Script::Ru => def.ru_shift,
    };
    let produced = if base.is_alphabetic() {
        match shifted {
            // `ß`: Shift types a mark and Caps Lock leaves the letter alone.
            Some(mark) if shift => mark,
            Some(_) => base,
            None if shift ^ caps => upper_char(base),
            None => base,
        }
    } else if shift {
        match shifted {
            Some(ch) => ch,
            None => return KeyClass::Other,
        }
    } else {
        base
    };

    let other_is_letter = other_char(script, produced)
        .map(|ch| ch.is_alphabetic())
        .unwrap_or(false);
    if produced.is_alphabetic() || other_is_letter {
        KeyClass::Glyph(produced)
    } else {
        KeyClass::End(Term::Mark(produced))
    }
}

/// What `ch`, typed in `from`, becomes on the same key in the other layout.
/// `!` stays `!`. A Russian comma (Shift+.) becomes `_`, and German `/` becomes `?`.
pub fn other_mark(from: Script, ch: char) -> Option<char> {
    let chord = chord_for(from, ch)?;
    match classify(chord.code, chord.shift, false, from.other()) {
        KeyClass::Glyph(out) | KeyClass::End(Term::Mark(out)) => Some(out),
        KeyClass::End(_) | KeyClass::Other => None,
    }
}

pub fn other_char(from: Script, ch: char) -> Option<char> {
    let lower = ch.to_lowercase().next()?;
    let upper = ch.is_uppercase();
    for key in KEYS {
        let src = match from {
            Script::De => key.de,
            Script::Ru => key.ru,
        };
        if src == lower {
            let dst = match from {
                Script::De => key.ru,
                Script::Ru => key.de,
            };
            return Some(if upper { upper_char(dst) } else { dst });
        }
    }
    None
}

pub fn transpose(from: Script, word: &str) -> Option<String> {
    let mut out = String::with_capacity(word.len());
    for ch in word.chars() {
        out.push(other_char(from, ch)?);
    }
    Some(out)
}

/// Script of the letters in `word`. Punctuation is ignored.
/// Mixed Cyrillic and Latin returns `None`.
pub fn script_of(word: &str) -> Option<Script> {
    let mut seen = None;
    for ch in word.chars() {
        let script = if is_german(ch) {
            Some(Script::De)
        } else if is_cyrillic(ch) {
            Some(Script::Ru)
        } else {
            None
        };
        if let Some(script) = script {
            match seen {
                None => seen = Some(script),
                Some(prev) if prev != script => return None,
                Some(_) => {}
            }
        }
    }
    seen
}

/// `^`, `´` and `` ` `` on the German layout.
fn is_dead_key(scan: u16, shift: bool) -> bool {
    (scan == 0x29 && !shift) || scan == 0x0D
}

fn is_german(ch: char) -> bool {
    ch.is_ascii_alphabetic() || matches!(ch, 'ä' | 'ö' | 'ü' | 'ß' | 'Ä' | 'Ö' | 'Ü')
}

fn is_cyrillic(ch: char) -> bool {
    matches!(ch, 'а'..='я' | 'А'..='Я' | 'ё' | 'Ё')
}

fn upper_char(ch: char) -> char {
    ch.to_uppercase().next().unwrap_or(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_roundtrips() {
        for key in KEYS {
            let ru = other_char(Script::De, key.de).unwrap();
            let de = other_char(Script::Ru, key.ru).unwrap();
            assert_eq!(ru, key.ru, "de {} -> ru", key.de);
            assert_eq!(de, key.de, "ru {} -> de", key.ru);
            assert_eq!(other_char(Script::Ru, ru).unwrap(), key.de);
            assert_eq!(other_char(Script::De, de).unwrap(), key.ru);
        }
    }

    #[test]
    fn classic_words() {
        assert_eq!(transpose(Script::De, "ghbdtn").as_deref(), Some("привет"));
        assert_eq!(transpose(Script::Ru, "привет").as_deref(), Some("ghbdtn"));
        assert_eq!(transpose(Script::De, "cj,frf").as_deref(), Some("собака"));
        assert_eq!(transpose(Script::De, "üjhjij").as_deref(), Some("хорошо"));
        assert_eq!(transpose(Script::De, "Ghbdtn").as_deref(), Some("Привет"));
        assert_eq!(transpose(Script::Ru, "рфддщ").as_deref(), Some("hallo"));
        assert_eq!(transpose(Script::De, "zt").as_deref(), Some("не"));
        assert_eq!(transpose(Script::De, "y").as_deref(), Some("я"));
        assert_eq!(transpose(Script::Ru, "нцуш").as_deref(), Some("zwei"));
        assert_eq!(transpose(Script::Ru, "ьжсреу").as_deref(), Some("möchte"));
        assert_eq!(transpose(Script::Ru, "Хиук").as_deref(), Some("Über"));
        assert_eq!(transpose(Script::Ru, "Ыекф-у").as_deref(), Some("Straße"));
        assert_eq!(transpose(Script::Ru, "пкщ-").as_deref(), Some("groß"));
        assert_eq!(transpose(Script::De, "rnjßnj").as_deref(), Some("кто-то"));
        assert_eq!(other_mark(Script::Ru, ','), Some('_'));
        assert_eq!(other_mark(Script::De, '/'), Some('?'));
        assert_eq!(other_mark(Script::De, '!'), Some('!'));
        assert_eq!(other_mark(Script::Ru, '?'), Some('/'));
        assert_eq!(other_mark(Script::De, '?'), Some('_'));
        assert_eq!(other_mark(Script::De, '§'), Some('№'));
    }

    #[test]
    fn classify_letters_and_punctuation() {
        assert_eq!(
            classify(0x10, false, false, Script::De),
            KeyClass::Glyph('q')
        );
        assert_eq!(
            classify(0x10, false, false, Script::Ru),
            KeyClass::Glyph('й')
        );
        assert_eq!(
            classify(0x10, true, false, Script::De),
            KeyClass::Glyph('Q')
        );
        assert_eq!(classify(0x10, true, true, Script::De), KeyClass::Glyph('q'));
        assert_eq!(
            classify(0x15, false, false, Script::De),
            KeyClass::Glyph('z')
        );
        assert_eq!(
            classify(0x2C, false, false, Script::De),
            KeyClass::Glyph('y')
        );
        assert_eq!(
            classify(0x1A, false, false, Script::De),
            KeyClass::Glyph('ü')
        );
        assert_eq!(
            classify(0x1A, true, false, Script::De),
            KeyClass::Glyph('Ü')
        );
        assert_eq!(
            classify(0x27, false, true, Script::De),
            KeyClass::Glyph('Ö')
        );
        assert_eq!(
            classify(0x28, false, false, Script::De),
            KeyClass::Glyph('ä')
        );
        assert_eq!(
            classify(0x33, false, false, Script::De),
            KeyClass::Glyph(',')
        );
        assert_eq!(
            classify(0x33, true, false, Script::De),
            KeyClass::End(Term::Mark(';'))
        );
        assert_eq!(
            classify(0x33, false, false, Script::Ru),
            KeyClass::Glyph('б')
        );
        assert_eq!(
            classify(0x33, true, false, Script::Ru),
            KeyClass::Glyph('Б')
        );
        assert_eq!(
            classify(0x35, false, false, Script::De),
            KeyClass::End(Term::Mark('-'))
        );
        assert_eq!(
            classify(0x35, false, false, Script::Ru),
            KeyClass::End(Term::Mark('.'))
        );
        assert_eq!(
            classify(0x35, true, false, Script::Ru),
            KeyClass::End(Term::Mark(','))
        );
        assert_eq!(
            classify(0x02, false, false, Script::De),
            KeyClass::End(Term::Mark('1'))
        );
        assert_eq!(
            classify(0x04, true, false, Script::Ru),
            KeyClass::End(Term::Mark('№'))
        );
        assert_eq!(
            classify(0x04, true, false, Script::De),
            KeyClass::End(Term::Mark('§'))
        );
        assert_eq!(
            classify(0x29, false, false, Script::Ru),
            KeyClass::Glyph('ё')
        );
        assert_eq!(classify(0x00, false, false, Script::De), KeyClass::Other);
    }

    #[test]
    fn eszett_shares_its_key_with_the_russian_hyphen() {
        assert_eq!(
            classify(0x0C, false, false, Script::De),
            KeyClass::Glyph('ß')
        );
        assert_eq!(
            classify(0x0C, true, false, Script::De),
            KeyClass::End(Term::Mark('?'))
        );
        assert_eq!(classify(0x0C, false, true, Script::De), KeyClass::Glyph('ß'));
        assert_eq!(
            classify(0x0C, false, false, Script::Ru),
            KeyClass::Glyph('-')
        );
        assert_eq!(
            classify(0x0C, true, false, Script::Ru),
            KeyClass::End(Term::Mark('_'))
        );
        assert_eq!(
            chord_for(Script::De, 'ß'),
            Some(Chord { code: 0x0C, shift: false })
        );
        assert_eq!(
            chord_for(Script::De, '?'),
            Some(Chord { code: 0x0C, shift: true })
        );
        assert_eq!(
            chord_for(Script::Ru, '?'),
            Some(Chord { code: 0x08, shift: true })
        );
    }

    #[test]
    fn german_dead_keys_drop_the_word() {
        assert_eq!(classify(0x29, false, false, Script::De), KeyClass::Other);
        assert_eq!(
            classify(0x29, true, false, Script::De),
            KeyClass::End(Term::Mark('°'))
        );
        assert_eq!(classify(0x0D, false, false, Script::De), KeyClass::Other);
        assert_eq!(classify(0x0D, true, false, Script::De), KeyClass::Other);
        assert_eq!(
            classify(0x0D, false, false, Script::Ru),
            KeyClass::End(Term::Mark('='))
        );
    }

    #[test]
    fn script_follows_the_letters() {
        assert_eq!(script_of("möchte"), Some(Script::De));
        assert_eq!(script_of("ß"), Some(Script::De));
        assert_eq!(script_of("кто-то"), Some(Script::Ru));
        assert_eq!(script_of("cj,frf"), Some(Script::De));
        assert_eq!(script_of("приvet"), None);
        assert_eq!(script_of("-"), None);
    }

    #[test]
    fn chords_match_the_keymap() {
        assert_eq!(
            chord_for(Script::De, 'q'),
            Some(Chord { code: 0x10, shift: false })
        );
        assert_eq!(
            chord_for(Script::Ru, 'й'),
            Some(Chord { code: 0x10, shift: false })
        );
        assert_eq!(
            chord_for(Script::Ru, 'Й'),
            Some(Chord { code: 0x10, shift: true })
        );
        assert_eq!(
            chord_for(Script::De, 'Ä'),
            Some(Chord { code: 0x28, shift: true })
        );
        assert_eq!(
            chord_for(Script::De, ','),
            Some(Chord { code: 0x33, shift: false })
        );
        assert_eq!(
            chord_for(Script::Ru, 'б'),
            Some(Chord { code: 0x33, shift: false })
        );
        let word = "привет";
        for ch in word.chars() {
            let chord = chord_for(Script::Ru, ch).unwrap();
            assert_eq!(
                classify(chord.code, chord.shift, false, Script::Ru),
                KeyClass::Glyph(ch)
            );
        }
    }
}

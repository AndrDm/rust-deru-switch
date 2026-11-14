use crate::dict::{is_abbreviation, normalize, HasWords};
use crate::keymap::{script_of, transpose, Script};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub text: String,
    /// Layout the word was typed in.
    pub typed_in: Script,
    /// Layout to leave active. Same as `typed_in` when only the case changes.
    pub switch_to: Script,
    /// Caps Lock produced this shape (`пРИВЕТ`). The platform releases it.
    pub release_caps: bool,
}

/// Correct `word` when it is not a word in its own script but the same
/// keystrokes are a word in the other one.
///
/// A dictionary word typed as one lowercase letter and then only capitals
/// (`пРИВЕТ`, `hALLO`) is recased to `Привет` / `Hallo` and stays on its
/// layout. All capitals and a capital run after the first letter stay.
///
/// Trailing punctuation stays as it appeared on screen (`ghbdtn,` → `привет,`).
/// A punctuation mark that is a letter on the other layout stays inside the
/// word (`cj,frf` → `собака`). Nothing is changed when the typed stem is
/// already a dictionary word, when both interpretations are words, or when
/// neither is.
///
/// A German abbreviation counts only in its own case: `TV` stays, `tv` is
/// `ем`. The Russian hyphen sits on the `ß` key, so a hyphenated Russian word
/// is one word here and is known when each of its parts is (`rnjßnj` → `кто-то`).
pub fn autocorrect(word: &str, dict: &impl HasWords) -> Option<Change> {
    let script = script_of(word)?;
    let chars: Vec<char> = word.chars().collect();
    let mut end = chars.len();
    while end > 0 {
        if chars[end..].iter().any(|ch| ch.is_alphabetic()) {
            break;
        }
        let stem: String = chars[..end].iter().collect();
        if letter_count(&stem) < 2 {
            break;
        }
        if dict.has(script, &normalize(&stem)) {
            return fix_caps_lock(script, &stem, &chars[end..]);
        }
        if written_exactly(script, &stem, dict) || hyphenated(script, &stem, dict) {
            return None;
        }
        if let Some(converted) = transpose(script, &stem) {
            let other = script.other();
            if dict.has(other, &normalize(&converted))
                || written_exactly(other, &converted, dict)
                || hyphenated(other, &converted, dict)
            {
                let mut text = converted;
                text.extend(chars[end..].iter().copied());
                return Some(Change {
                    text,
                    typed_in: script,
                    switch_to: script.other(),
                    release_caps: false,
                });
            }
        }
        if end == 0 {
            break;
        }
        let last = chars[end - 1];
        if last.is_alphabetic() {
            break;
        }
        end -= 1;
    }
    None
}

/// `TV`, `kB`: an abbreviation the dictionary holds in exactly this case.
fn written_exactly(script: Script, word: &str, dict: &impl HasWords) -> bool {
    script == Script::De && is_abbreviation(word) && dict.has(script, word)
}

/// `кто-то`, `по-русски`: Russian words joined by hyphens.
fn hyphenated(script: Script, word: &str, dict: &impl HasWords) -> bool {
    script == Script::Ru
        && word.contains('-')
        && word
            .split('-')
            .all(|part| dict.has(script, &normalize(part)))
}

/// Transpose the current word without asking the dictionary.
/// Trailing punctuation stays as it appeared (`ghbdtn,` → `привет,`).
/// A mark inside the word is still a key of the other layout (`cj,frf` → `собака`).
pub fn force_convert(word: &str) -> Option<Change> {
    let script = script_of(word)?;
    let chars: Vec<char> = word.chars().collect();
    let mut end = chars.len();
    while end > 0 && !chars[end - 1].is_alphabetic() {
        end -= 1;
    }
    if end == 0 {
        return None;
    }
    let stem: String = chars[..end].iter().collect();
    let converted = transpose(script, &stem)?;
    if converted == stem {
        return None;
    }
    let mut text = converted;
    text.extend(chars[end..].iter().copied());
    Some(Change {
        text,
        typed_in: script,
        switch_to: script.other(),
        release_caps: false,
    })
}

/// Caps Lock with Shift only on the first letter: `пРИВЕТ` → `Привет`.
/// `None` leaves a dictionary word as it was typed.
fn fix_caps_lock(script: Script, stem: &str, tail: &[char]) -> Option<Change> {
    let fixed = invert_caps_lock(stem)?;
    let mut text = fixed;
    text.extend(tail.iter().copied());
    Some(Change {
        text,
        typed_in: script,
        switch_to: script,
        release_caps: true,
    })
}

fn invert_caps_lock(stem: &str) -> Option<String> {
    let mut chars = stem.chars();
    let first = chars.next()?;
    if !first.is_alphabetic() || !first.is_lowercase() {
        return None;
    }
    let mut out = String::new();
    out.extend(first.to_uppercase());
    let mut rest = false;
    for ch in chars {
        if !ch.is_alphabetic() || !ch.is_uppercase() {
            return None;
        }
        rest = true;
        out.extend(ch.to_lowercase());
    }
    rest.then_some(out)
}

fn letter_count(word: &str) -> usize {
    word.chars().filter(|ch| ch.is_alphabetic()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dict::MapDict;

    fn dict() -> MapDict {
        MapDict::new(
            &["hallo", "welt", "an", "hund", "der", "tot", "groß", "möchte", "TV", "kB"],
            &[
                "привет",
                "собака",
                "хорошо",
                "еще",
                "он",
                "не",
                "что",
                "кто",
                "то",
                "ем",
                "ли",
            ],
        )
    }

    #[test]
    fn switches_wrong_layout_words() {
        let dict = dict();
        assert_eq!(autocorrect("ghbdtn", &dict).unwrap().text, "привет");
        assert_eq!(autocorrect("Ghbdtn", &dict).unwrap().text, "Привет");
        assert_eq!(autocorrect("cj,frf", &dict).unwrap().text, "собака");
        assert_eq!(autocorrect("üjhjij", &dict).unwrap().text, "хорошо");
        assert_eq!(autocorrect("рфддщ", &dict).unwrap().text, "hallo");
        assert_eq!(autocorrect("jz", &dict).unwrap().text, "он");
        assert_eq!(autocorrect("zt", &dict).unwrap().text, "не");
        assert_eq!(autocorrect("xnj", &dict).unwrap().text, "что");
        assert_eq!(autocorrect("ьжсреу", &dict).unwrap().text, "möchte");
        assert_eq!(autocorrect("Ьжсреу", &dict).unwrap().text, "Möchte");
    }

    #[test]
    fn eszett_and_the_russian_hyphen_share_a_key() {
        let dict = dict();
        let de = autocorrect("пкщ-", &dict).unwrap();
        assert_eq!(de.text, "groß");
        assert_eq!(de.switch_to, Script::De);
        let ru = autocorrect("rnjßnj", &dict).unwrap();
        assert_eq!(ru.text, "кто-то");
        assert_eq!(ru.switch_to, Script::Ru);
        assert!(autocorrect("кто-то", &dict).is_none());
        assert!(autocorrect("привет-", &dict).is_none());
        assert!(autocorrect("groß", &dict).is_none());
        assert!(autocorrect("rnjß", &dict).is_none());
    }

    #[test]
    fn abbreviations_match_only_in_their_own_case() {
        let dict = dict();
        assert!(autocorrect("TV", &dict).is_none());
        assert!(autocorrect("kB", &dict).is_none());
        assert_eq!(autocorrect("tv", &dict).unwrap().text, "ем");
        assert_eq!(autocorrect("kb", &dict).unwrap().text, "ли");
    }

    #[test]
    fn keeps_real_words_and_unknowns() {
        let dict = dict();
        assert!(autocorrect("hallo", &dict).is_none());
        assert!(autocorrect("привет", &dict).is_none());
        assert!(autocorrect("an", &dict).is_none());
        assert!(autocorrect("asdf", &dict).is_none());
        assert!(autocorrect("q", &dict).is_none());
        assert!(autocorrect("tot", &dict).is_none());
        assert!(autocorrect("еще", &dict).is_none());
    }

    #[test]
    fn peels_trailing_punctuation() {
        let dict = dict();
        assert_eq!(autocorrect("ghbdtn,", &dict).unwrap().text, "привет,");
        assert_eq!(autocorrect("ghbdtn.", &dict).unwrap().text, "привет.");
        assert!(autocorrect("hallo,", &dict).is_none());
    }

    #[test]
    fn caps_lock_on_the_first_letter_flips_case() {
        let dict = dict();
        let ru = autocorrect("пРИВЕТ", &dict).unwrap();
        assert_eq!(ru.text, "Привет");
        assert_eq!(ru.typed_in, Script::Ru);
        assert_eq!(ru.switch_to, Script::Ru);
        assert!(ru.release_caps);
        assert!(!autocorrect("ghbdtn", &dict).unwrap().release_caps);
        assert_eq!(autocorrect("пРИВЕТ,", &dict).unwrap().text, "Привет,");

        let de = autocorrect("hALLO", &dict).unwrap();
        assert_eq!(de.text, "Hallo");
        assert_eq!(de.typed_in, Script::De);
        assert_eq!(de.switch_to, Script::De);
        assert_eq!(autocorrect("hALLO,", &dict).unwrap().text, "Hallo,");
        assert_eq!(autocorrect("mÖCHTE", &dict).unwrap().text, "Möchte");

        let yo = MapDict::new(&[], &["ежик"]);
        assert_eq!(autocorrect("ёЖИК", &yo).unwrap().text, "Ёжик");
        let short = MapDict::new(&[], &["на"]);
        assert_eq!(autocorrect("нА", &short).unwrap().text, "На");
        let abbr = MapDict::new(&["ios"], &["сша"]);
        assert_eq!(autocorrect("iOS", &abbr).unwrap().text, "Ios");
        assert_eq!(autocorrect("сША", &abbr).unwrap().text, "Сша");
    }

    #[test]
    fn other_case_patterns_stay() {
        let dict = dict();
        assert!(autocorrect("ПРИВЕТ", &dict).is_none());
        assert!(autocorrect("ПРивет", &dict).is_none());
        assert!(autocorrect("пРивет", &dict).is_none());
        assert!(autocorrect("Привет", &dict).is_none());
        assert!(autocorrect("привет", &dict).is_none());
        assert!(autocorrect("HALLO", &dict).is_none());
        assert!(autocorrect("HAllo", &dict).is_none());
        assert!(autocorrect("Hallo", &dict).is_none());
        let unknown = MapDict::new(&[], &[]);
        assert!(autocorrect("пРИВЕТ", &unknown).is_none());
    }

    #[test]
    fn yo_matches_dictionary_without_yo() {
        let dict = dict();
        assert!(autocorrect("ещё", &dict).is_none());
    }

    #[test]
    fn force_ignores_dictionary() {
        assert_eq!(force_convert("asdf").unwrap().text, "фыва");
        assert_eq!(force_convert("ghbdtn,").unwrap().text, "привет,");
        assert_eq!(force_convert("cj,frf").unwrap().text, "собака");
        assert_eq!(force_convert("нцуш").unwrap().text, "zwei");
    }

    #[test]
    fn embedded_dictionary_handles_everyday_words() {
        use crate::dict::StaticDict;
        let dict = StaticDict::load();
        assert!(dict.has(Script::Ru, "привет"));
        assert!(dict.has(Script::De, "hallo"));
        assert!(dict.has(Script::De, "straße"));
        assert!(dict.has(Script::De, "labview"));
        assert!(!dict.has(Script::De, "hello"));
        assert_eq!(autocorrect("ghbdtn", &dict).unwrap().text, "привет");
        assert_eq!(autocorrect("cj,frf", &dict).unwrap().text, "собака");
        assert_eq!(autocorrect("üjhjij", &dict).unwrap().text, "хорошо");
        assert_eq!(autocorrect("rnjßnj", &dict).unwrap().text, "кто-то");
        assert_eq!(autocorrect("ns", &dict).unwrap().text, "ты");
        assert_eq!(autocorrect("bp", &dict).unwrap().text, "из");
        assert_eq!(autocorrect("рфддщ", &dict).unwrap().text, "hallo");
        assert_eq!(autocorrect("Ыекф-у", &dict).unwrap().text, "Straße");
        assert_eq!(autocorrect("ьжсреу", &dict).unwrap().text, "möchte");
        assert_eq!(autocorrect("пкщ-", &dict).unwrap().text, "groß");
        assert_eq!(autocorrect("нцуш", &dict).unwrap().text, "zwei");
        assert!(autocorrect("hallo", &dict).is_none());
        assert!(autocorrect("привет", &dict).is_none());
        assert!(autocorrect("TV", &dict).is_none());
        assert!(autocorrect("PDF", &dict).is_none());
        assert!(autocorrect("LabVIEW", &dict).is_none());
        assert_eq!(autocorrect("пРИВЕТ", &dict).unwrap().text, "Привет");
        assert_eq!(autocorrect("hALLO", &dict).unwrap().text, "Hallo");
        assert!(autocorrect("ПРИВЕТ", &dict).is_none());
        assert!(autocorrect("ПРивет", &dict).is_none());
        assert!(autocorrect("der", &dict).is_none());
        assert!(autocorrect("собака", &dict).is_none());
        assert!(dict.has(Script::De, "..."));
        let dots = autocorrect("ююю", &dict).unwrap();
        assert_eq!(dots.text, "...");
        assert_eq!(dots.switch_to, Script::De);
        assert_eq!(autocorrect("ЮЮЮ", &dict).unwrap().text, "...");
        assert!(autocorrect("юю", &dict).is_none());
        for word in [
            "und", "nicht", "für", "über", "können", "weiß", "Zeit", "Jahr", "heute", "danke",
        ] {
            assert!(
                autocorrect(word, &dict).is_none(),
                "{word} should stay as typed"
            );
        }
    }
}

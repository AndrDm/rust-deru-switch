# CLAUDE.md

Guidance for working on this repository. The original task spec is in `AGENTS.md`.

## What this is

Zyabra DE/RU: a Windows tray program, written in Rust, that fixes words typed in the wrong keyboard layout. If a word typed in German is a Russian word on the same keys, it is erased, retyped in Russian, and the layout switches (`ghbdtn` → `привет`). The same works from Russian to German (`рфддщ` → `hallo`).

It is a fork of [Zyabra](https://github.com/DmNep/zyabra), which handled English/Russian on Windows, Linux and macOS. This fork:

- replaces English with German everywhere: it reads `de.txt` and `extra-de.txt`, not the `-en` files;
- supports **Windows only**. Linux and macOS support was removed.

## Status

The code is converted. Still open:

1. Delete leftovers: `src/linux/`, `src/mac/`, `dictionaries/en.txt`, `dictionaries/extra-en.txt`.
2. `.github/workflows/ci.yml` still has the Ubuntu `test` job and the `mac` job. Remove the `mac` job and run `test` on `windows-latest`.
3. Run `cargo build --release` on Windows. `Cargo.lock` still lists `evdev`, `x11rb`, `libc` and `objc`; the first build prunes them.
4. `NOTICE.md` needs the source and licence of `dictionaries/de.txt`, which are not recorded yet.

## Layout

```
src/
  lib.rs        library crate, #![forbid(unsafe_code)], no I/O
  keymap.rs     physical keys: German QWERTZ ↔ Russian ЙЦУКЕН, Script::{De, Ru}
  decide.rs     autocorrect / force_convert: what to rewrite, per word
  engine.rs     one-word buffer, Pause toggling, undo
  dict.rs       embedded dictionaries (HasWords, StaticDict, MapDict for tests)
  args.rs       command line (--key CODE, --key 2sh, help); text is in Russian
  shiftpair.rs  double-Shift detection
  main.rs       Windows entry point; on any other OS it prints an error and exits
  win/          everything that talks to Windows: hook.rs, send.rs, ui.rs, mod.rs
build.rs        gzips the dictionaries into OUT_DIR
dictionaries/   de.txt, ru.txt, extra-de.txt, extra-ru.txt
tools/build_dicts.py  rebuilds ru.txt from third_party/ (run by hand)
```

The library decides what to do and `src/win/` carries it out. Keep that split: the library has no unsafe code, no Windows types and no I/O.

## Build and test

```
cargo test             # library tests, any OS (Windows deps are target-specific)
cargo build --release  # Windows only → target/release/zyabra.exe
```

The `embedded_dictionary_handles_everyday_words` test loads the real ~30 MB `de.txt` and `ru.txt`, so it takes a few seconds.

## Design decisions (keep these)

- **Keys are paired by physical position**, by AT set‑1 scan code (`KEYS` and `NUMS` in `keymap.rs`), not by alphabet. QWERTZ: `z`/`y` are on `н`/`я`; `ü`, `ö`, `ä` are on `х`, `ж`, `э`; `+` is on `ъ`.
- **`ß` (scan 0x0C) shares a key with the Russian hyphen.** So a hyphen typed in Russian is part of the word. Shift+`ß` is `?`, and Caps Lock leaves `ß` alone (`de_shift: Some('?')` on a letter key). Hyphenated Russian words count as dictionary words when every part is one (`rnjßnj` → `кто-то`).
- **German dead keys** (`^` unshifted on 0x29, and `´`/`` ` `` on 0x0D) return `KeyClass::Other` and drop the current word. AltGr combinations never reach the keymap.
- **German abbreviations are case-sensitive.** `build.rs` lowercases `de.txt` *except* words with a capital after the first letter (`TV`, `kB`, `GmbH`), which stay as written. `decide.rs` accepts them only in that exact case. Without this, about 100 common short Russian words (`ты` = `ns`, `из` = `bp`, `мы` = `vs`) could never be corrected.
- **Words that are valid in both languages stay as typed** (about 130 pairs, e.g. `tot`/`еще`, `in`/`шт`). Pause converts them by hand. Do not "fix" this by preferring one language.
- **German word length is 2–30 letters**, because of compounds. Russian stays at 2–20. Allowed German letters are `a–z`, `ä`, `ö`, `ü`, `ß`.
- **Windows layout IDs:** German `00000407` (primary language `0x07`), Russian `00000419` (`0x19`). With any other active layout, including English, the program leaves keys alone.

## Conventions

- **No network, ever.** CI greps `src`, `build.rs` and `Cargo.toml` for network APIs and fails if it finds any. Nothing typed is written to disk or the clipboard.
- **UI, help and error strings are in Russian.** Comments in the `extra-*.txt` files are in Russian. Code comments and docs are in English.
- **Line endings are LF** in all text files.
- Dictionary extras: one word per line, `#` starts a comment, a run of periods is allowed (`...` is the German face of `ююю`).
- **Commits:** never leave `Co-authored-by: Cursor <cursoragent@cursor.com>` in a commit message (see `.cursor/rules/no-cursoragent.mdc`). Never force-push `main` without explicit permission.
- Add a test with every keymap or decision change. The existing tests in `keymap.rs` and `decide.rs` are the reference for QWERTZ behaviour.

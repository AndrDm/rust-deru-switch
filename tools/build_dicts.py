"""Build the embedded Russian word list from an already downloaded source.

The running program never calls this and never uses the network.
Run it by hand after placing the source file in third_party/:

  third_party/russian-cp1251.txt     https://github.com/danakt/russian-words (MIT), cp1251

The German list, dictionaries/de.txt, is used as it is: build.rs filters and
lowercases it during the build.
"""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
THIRD = ROOT / "third_party"
OUT = ROOT / "dictionaries"


def russian_words() -> set[str]:
    text = (THIRD / "russian-cp1251.txt").read_bytes().decode("cp1251")
    words: set[str] = set()
    for line in text.splitlines():
        word = line.strip().lower().replace("ё", "е")
        if 2 <= len(word) <= 20 and all("а" <= ch <= "я" for ch in word):
            words.add(word)
    return words


def write(path: Path, words: set[str]) -> None:
    path.write_text("\n".join(sorted(words)) + "\n", encoding="utf-8", newline="\n")


def main() -> None:
    OUT.mkdir(exist_ok=True)
    ru = russian_words()
    write(OUT / "ru.txt", ru)
    print(f"ru {len(ru)}")


if __name__ == "__main__":
    main()

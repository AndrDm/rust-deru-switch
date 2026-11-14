use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=dictionaries/de.txt");
    println!("cargo:rerun-if-changed=dictionaries/ru.txt");
    println!("cargo:rerun-if-changed=assets/zyabra.ico");
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let de = german_words("dictionaries/de.txt");
    gzip(de.as_bytes(), "dictionaries/de.txt", &out.join("de.txt.gz"));
    let ru = std::fs::read("dictionaries/ru.txt")
        .unwrap_or_else(|err| panic!("read dictionaries/ru.txt: {err}"));
    gzip(&ru, "dictionaries/ru.txt", &out.join("ru.txt.gz"));
    //embed_version(&out);
}

/*
fn embed_version(out: &std::path::Path) {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let version = std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION");
    let mut parts = version.split('.');
    let major = parts.next().unwrap_or("0");
    let minor = parts.next().unwrap_or("0");
    let patch = parts.next().unwrap_or("0");
    let rc_path = out.join("zyabra.rc");
    let obj_path = out.join("zyabra-version.o");
    std::fs::copy("assets/zyabra.ico", out.join("zyabra.ico"))
        .unwrap_or_else(|err| panic!("copy icon: {err}"));
    let text = format!(
        r#"#pragma code_page(65001)
1 ICON "zyabra.ico"
1 VERSIONINFO
FILEVERSION {major},{minor},{patch},0
PRODUCTVERSION {major},{minor},{patch},0
FILEFLAGSMASK 0x3fL
FILEFLAGS 0
FILEOS 0x40004L
FILETYPE 0x1L
FILESUBTYPE 0
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904B0"
    BEGIN
      VALUE "FileDescription", "Зябра {version}"
      VALUE "FileVersion", "{version}"
      VALUE "InternalName", "zyabra"
      VALUE "OriginalFilename", "zyabra-{version}.exe"
      VALUE "ProductName", "Зябра {version}"
      VALUE "ProductVersion", "{version}"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x409, 1200
  END
END
"#
    );
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend(text.into_bytes());
    std::fs::write(&rc_path, &bytes).unwrap_or_else(|err| panic!("write rc: {err}"));
    let status = std::process::Command::new("windres")
        .current_dir(out)
        .args(["--codepage", "65001", "-O", "coff", "-i"])
        .arg(&rc_path)
        .arg("-o")
        .arg(&obj_path)
        .status()
        .unwrap_or_else(|err| panic!("windres: {err}"));
    if !status.success() {
        panic!("windres failed");
    }
    println!("cargo:rustc-link-arg-bins={}", obj_path.display());
}
*/

/// `dictionaries/de.txt` is a plain German word list, one word per line, in
/// its natural case. Words of 2 to 30 letters are kept, sorted, one per line.
/// A word with a capital after its first letter (`TV`, `kB`, `GmbH`) is an
/// abbreviation and stays as written. Everything else is lowercased.
fn german_words(src: &str) -> String {
    let text = std::fs::read_to_string(src).unwrap_or_else(|err| panic!("read {src}: {err}"));
    let mut words: Vec<String> = text
        .lines()
        .map(str::trim)
        .filter(|word| (2..=30).contains(&word.chars().count()))
        .filter(|word| word.chars().all(is_german_letter))
        .map(|word| {
            if word.chars().skip(1).any(char::is_uppercase) {
                word.to_string()
            } else {
                word.to_lowercase()
            }
        })
        .collect();
    words.sort_unstable();
    words.dedup();
    let mut out = words.join("\n");
    out.push('\n');
    out
}

fn is_german_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || matches!(ch, 'ä' | 'ö' | 'ü' | 'ß' | 'Ä' | 'Ö' | 'Ü')
}

fn gzip(data: &[u8], src: &str, dst: &std::path::Path) {
    let file = File::create(dst).unwrap_or_else(|err| panic!("create {}: {err}", dst.display()));
    let mut encoder = flate2::write::GzEncoder::new(file, flate2::Compression::best());
    encoder
        .write_all(data)
        .unwrap_or_else(|err| panic!("compress {src}: {err}"));
    encoder
        .finish()
        .unwrap_or_else(|err| panic!("finish {src}: {err}"));
}

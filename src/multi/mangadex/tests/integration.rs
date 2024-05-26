use std::path::PathBuf;

use midoku_bindings::Bindings;
use midoku_types::manga::{ContentRating, ReadingMode, Status};
use once_cell::sync::Lazy;

const MANGA_ID: &str = "58d988fb-be92-41a0-8340-17381ab7869a";
const CHAPTER_ID: &str = "a27c6a6c-4212-4c8a-863e-8df5fc2c093c";

static EXTENSION_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let package_name = env!("CARGO_PKG_NAME");
    let extension_name = package_name.replace("-", "_");

    // Build the extension in release mode
    let output = std::process::Command::new("cargo-component")
        .args(&[
            "build",
            "--release",
            "--package",
            package_name,
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("Failed to build the extension");

    if !output.status.success() {
        panic!(
            "Failed to build the extension: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Parse the built extension path from the stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    let extension_path = stderr
        .lines()
        .find(|line| line.ends_with(&format!("{}.wasm", extension_name)))
        .map(|line| line.trim_end().split_whitespace().last().unwrap())
        .expect("Failed to parse the extension path");

    extension_path.into()
});

#[test]
fn test_bindings_from_file() {
    let bindings = Bindings::from_file(EXTENSION_PATH.as_path());
    assert!(bindings.is_ok());
}

#[test]
fn test_bindings_initialize() {
    let bindings = Bindings::from_file(EXTENSION_PATH.as_path()).unwrap();
    let _ = bindings.initialize().unwrap();
}

#[test]
fn test_bindings_get_manga_list() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path()).unwrap();
    let (manga_list, has_next) = bindings.get_manga_list(vec![], 0).unwrap();

    assert_eq!(manga_list.len(), 20);
    assert!(has_next);
}

#[test]
fn test_bindings_get_manga_details() {
    let bindings = midoku_bindings::Bindings::from_file(EXTENSION_PATH.as_path()).unwrap();
    let manga = bindings.get_manga_details(MANGA_ID.to_string()).unwrap();

    assert_eq!(manga.id, MANGA_ID);
    assert_eq!(manga.title, "enígmә");
    assert_eq!(
        manga.url,
        "https://mangadex.org/title/58d988fb-be92-41a0-8340-17381ab7869a"
    );
    assert_eq!(
        manga.description,
        "Haiba Sumio is a student at a Tokyo High School with an interesting ability… He occasionally falls asleep and wakes up with premonitions of the future written out on his \"Dream Diary.\" With this ability, Sumio helps out people in trouble before anything bad happens, until one day when his peaceful life completely changes."
    );
    assert_eq!(
        manga.cover_url,
        "https://mangadex.org/covers/58d988fb-be92-41a0-8340-17381ab7869a/8e18ca42-2d7d-42c9-a0b6-3134eb2bb042.jpg"
    );
    assert_eq!(manga.author_name, "Sakaki Kenji");
    assert_eq!(manga.artist_name, "Sakaki Kenji");
    assert_eq!(
        manga.categories,
        vec![
            "Action",
            "Psychological",
            "Comedy",
            "Adventure",
            "Drama",
            "School Life",
            "Supernatural",
            "Mystery"
        ]
    );
    assert_eq!(manga.status, Status::Completed);
    assert_eq!(manga.content_rating, ContentRating::Safe);
    assert_eq!(manga.reading_mode, ReadingMode::RightToLeft);
}

#[test]
fn test_bindings_get_chapter_list() {
    let bindings = Bindings::from_file(EXTENSION_PATH.as_path()).unwrap();
    let chapter_list = bindings.get_chapter_list(MANGA_ID.to_string()).unwrap();

    assert_eq!(chapter_list.len(), 57);
}

#[test]
fn test_bindings_get_page_list() {
    let bindings = Bindings::from_file(EXTENSION_PATH.as_path()).unwrap();
    let page_list = bindings
        .get_page_list(MANGA_ID.to_string(), CHAPTER_ID.to_string())
        .unwrap();

    assert_eq!(page_list.len(), 54);
}

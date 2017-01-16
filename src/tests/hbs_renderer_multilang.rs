#[cfg(test)]

use std::path::{Path, PathBuf};

use book::MDBook;
use renderer::{Renderer, HtmlHandlebars};
use utils;

#[test]
fn it_renders_multilanguage_book() {
    let path = PathBuf::from(".").join("src").join("tests").join("book-wonderland-multilang");

    let renderer = HtmlHandlebars::new();
    if let Err(e) = renderer.build(&path) {
        panic!("{:#?}", e);
    }

    let mut proj = MDBook::new(&path);
    proj.read_config();
    proj.parse_books();

    let mut book_path: &Path = proj.translations.get("en").unwrap().config.get_dest();
    let mut chapter_path: PathBuf = PathBuf::from("".to_string());
    let mut s: String = String::new();

    // Test if index.html in the project dest folder is the main book's first chapter

    chapter_path = proj.get_dest_base().join("index.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<title>Titlepage - Alice's Adventures in Wonderland</title>"));
    assert!(s.contains("<h1>Alice's Adventures in Wonderland</h1>"));
    assert!(s.contains("<base href=\"\">"));

    // Test if each translation was rendered

    chapter_path = book_path.join("tears.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<h1>The Pool of Tears</h1>"));
    assert!(s.contains("<base href=\"../\">"));
    assert!(s.contains("li><a href=\"en/tears.html\" class=\"active\"><strong>2.</strong> The Pool of Tears</a></li>"));

    book_path = proj.translations.get("fr").unwrap().config.get_dest();
    chapter_path = book_path.join("tears.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<h1>La mare aux larmes</h1>"));
    assert!(s.contains("<base href=\"../\">"));

    book_path = proj.translations.get("hu").unwrap().config.get_dest();
    chapter_path = book_path.join("tears.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<h1>Könnytó</h1>"));
    assert!(s.contains("<base href=\"../\">"));

    // Test if book's asset files were copied

    assert!(proj.get_dest_base().join("images").join("Queen.jpg").exists());

    // Test if default translation indexes are displayed

    book_path = proj.translations.get("hu").unwrap().config.get_dest();
    chapter_path = book_path.join("tarka-farka.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<a href=\"en/index.html\">en</a>"));
    assert!(s.contains("<a href=\"hu/index.html\">hu</a>"));
    assert!(s.contains("<a href=\"fr/index.html\">fr</a>"));

    // Test if translation links given in the TOML header were rendered

    book_path = proj.translations.get("en").unwrap().config.get_dest();
    chapter_path = book_path.join("rabbit-hole.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<a href=\"hu/nyuszi.html\">hu</a>"));
    assert!(s.contains("<a href=\"fr/terrier.html\">fr</a>"));

    // Test if translation links by translation_id were found

    book_path = proj.translations.get("en").unwrap().config.get_dest();
    chapter_path = book_path.join("long-tale.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<a href=\"en/long-tale.html\">en</a>"));
    assert!(s.contains("<span>fr</span>"));
    assert!(s.contains("<a href=\"hu/tarka-farka.html\">hu</a>"));

    // Test if translation links by src_path were found

    book_path = proj.translations.get("en").unwrap().config.get_dest();
    chapter_path = book_path.join("tears.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<a href=\"en/tears.html\">en</a>"));
    assert!(s.contains("<a href=\"fr/tears.html\">fr</a>"));
    assert!(s.contains("<a href=\"hu/tears.html\">hu</a>"));

    // Test if print.html is produced for each translations

    book_path = proj.translations.get("en").unwrap().config.get_dest();
    chapter_path = book_path.join("print.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<h1>The Pool of Tears</h1>"));

    book_path = proj.translations.get("fr").unwrap().config.get_dest();
    chapter_path = book_path.join("print.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<h1>La mare aux larmes</h1>"));

    book_path = proj.translations.get("hu").unwrap().config.get_dest();
    chapter_path = book_path.join("print.html");
    s = utils::fs::file_to_string(&chapter_path).unwrap();
    assert!(s.contains("<h1>Könnytó</h1>"));
}

#[test]
fn it_recognizes_first_translation_as_main() {
    let path = PathBuf::from(".").join("src").join("tests").join("book-noneng-main");

    let mut proj = MDBook::new(&path);
    proj.read_config();

    let t = proj.translations.get("hu").unwrap();
    assert!(t.config.is_main_book);
}

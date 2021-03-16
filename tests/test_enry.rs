use rstest::rstest;

use enry::{
    get_color,
    get_language,
    get_language_by_content,
    get_language_by_emacs_modeline,
    get_language_by_extension,
    get_language_by_filename,
    get_language_by_modeline,
    get_language_by_shebang,
    get_language_by_vim_modeline,
    get_language_extensions,
    get_languages,
    get_mime_type,
    is_binary,
    is_configuration,
    is_documentation,
    is_dot_file,
    is_image,
};

#[rstest(
    filename, content, language,
    case("test.py", "import os", "Python"),
    case("", "#!/usr/bin/bash", "Shell"),
    case("test.hs", "", "Haskell"),
)]
fn test_get_language(filename: &str, content: &str, language: &str) {
    assert_eq!(get_language(filename, content).unwrap(), language);
}


#[rstest]
fn test_get_language_by_filename() {
    const FILENAME: &str = "pom.xml";
    const LANGUAGE: &str = "Maven POM";
    const SAFE: bool = true;
    let guess = get_language_by_filename(FILENAME).unwrap();
    assert_eq!(guess.language, LANGUAGE);
    assert_eq!(guess.safe, SAFE);
}


#[rstest]
fn test_get_language_by_content() {
    const FILENAME: &str = "test.php";
    const CONTENT: &str = "<?php $foo = bar();";
    const LANGUAGE: &str = "PHP";
    const SAFE: bool = true;
    let guess = get_language_by_content(FILENAME, CONTENT).unwrap();
    assert_eq!(guess.language, LANGUAGE);
    assert_eq!(guess.safe, SAFE);
}


#[rstest]
fn test_get_language_by_emacs_modeline() {
    const MODELINE: &str = "// -*- font:bar;mode:c++ -*-\ntemplate <typename X> class { X i; };";
    const LANGUAGE: &str = "C++";
    const SAFE: bool = true;
    let guess = get_language_by_emacs_modeline(MODELINE).unwrap();
    assert_eq!(guess.language, LANGUAGE);
    assert_eq!(guess.safe, SAFE);
}


#[rstest]
fn test_get_language_by_vim_modeline() {
    const MODELINE: &str = "# vim: noexpandtab: ft=javascript";
    const LANGUAGE: &str = "JavaScript";
    const SAFE: bool = true;
    let guess = get_language_by_vim_modeline(MODELINE).unwrap();
    assert_eq!(guess.language, LANGUAGE);
    assert_eq!(guess.safe, SAFE);
}


#[rstest(
    modeline, language, safe,
    case("// -*- font:bar;mode:c++ -*-\ntemplate <typename X> class { X i; };", "C++", true),
    case("# vim: noexpandtab: ft=javascript", "JavaScript", true),
)]
fn test_get_language_by_modeline(modeline: &str, language: &str, safe: bool) {
    let guess = get_language_by_modeline(modeline).unwrap();
    assert_eq!(guess.language, language);
    assert_eq!(guess.safe, safe);
}

#[rstest(
    filename, language, safe,
    case("test.lisp", "Common Lisp", false),
    case("test.path", "", false),
)]
fn test_get_language_by_extension(filename: &str, language: &str, safe: bool) {
    let guess = get_language_by_extension(filename).unwrap();
    assert_eq!(guess.language, language);
    assert_eq!(guess.safe, safe);
}

#[rstest]
fn test_get_language_by_shebang() {
    const SHEBANG: &str = "#!/usr/bin/python3";
    const LANGUAGE: &str = "Python";
    const SAFE: bool = true;
    let guess = get_language_by_shebang(SHEBANG).unwrap();
    assert_eq!(guess.language, LANGUAGE);
    assert_eq!(guess.safe, SAFE);
}


#[rstest]
fn test_get_mime_type() {
    const FILENAME: &str = "test.rb";
    const LANGUAGE: &str = "Ruby";
    const MIME_TYPE: &str = "text/x-ruby";
    assert_eq!(get_mime_type(FILENAME, LANGUAGE).unwrap(), MIME_TYPE);
}


#[rstest]
fn test_is_binary() {
    const CONTENT: &str = "println!('Hello world!\n');";
    const IS_BINARY: bool = false;
    assert_eq!(is_binary(CONTENT).unwrap(), IS_BINARY);
}


#[rstest(
    path, is_documentation_actual,
    case("sss/documentation/", true),
    case("docs/", true),
    case("test/", false),
)]
fn test_is_documentation(path: &str, is_documentation_actual: bool) {
    assert_eq!(is_documentation(path).unwrap(), is_documentation_actual);
}


#[rstest(
    path, is_dot_actual,
    case(".env", true),
    case("something.py", false),
)]
fn test_is_dot(path: &str, is_dot_actual: bool) {
    assert_eq!(is_dot_file(path).unwrap(), is_dot_actual);
}


#[rstest(
    path, is_config_actual,
    case("configuration.yml", true),
    case("some_code.py", false),
)]
fn test_is_configuration(path: &str, is_config_actual: bool) {
    assert_eq!(is_configuration(path).unwrap(), is_config_actual);
}


#[rstest(
    path, is_image_actual,
    case("sfw.jpg", true),
    case("shrek-picture.png", true),
    case("openjdk-1000.parquet", false),
)]
fn test_is_image(path: &str, is_image_actual: bool) {
    assert_eq!(is_image(path).unwrap(), is_image_actual);
}


#[rstest]
fn test_get_color() {
    const LANGUAGE: &str = "Go";
    const COLOR: &str = "#00ADD8";
    assert_eq!(get_color(LANGUAGE).unwrap(), COLOR);
}


#[rstest]
fn test_get_languages() {
    const FILENAME: &str = "test.py";
    const CONTENT: &str = "import os";
    const LANGUAGE: &str = "Python";
    assert_eq!(get_languages(FILENAME, CONTENT).unwrap(), [LANGUAGE]);
}


#[rstest]
fn test_get_language_extensions() {
    const LANGUAGE: &str = "Python";
    const EXTENSIONS: [&str; 18] = [
        ".py", ".cgi", ".fcgi", ".gyp", ".gypi", ".lmi", ".py3", ".pyde",
        ".pyi", ".pyp", ".pyt", ".pyw", ".rpy", ".smk", ".spec", ".tac",
        ".wsgi", ".xpy"
    ];
    assert_eq!(get_language_extensions(LANGUAGE).unwrap(), EXTENSIONS);
}

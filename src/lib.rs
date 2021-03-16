use std::ffi::{c_void, CStr, CString};
use std::io::{BufRead, Error, Read};
use std::os::raw::{c_char, c_uchar, c_uint};
use std::ptr::{null, null_mut};

use crate::go::slice::{GoSlice, ToGoSlice};
use crate::go::string::{GoString, ToGoString};
use crate::go::guess::{GoGuess, Guess};

mod go;

extern "C" {
    fn GetLanguage(filename: GoString, content: GoSlice) -> GoString;
    fn GetMimeType(path: GoString, language: GoString) -> GoString;
    fn GetLanguages(filename: GoString, content: GoSlice, result: &mut GoSlice);
    fn GetLanguageExtensions(language: GoString, result: &mut GoSlice);

    fn GetLanguageByContent(filename: GoString, content: GoSlice) -> GoGuess;
    fn GetLanguageByExtension(filename: GoString) -> GoGuess;
    fn GetLanguageByFilename(filename: GoString) -> GoGuess;
    fn GetLanguageByModeline(content: GoSlice) -> GoGuess;
    fn GetLanguageByShebang(content: GoSlice) -> GoGuess;
    fn GetLanguageByVimModeline(content: GoSlice) -> GoGuess;
    fn GetLanguageByEmacsModeline(content: GoSlice) -> GoGuess;

    fn IsBinary(data: GoSlice) -> c_uchar;
    fn IsConfiguration(path: GoString) -> c_uchar;
    fn IsDocumentation(path: GoString) -> c_uchar;
    fn IsDotFile(path: GoString) -> c_uchar;
    fn IsImage(path: GoString) -> c_uchar;
    fn IsVendor(path: GoString) -> c_uchar;
    fn IsGenerated(path: GoString, content: GoSlice) -> c_uchar;
    fn GetColor(language: GoString) -> GoString;

    // Additional internal functions possible to define
    // fn GetLanguagesByContent(filename: GoString, content: GoSlice, candidates: GoSlice, result: &mut GoSlice);
    // fn GetLanguagesByEmacsModeline(filename: GoString, content: GoSlice, candidates: GoSlice, result: &mut GoSlice);
    // fn GetLanguagesByExtension(filename: GoString, content: GoSlice, candidates: GoSlice, result: &mut GoSlice);
    // fn GetLanguagesByFilename(filename: GoString, content: GoSlice, candidates: GoSlice, result: &mut GoSlice);
    // fn GetLanguagesByModeline(filename: GoString, content: GoSlice, candidates: GoSlice, result: &mut GoSlice);
    // fn GetLanguagesByShebang(filename: GoString, content: GoSlice, candidates: GoSlice, result: &mut GoSlice);
    // fn GetLanguagesByVimModeline(filename: GoString, content: GoSlice, candidates: GoSlice, result: &mut GoSlice);
}

pub fn get_languages(filename: &str, content: &str) -> Vec<String> {
    let c_filename = CString::new(filename).expect("Can't construct string");
    let c_content = CString::new(content).expect("Can't construct content string");
    let mut go_result = GoSlice::default();
    unsafe {
        GetLanguages(c_filename.as_go_string(), c_content.as_go_slice(), &mut go_result);
        Vec::from(go_result)
    }
}

pub fn get_language_by_content(filename: &str, content: &str) -> Guess {
    let c_filename = CString::new(filename).expect("Can't construct string");
    let c_content = CString::new(content).expect("Can't construct content string");
    unsafe {
        Guess::from(
            GetLanguageByContent(
                c_filename.as_go_string(),
                c_content.as_go_slice(),
            )
        )
    }
}

pub fn get_language_extensions(language: &str) -> Vec<String> {
    let c_language = CString::new(language).unwrap();
    let go_language = c_language.as_go_string();
    let mut go_result = GoSlice::default();
    unsafe {
        GetLanguageExtensions(go_language, &mut go_result);
        Vec::from(go_result)
    }
}

pub fn get_language(filename: &str, content: &str) -> String {
    let c_filename = CString::new(filename).unwrap();
    let c_content = CString::new(content).unwrap();
    unsafe {
        GetLanguage(c_filename.as_go_string(), c_content.as_go_slice()).to_string()
    }
}

pub fn get_mime_type(path: &str, language: &str) -> String {
    let c_path = CString::new(path).unwrap();
    let c_language = CString::new(language).unwrap();
    unsafe {
        GetMimeType(c_path.as_go_string(), c_language.as_go_string()).to_string()
    }
}

pub fn get_language_by_extension(filename: &str) -> Guess {
    let c_filename = CString::new(filename).unwrap();
    unsafe { Guess::from(GetLanguageByExtension(c_filename.as_go_string())) }
}

pub fn get_language_by_filename(filename: &str) -> Guess {
    let c_filename = CString::new(filename).unwrap();
    unsafe { Guess::from(GetLanguageByFilename(c_filename.as_go_string())) }
}

pub fn get_language_by_modeline(content: &str) -> Guess {
    let c_content = CString::new(content).unwrap();
    unsafe { Guess::from(GetLanguageByModeline(c_content.as_go_slice())) }
}

pub fn get_language_by_shebang(content: &str) -> Guess {
    let c_content = CString::new(content).unwrap();
    unsafe { Guess::from(GetLanguageByShebang(c_content.as_go_slice())) }
}

pub fn get_language_by_vim_modeline(content: &str) -> Guess {
    let c_content = CString::new(content).unwrap();
    unsafe { Guess::from(GetLanguageByVimModeline(c_content.as_go_slice())) }
}

pub fn get_language_by_emacs_modeline(content: &str) -> Guess {
    let c_content = CString::new(content).unwrap();
    unsafe { Guess::from(GetLanguageByEmacsModeline(c_content.as_go_slice())) }
}

pub fn is_binary(data: &str) -> bool {
    let c_data = CString::new(data).unwrap();
    unsafe { IsBinary(c_data.as_go_slice()) == 1 }
}

pub fn is_configuration(path: &str) -> bool {
    let c_path = CString::new(path).unwrap();
    unsafe { IsConfiguration(c_path.as_go_string()) == 1 }
}

pub fn is_documentation(path: &str) -> bool {
    let c_path = CString::new(path).unwrap();
    unsafe { IsDocumentation(c_path.as_go_string()) == 1 }
}

pub fn is_dot_file(path: &str) -> bool {
    let c_path = CString::new(path).unwrap();
    unsafe { IsDotFile(c_path.as_go_string()) == 1 }
}

pub fn is_image(path: &str) -> bool {
    let c_path = CString::new(path).unwrap();
    unsafe { IsImage(c_path.as_go_string()) == 1 }
}

pub fn is_vendor(path: &str) -> bool {
    let c_path = CString::new(path).unwrap();
    unsafe { IsVendor(c_path.as_go_string()) == 1 }
}

pub fn is_generated(path: &str, content: &str) -> bool {
    let c_path = CString::new(path).unwrap();
    let c_content = CString::new(content).unwrap();
    unsafe { IsGenerated(c_path.as_go_string(), c_content.as_go_slice()) == 1}
}

pub fn get_color(language: &str) -> String {
    let c_language = CString::new(language).unwrap();
    unsafe { GetColor(c_language.as_go_string()).to_string() }
}
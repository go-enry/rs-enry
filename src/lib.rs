use std::ffi::{CString, NulError};
use std::os::raw::c_uchar;

use crate::go::guess::GoGuess;
use crate::go::slice::{GoSlice, ToGoSlice};
use crate::go::string::{GoString, ToGoString};

pub use go::guess::Guess;

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

/// `get_languages()` applies a sequence of strategies based on the given filename and
/// content to find out the most probable languages to return.
///
/// If it finds a strategy that produces a single result, it will be returned;
/// otherise the last strategy that returned multiple results will be returned.
/// If the content is binary, no results will be returned. This matches the
/// behavior of [Linguist.detect]
///
/// At least one of arguments should be set. If content is missing, language
/// detection will be based on the filename. The function won't read the file, given an empty content.
///
/// [Linguist.detect]: https://github.com/github/linguist/blob/aad49acc0624c70d654a8dce447887dbbc713c7a/lib/linguist.rb#L14-L49
pub fn get_languages<S: AsRef<str>, B: AsRef<[u8]>>(
    filename: S,
    content: B,
) -> Result<Vec<String>, NulError> {
    let c_filename = CString::new(filename.as_ref()).expect("Can't construct string");
    let c_content = CString::new(content.as_ref()).expect("Can't construct content string");
    let mut go_result = GoSlice::default();
    unsafe {
        GetLanguages(
            c_filename.as_go_string(),
            c_content.as_go_slice(),
            &mut go_result,
        );
        Ok(Vec::from(go_result))
    }
}

/// `get_languages_by_content()` returns a slice of languages for the given
/// content. It is a Strategy that uses content-based regexp heuristics and a
/// filename extension.
///
/// If there are more than one possibles languages it returns the first language
/// by alphabetically order and the `safe` field of `Guess` will be set to false.
pub fn get_language_by_content<S: AsRef<str>, B: AsRef<[u8]>>(
    filename: S,
    content: B,
) -> Result<Guess, NulError> {
    let c_filename = CString::new(filename.as_ref())?;
    let c_content = CString::new(content.as_ref())?;
    unsafe {
        Ok(Guess::from(GetLanguageByContent(
            c_filename.as_go_string(),
            c_content.as_go_slice(),
        )))
    }
}

/// `get_language_extensions()` returns all extensions associated with the given language.
pub fn get_language_extensions<S: AsRef<str>>(language: S) -> Result<Vec<String>, NulError> {
    let c_language = CString::new(language.as_ref())?;
    let mut go_result = GoSlice::default();
    unsafe {
        GetLanguageExtensions(c_language.as_go_string(), &mut go_result);
        Ok(Vec::from(go_result))
    }
}

/// `get_language()` applies a sequence of strategies based on the given filename
/// and content to find out the most probable language to return.
pub fn get_language<S: AsRef<str>, B: AsRef<[u8]>>(
    filename: S,
    content: B,
) -> Result<String, NulError> {
    let c_filename = CString::new(filename.as_ref())?;
    let c_content = CString::new(content.as_ref())?;
    unsafe { Ok(GetLanguage(c_filename.as_go_string(), c_content.as_go_slice()).to_string()) }
}

/// `get_mime_type()` returns a MIME type of a given file based on its languages.
pub fn get_mime_type<S: AsRef<str>>(path: S, language: S) -> Result<String, NulError> {
    let c_path = CString::new(path.as_ref())?;
    let c_language = CString::new(language.as_ref())?;
    unsafe { Ok(GetMimeType(c_path.as_go_string(), c_language.as_go_string()).to_string()) }
}

/// `get_language_by_extension()` returns detected language.
///
/// If there are more than one possibles languages it returns the first language
/// by alphabetically order and the `safe` field of `Guess` will be set to false.
pub fn get_language_by_extension<S: AsRef<str>>(filename: S) -> Result<Guess, NulError> {
    let c_filename = CString::new(filename.as_ref())?;
    unsafe {
        Ok(Guess::from(GetLanguageByExtension(
            c_filename.as_go_string(),
        )))
    }
}

/// `get_language_by_filename()` returns detected language.
///
/// If there are more than one possibles languages it returns the first language
/// by alphabetically order and the `safe` field of `Guess` will be set to false.
pub fn get_language_by_filename<S: AsRef<str>>(filename: S) -> Result<Guess, NulError> {
    let c_filename = CString::new(filename.as_ref())?;
    unsafe {
        Ok(Guess::from(GetLanguageByFilename(
            c_filename.as_go_string(),
        )))
    }
}

/// `get_language_by_modeline()` returns detected language.
///
/// If there are more than one possibles languages it returns the first language
/// by alphabetically order and the `safe` field of `Guess` will be set to false.
pub fn get_language_by_modeline<B: AsRef<[u8]>>(content: B) -> Result<Guess, NulError> {
    let c_content = CString::new(content.as_ref())?;
    unsafe { Ok(Guess::from(GetLanguageByModeline(c_content.as_go_slice()))) }
}

/// `get_language_by_shebang()` returns detected language.
///
/// If there are more than one possibles languages it returns the first language
/// by alphabetically order and the `safe` field of `Guess` will be set to false.
pub fn get_language_by_shebang<B: AsRef<[u8]>>(content: B) -> Result<Guess, NulError> {
    let c_content = CString::new(content.as_ref())?;
    unsafe { Ok(Guess::from(GetLanguageByShebang(c_content.as_go_slice()))) }
}

/// `get_language_by_vim_modeline()` returns detected language.
///
/// If there are more than one possibles languages it returns the first language
/// by alphabetically order and the `safe` field of `Guess` will be set to false.
pub fn get_language_by_vim_modeline<B: AsRef<[u8]>>(content: B) -> Result<Guess, NulError> {
    let c_content = CString::new(content.as_ref())?;
    unsafe {
        Ok(Guess::from(GetLanguageByVimModeline(
            c_content.as_go_slice(),
        )))
    }
}

/// `get_language_by_emacs_modeline()` returns detected language.
///
/// If there are more than one possibles languages it returns the first language
/// by alphabetically order and the `safe` field of `Guess` will be set to false.
pub fn get_language_by_emacs_modeline<B: AsRef<[u8]>>(content: B) -> Result<Guess, NulError> {
    let c_content = CString::new(content.as_ref())?;
    unsafe {
        Ok(Guess::from(GetLanguageByEmacsModeline(
            c_content.as_go_slice(),
        )))
    }
}

/// `is_binary()` detects if data is a binary value based
/// on this [code snippet](http://git.kernel.org/cgit/git/git.git/tree/xdiff-interface.c?id=HEAD#n198).
pub fn is_binary<B: AsRef<[u8]>>(data: B) -> Result<bool, NulError> {
    let c_data = CString::new(data.as_ref())?;
    unsafe { Ok(IsBinary(c_data.as_go_slice()) == 1) }
}

/// `is_configuration()` tells if filename is in one of the configuration languages.
pub fn is_configuration<S: AsRef<str>>(path: S) -> Result<bool, NulError> {
    let c_path = CString::new(path.as_ref())?;
    unsafe { Ok(IsConfiguration(c_path.as_go_string()) == 1) }
}

/// `is_documentation()` returns whether or not path is a documentation path.
pub fn is_documentation<S: AsRef<str>>(path: S) -> Result<bool, NulError> {
    let c_path = CString::new(path.as_ref())?;
    unsafe { Ok(IsDocumentation(c_path.as_go_string()) == 1) }
}

/// `is_dot_file()` returns whether or not path has dot as a prefix.
pub fn is_dot_file<S: AsRef<str>>(path: S) -> Result<bool, NulError> {
    let c_path = CString::new(path.as_ref())?;
    unsafe { Ok(IsDotFile(c_path.as_go_string()) == 1) }
}

/// `is_image()` tells if a given file is an image (PNG, JPEG or GIF format).
pub fn is_image<S: AsRef<str>>(path: S) -> Result<bool, NulError> {
    let c_path = CString::new(path.as_ref())?;
    unsafe { Ok(IsImage(c_path.as_go_string()) == 1) }
}

/// `is_vendor()` returns whether or not path is a vendor path.
pub fn is_vendor<S: AsRef<str>>(path: S) -> Result<bool, NulError> {
    let c_path = CString::new(path.as_ref())?;
    unsafe { Ok(IsVendor(c_path.as_go_string()) == 1) }
}

/// `is_generated()` returns whether the file with the given path and content
/// is a generated file.
pub fn is_generated<S: AsRef<str>, B: AsRef<[u8]>>(path: S, content: B) -> Result<bool, NulError> {
    let c_path = CString::new(path.as_ref())?;
    let c_content = CString::new(content.as_ref())?;
    unsafe { Ok(IsGenerated(c_path.as_go_string(), c_content.as_go_slice()) == 1) }
}

/// `get_color()` returns the HTML color code of a given language.
pub fn get_color<S: AsRef<str>>(language: S) -> Result<String, NulError> {
    let c_language = CString::new(language.as_ref())?;
    unsafe { Ok(GetColor(c_language.as_go_string()).to_string()) }
}

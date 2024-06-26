use thiserror::Error;

/// Internal Error Type
#[derive(Debug, Error)]
pub enum ChorError {
    /// Error thrown when parsing a NoteName from a String.
    NoteNameParse(<super::NoteName as std::str::FromStr>::Err),
    /// Error thrown when parsing a NoteName from a German String.
    NoteNameParseGerman,
    /// Error thrown when parsing a Note's Octave from a String.
    NoteOctaveParse(<i32 as std::str::FromStr>::Err),
    /// Error thrown when attempting to create a multinote that does not exist.
    NotAValidMultinote(String),
}

impl std::fmt::Display for ChorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChorError::NoteNameParse(e) => e.fmt(f),
            ChorError::NoteOctaveParse(e) => e.fmt(f),
            ChorError::NotAValidMultinote(err) => err.fmt(f),
            ChorError::NoteNameParseGerman => f.write_str("German note parsing failed"),
        }
    }
}

// TODO: Thiserror

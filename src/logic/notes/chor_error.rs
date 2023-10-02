/// Internal Error Type
#[derive(Debug)]
pub enum ChorError {
    /// Error thrown when parsing a NoteName from a String.
    NoteNameParseError(<super::NoteName as std::str::FromStr>::Err),
    /// Error thrown when parsing a Note's Octave from a String.
    NoteOctaveParseError(<i32 as std::str::FromStr>::Err),
}

// TODO: Thiserror

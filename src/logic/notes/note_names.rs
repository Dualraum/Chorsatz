use super::chor_error;
use strum_macros::Display;

/// Names of notes that can be compared to each other.
#[derive(
    Debug, Clone, Copy, Eq, strum_macros::EnumString, strum_macros::EnumIter, Display, Default,
)]
#[allow(dead_code)]
pub enum NoteName {
    #[default]
    C,
    Cs,
    Css,
    Dff,
    Df,
    D,
    Ds,
    Dss,
    Eff,
    Ef,
    E,
    Es,
    Ess,
    Fff,
    Ff,
    F,
    Fs,
    Fss,
    Gff,
    Gf,
    G,
    Gs,
    Gss,
    Aff,
    Af,
    A,
    As,
    Ass,
    Bff,
    Bf,
    B,
    Bs,
    Bss,
    Cff,
    Cf,
}

impl NoteName {
    /// Returns the position of this note in the Cmajor scale, as well as if this movement moves it down up one scale
    pub fn to_c_major_position_shift(self) -> (usize, i32) {
        (
            match self {
                NoteName::C => 0,
                NoteName::Cs => 1,
                NoteName::Css => 2,
                NoteName::Dff => 0,
                NoteName::Df => 1,
                NoteName::D => 2,
                NoteName::Ds => 3,
                NoteName::Dss => 4,
                NoteName::Eff => 2,
                NoteName::Ef => 3,
                NoteName::E => 4,
                NoteName::Es => 5,
                NoteName::Ess => 6,
                NoteName::Fff => 3,
                NoteName::Ff => 4,
                NoteName::F => 5,
                NoteName::Fs => 6,
                NoteName::Fss => 7,
                NoteName::Gff => 5,
                NoteName::Gf => 6,
                NoteName::G => 7,
                NoteName::Gs => 8,
                NoteName::Gss => 9,
                NoteName::Aff => 7,
                NoteName::Af => 8,
                NoteName::A => 9,
                NoteName::As => 10,
                NoteName::Ass => 11,
                NoteName::Bff => 9,
                NoteName::Bf => 10,
                NoteName::B => 11,
                NoteName::Bs => 0,
                NoteName::Bss => 1,
                NoteName::Cff => 10,
                NoteName::Cf => 11,
            },
            match self {
                NoteName::Bs | NoteName::Bss => 1,
                NoteName::Cf | NoteName::Css => -1,
                _ => 0,
            },
        )
    }

    /// Returns how many full note lines above a C this note would lie, along with the number of #/b signs in front of it.
    pub fn get_note_line_and_sign(&self) -> (f32, f32) {
        match self {
            NoteName::C => (0., 0.),
            NoteName::Cs => (0., 0.5),
            NoteName::Css => (0., 1.),
            NoteName::Dff => (1., -1.),
            NoteName::Df => (1., -0.5),
            NoteName::D => (1., 0.),
            NoteName::Ds => (1., 0.5),
            NoteName::Dss => (1., 1.),
            NoteName::Eff => (2., -1.),
            NoteName::Ef => (2., -0.5),
            NoteName::E => (2., 0.),
            NoteName::Es => (2., 0.5),
            NoteName::Ess => (2., 1.),
            NoteName::Fff => (3., -1.),
            NoteName::Ff => (3., -0.5),
            NoteName::F => (3., 0.),
            NoteName::Fs => (3., 0.5),
            NoteName::Fss => (3., 1.),
            NoteName::Gff => (4., -1.),
            NoteName::Gf => (4., -0.5),
            NoteName::G => (4., 0.),
            NoteName::Gs => (4., 0.5),
            NoteName::Gss => (4., 1.),
            NoteName::Aff => (5., -1.),
            NoteName::Af => (5., -0.5),
            NoteName::A => (5., 0.),
            NoteName::As => (5., 0.5),
            NoteName::Ass => (5., 1.),
            NoteName::Bff => (6., -1.),
            NoteName::Bf => (6., -0.5),
            NoteName::B => (6., 0.),
            NoteName::Bs => (6., 0.5),
            NoteName::Bss => (6., 1.0),
            NoteName::Cff => (0., -1.),
            NoteName::Cf => (0., -0.5),
        }
    }

    pub fn to_playable_note(self) -> String {
        match self {
            NoteName::C => "c",
            NoteName::Cs => "cs",
            NoteName::Css => "d",
            NoteName::Dff => "c",
            NoteName::Df => "cs",
            NoteName::D => "d",
            NoteName::Ds => "ds",
            NoteName::Dss => "e",
            NoteName::Eff => "d",
            NoteName::Ef => "ds",
            NoteName::E => "e",
            NoteName::Es => "f",
            NoteName::Ess => "fs",
            NoteName::Fff => "ds",
            NoteName::Ff => "e",
            NoteName::F => "f",
            NoteName::Fs => "fs",
            NoteName::Fss => "g",
            NoteName::Gff => "f",
            NoteName::Gf => "fs",
            NoteName::G => "g",
            NoteName::Gs => "gs",
            NoteName::Gss => "a",
            NoteName::Aff => "g",
            NoteName::Af => "gs",
            NoteName::A => "a",
            NoteName::As => "as",
            NoteName::Ass => "b",
            NoteName::Bff => "a",
            NoteName::Bf => "as",
            NoteName::B => "b",
            NoteName::Bs => "c",
            NoteName::Bss => "cs",
            NoteName::Cff => "as",
            NoteName::Cf => "b",
        }
        .to_string()
    }

    /// Converts the note to its german name.
    pub fn to_german_note(self) -> String {
        match self {
            NoteName::C => "C",
            NoteName::Cs => "Cis",
            NoteName::Css => "Cisis",
            NoteName::Dff => "Deses",
            NoteName::Df => "Des",
            NoteName::D => "D",
            NoteName::Ds => "Dis",
            NoteName::Dss => "Disis",
            NoteName::Eff => "Eses",
            NoteName::Ef => "Es",
            NoteName::E => "E",
            NoteName::Es => "Eis",
            NoteName::Ess => "Eisis",
            NoteName::Fff => "Feses",
            NoteName::Ff => "Fes",
            NoteName::F => "F",
            NoteName::Fs => "Fis",
            NoteName::Fss => "Fisis",
            NoteName::Gff => "Geses",
            NoteName::Gf => "Ges",
            NoteName::G => "G",
            NoteName::Gs => "Gis",
            NoteName::Gss => "Gisis",
            NoteName::Aff => "Asas",
            NoteName::Af => "As",
            NoteName::A => "A",
            NoteName::As => "Ais",
            NoteName::Ass => "Aisis",
            NoteName::Bff => "Heses",
            NoteName::Bf => "B",
            NoteName::B => "H",
            NoteName::Bs => "His",
            NoteName::Bss => "Hisis",
            NoteName::Cff => "Ceses",
            NoteName::Cf => "Ces",
        }
        .to_string()
    }

    /// Converts a german name to a note.
    pub fn from_german_name(s: &str) -> Result<Self, chor_error::ChorError> {
        Ok(match s {
            "C" => NoteName::C,
            "Cis" => NoteName::Cs,
            "Cisis" => NoteName::Css,
            "Deses" => NoteName::Dff,
            "Des" => NoteName::Df,
            "D" => NoteName::D,
            "Dis" => NoteName::Ds,
            "Disis" => NoteName::Dss,
            "Eses" => NoteName::Eff,
            "Es" => NoteName::Ef,
            "E" => NoteName::E,
            "Eis" => NoteName::Es,
            "Eisis" => NoteName::Ess,
            "Feses" => NoteName::Fff,
            "Fes" => NoteName::Ff,
            "F" => NoteName::F,
            "Fis" => NoteName::Fs,
            "Fisis" => NoteName::Fss,
            "Geses" => NoteName::Gff,
            "Ges" => NoteName::Gf,
            "G" => NoteName::G,
            "Gis" => NoteName::Gs,
            "Gisis" => NoteName::Gss,
            "Asas" => NoteName::Aff,
            "As" => NoteName::Af,
            "A" => NoteName::A,
            "Ais" => NoteName::As,
            "Aisis" => NoteName::Ass,
            "Heses" => NoteName::Bff,
            "B" => NoteName::Bf,
            "H" => NoteName::B,
            "His" => NoteName::Bs,
            "Hisis" => NoteName::Bss,
            "Ceses" => NoteName::Cff,
            "Ces" => NoteName::Cf,
            _ => {
                return Err(chor_error::ChorError::NoteNameParseGerman);
            }
        })
    }
}

impl PartialEq for NoteName {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
            || self.to_c_major_position_shift().0 == other.to_c_major_position_shift().0
    }
}

impl std::hash::Hash for NoteName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[test]
fn note_compare_test() {
    assert_eq!(NoteName::C, NoteName::C);
    assert_eq!(NoteName::Cs, NoteName::Cs);
    assert_eq!(NoteName::Cs, NoteName::Df);
    assert_eq!(NoteName::Df, NoteName::Cs);
    assert_eq!(NoteName::Cs, NoteName::Df);
    assert_eq!(NoteName::Df, NoteName::Cs);
    assert_eq!(NoteName::F, NoteName::Es);
    assert_eq!(NoteName::As, NoteName::Bf);
    assert_eq!(NoteName::Cf, NoteName::B);
}

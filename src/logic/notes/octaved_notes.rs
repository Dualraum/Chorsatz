use std::{fmt::Display, str::FromStr};

use super::NoteName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OctavedNote {
    pub note: NoteName,
    pub octave: i32,
}

impl OctavedNote {
    pub fn new(note: NoteName, octave: i32) -> Self {
        Self { note, octave }
    }

    pub fn get_value(&self) -> f32 {
        // get the position in the C-Dur scale and, for some notes with 'overflow' if there is an octave shift
        let (position, shift) = self.note.to_c_major_position_shift();
        // use the position to get the half tones and shift by the appropriate amount of octaves
        super::scale::POSITION_TO_HALFTONES[position]
        // add octave
         + (7 * (self.octave + shift)) as f32
    }

    // Returns, how many full note lines above C0 this note would lie, along with the number of #/b signs in front of it.
    pub fn get_note_line_and_sign(&self) -> (f32, f32) {
        let (line, signs) = self.note.get_note_line_and_sign();
        (line + 7. * self.octave as f32, signs)
    }

    pub fn to_playable_note(self) -> String {
        self.note.to_playable_note() + &(self.octave + 3).to_string()
    }

    pub fn to_german_name(self) -> String {
        format!("{}{}", self.note.to_german_note(), self.octave)
    }
}

impl FromStr for OctavedNote {
    type Err = super::ChorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.find(|c: char| c.is_numeric() || c == '-') {
            Some(index) => OctavedNote {
                note: s[0..index]
                    .parse()
                    .map_err(super::ChorError::NoteNameParse)?,
                octave: s[index..]
                    .parse()
                    .map_err(super::ChorError::NoteOctaveParse)?,
            },
            None => OctavedNote {
                note: s.parse().map_err(super::ChorError::NoteNameParse)?,
                octave: 0,
            },
        })
    }
}

impl Display for OctavedNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.note, self.octave)
    }
}

impl std::ops::Sub<OctavedNote> for OctavedNote {
    type Output = f32;

    fn sub(self, rhs: OctavedNote) -> Self::Output {
        let true_diff = self.get_value() - rhs.get_value();
        true_diff + 1_f32.copysign(true_diff)
    }
}

#[test]
fn diff_test() {
    assert_eq!(
        OctavedNote::new(NoteName::C, 0) - OctavedNote::new(NoteName::C, 0),
        1.
    );

    assert_eq!(
        OctavedNote::new(NoteName::A, 0) - OctavedNote::new(NoteName::B, 0),
        -2.
    );

    assert_eq!(
        OctavedNote::new(NoteName::C, 1) - OctavedNote::new(NoteName::C, 0),
        8.
    );

    assert_eq!(
        OctavedNote::new(NoteName::As, 1) - OctavedNote::new(NoteName::Gs, 1),
        2.
    );

    assert_eq!(OctavedNote::new(NoteName::C, 0), "C0".parse().unwrap(),);

    assert_eq!(OctavedNote::new(NoteName::A, 0), "A0".parse().unwrap());
    assert_eq!(OctavedNote::new(NoteName::A, 0), "A".parse().unwrap());

    assert_eq!(OctavedNote::new(NoteName::C, 1), "C1".parse().unwrap());

    assert_eq!(OctavedNote::new(NoteName::Af, -1), "Af-1".parse().unwrap());
    assert!("Abc1".parse::<OctavedNote>().is_err());
    assert!("gde".parse::<OctavedNote>().is_err());
}

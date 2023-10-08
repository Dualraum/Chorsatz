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
        // faster but not as clean
        //scale::POSITION_TO_HALFTONES[self.note.to_c_dur_position()]
        // slower but cleaner
        super::scale::C_MAJOR.note_to_halftone(self.note)
        // add octave
         + (7 * self.octave) as f32
    }

    pub fn to_hum_note(self) -> String {
        self.note.to_hum_note() + &(self.octave + 2).to_string()
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
        OctavedNote::new(NoteName::A, 0) - OctavedNote::new(NoteName::H, 0),
        -2.
    );

    assert_eq!(
        OctavedNote::new(NoteName::C, 1) - OctavedNote::new(NoteName::C, 0),
        8.
    );

    assert_eq!(
        OctavedNote::new(NoteName::Ais, 1) - OctavedNote::new(NoteName::Gis, 1),
        2.
    );

    assert_eq!(OctavedNote::new(NoteName::C, 0), "C0".parse().unwrap(),);

    assert_eq!(OctavedNote::new(NoteName::A, 0), "A0".parse().unwrap());
    assert_eq!(OctavedNote::new(NoteName::A, 0), "A".parse().unwrap());

    assert_eq!(OctavedNote::new(NoteName::C, 1), "C1".parse().unwrap());

    assert_eq!(OctavedNote::new(NoteName::As, -1), "As-1".parse().unwrap());
    assert!("Abc1".parse::<OctavedNote>().is_err());
    assert!("gde".parse::<OctavedNote>().is_err());
}

use super::NoteName;
use crate::scale;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OctavedNote {
    pub note: NoteName,
    pub octave: i32,
}

impl std::ops::Sub<OctavedNote> for OctavedNote {
    type Output = f32;

    fn sub(self, rhs: OctavedNote) -> Self::Output {
        let d = self.get_value() - rhs.get_value();
        d + 1_f32.copysign(d)
    }
}

impl OctavedNote {
    pub fn new(note: NoteName, octave: i32) -> Self {
        Self { note, octave }
    }

    pub fn get_value(&self) -> f32 {
        // faster but not as clean
        //scale::POSITION_TO_HALFTONES[self.note.to_c_dur_position()]

        scale::C_MAJOR.note_to_halftone(self.note) + (7 * self.octave) as f32
    }
}
#[test]
fn diff_test() {
    assert_eq!(
        OctavedNote {
            note: NoteName::C,
            octave: 0
        } - OctavedNote {
            note: NoteName::C,
            octave: 0
        },
        1.
    );

    assert_eq!(
        OctavedNote {
            note: NoteName::A,
            octave: 0
        } - OctavedNote {
            note: NoteName::H,
            octave: 0
        },
        -2.
    );

    assert_eq!(
        OctavedNote {
            note: NoteName::C,
            octave: 1
        } - OctavedNote {
            note: NoteName::C,
            octave: 0
        },
        8.
    );

    assert_eq!(
        OctavedNote {
            note: NoteName::Ais,
            octave: 1
        } - OctavedNote {
            note: NoteName::Gis,
            octave: 1
        },
        2.
    );
}

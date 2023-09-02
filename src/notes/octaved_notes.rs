use super::NoteName;
use crate::scale;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        scale::C_MAJOR.note_to_halftone(self.note)
        // add octave
         + (7 * self.octave) as f32
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
}

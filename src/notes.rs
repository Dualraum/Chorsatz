use itertools::Itertools;

use crate::scale;

/// Names of notes that can be compared to each other.
#[derive(Debug, Clone, Copy, Eq)]
#[allow(dead_code)]
pub enum NoteName {
    C,
    Cis,
    Des,
    D,
    Dis,
    Es,
    E,
    F,
    Eis,
    Fis,
    Ges,
    G,
    Gis,
    As,
    A,
    Ais,
    B,
    H,
    Ces,
}

pub type C_Major_Position = u32;

impl NoteName {
    pub fn to_c_dur_position(self) -> C_Major_Position {
        match self {
            NoteName::C => 0,
            NoteName::Cis => 1,
            NoteName::Des => 1,
            NoteName::D => 2,
            NoteName::Dis => 3,
            NoteName::Es => 3,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::Eis => 5,
            NoteName::Fis => 6,
            NoteName::Ges => 6,
            NoteName::G => 7,
            NoteName::Gis => 8,
            NoteName::As => 8,
            NoteName::A => 9,
            NoteName::Ais => 10,
            NoteName::B => 10,
            NoteName::H => 11,
            NoteName::Ces => 11,
        }
    }
}

impl From<NoteName> for C_Major_Position {
    fn from(value: NoteName) -> Self {
        value.to_c_dur_position()
    }
}

impl PartialEq for NoteName {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
            || self.to_c_dur_position() == other.to_c_dur_position()
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
    assert_eq!(NoteName::Cis, NoteName::Cis);
    assert_eq!(NoteName::Cis, NoteName::Des);
    assert_eq!(NoteName::Des, NoteName::Cis);
    assert_eq!(NoteName::Cis, NoteName::Des);
    assert_eq!(NoteName::Des, NoteName::Cis);
    assert_eq!(NoteName::F, NoteName::Eis);
    assert_eq!(NoteName::Ais, NoteName::B);
    assert_eq!(NoteName::Ces, NoteName::H);
}

pub struct OctavedNote {
    note: NoteName,
    octave: i32,
}

impl OctavedNote {
    pub fn true_diff(scale: &scale::Scale, note1: Self, note2: Self) -> f32 {
        (scale.note_to_halftone(note2.note) + (7 * note2.octave) as f32)
            - (scale.note_to_halftone(note1.note) + (7 * note1.octave) as f32)
    }

    pub fn diff(scale: &scale::Scale, note1: Self, note2: Self) -> f32 {
        let d = Self::true_diff(scale, note1, note2);
        d + 1_f32.copysign(d)
    }
}

#[test]
fn diff_test() {
    let scale_g = scale::generate_scale("G");
    let scale_c = scale::generate_scale("C");

    assert_eq!(
        OctavedNote::diff(
            &scale_c,
            OctavedNote {
                note: NoteName::C,
                octave: 0
            },
            OctavedNote {
                note: NoteName::C,
                octave: 0
            }
        ),
        1.
    );

    assert_eq!(
        OctavedNote::diff(
            &scale_g,
            OctavedNote {
                note: NoteName::A,
                octave: 0
            },
            OctavedNote {
                note: NoteName::H,
                octave: 0
            }
        ),
        2.
    );

    assert_eq!(
        OctavedNote::diff(
            &scale_c,
            OctavedNote {
                note: NoteName::C,
                octave: 1
            },
            OctavedNote {
                note: NoteName::C,
                octave: 0
            }
        ),
        -8.
    )
}

/// Last Note will be Bass-Note
pub struct Quatrain(NoteName, NoteName, NoteName, NoteName);

pub struct Triad(NoteName, NoteName, NoteName);
pub struct SATB_Block(OctavedNote, OctavedNote, OctavedNote, OctavedNote);

pub fn permute(triad: Triad) -> Vec<SATB_Block> {
    let mut base = Vec::new();
    for perm in [triad.0, triad.1, triad.2].iter().permutations(3).unique() {
        base.push(SATB_Block(
            OctavedNote {
                note: *perm[0],
                octave: 1,
            },
            OctavedNote {
                note: *perm[1],
                octave: 1,
            },
            OctavedNote {
                note: *perm[2],
                octave: 0,
            },
            OctavedNote {
                note: triad.0,
                octave: 0,
            },
        ))
    }

    base
}

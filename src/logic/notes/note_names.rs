use strum_macros::Display;

/// Names of notes that can be compared to each other.
#[derive(Debug, Clone, Copy, Eq, strum_macros::EnumString, Display, Default)]
#[allow(dead_code)]
pub enum NoteName {
    #[default]
    C,
    Cis,
    Cisis,
    Deses,
    Des,
    D,
    Dis,
    Disis,
    Eses,
    Es,
    E,
    Eis,
    Eisis,
    Feses,
    Fes,
    F,
    Fis,
    Fisis,
    Geses,
    Ges,
    G,
    Gis,
    Gisis,
    Asas,
    As,
    A,
    Ais,
    Aisis,
    Heses,
    B,
    H,
    His,
    Hisis,
    Ceses,
    Ces,
}

impl NoteName {
    pub fn to_c_dur_position_shift(self) -> (usize, i32) {
        (
            match self {
                NoteName::C => 0,
                NoteName::Cis => 1,
                NoteName::Cisis => 2,
                NoteName::Deses => 0,
                NoteName::Des => 1,
                NoteName::D => 2,
                NoteName::Dis => 3,
                NoteName::Disis => 4,
                NoteName::Eses => 2,
                NoteName::Es => 3,
                NoteName::E => 4,
                NoteName::Eis => 5,
                NoteName::Eisis => 6,
                NoteName::Feses => 3,
                NoteName::Fes => 4,
                NoteName::F => 5,
                NoteName::Fis => 6,
                NoteName::Fisis => 7,
                NoteName::Geses => 5,
                NoteName::Ges => 6,
                NoteName::G => 7,
                NoteName::Gis => 8,
                NoteName::Gisis => 9,
                NoteName::Asas => 7,
                NoteName::As => 8,
                NoteName::A => 9,
                NoteName::Ais => 10,
                NoteName::Aisis => 11,
                NoteName::Heses => 9,
                NoteName::B => 10,
                NoteName::H => 11,
                NoteName::His => 0,
                NoteName::Hisis => 1,
                NoteName::Ceses => 10,
                NoteName::Ces => 11,
            },
            match self {
                NoteName::His | NoteName::Hisis => 1,
                NoteName::Ces | NoteName::Ceses => -1,
                _ => 0,
            },
        )
    }

    // Returns, how many full note lines above a C this note would lie, along with the number of #/b signs in front of it.
    pub fn get_note_line_and_sign(&self) -> (f32, f32) {
        match self {
            NoteName::C => (0., 0.),
            NoteName::Cis => (0., 0.5),
            NoteName::Cisis => (0., 1.),
            NoteName::Deses => (1., -1.),
            NoteName::Des => (1., -0.5),
            NoteName::D => (1., 0.),
            NoteName::Dis => (1., 0.5),
            NoteName::Disis => (1., 1.),
            NoteName::Eses => (2., -1.),
            NoteName::Es => (2., -0.5),
            NoteName::E => (2., 0.),
            NoteName::Eis => (2., 0.5),
            NoteName::Eisis => (2., 1.),
            NoteName::Feses => (3., -1.),
            NoteName::Fes => (3., -0.5),
            NoteName::F => (3., 0.),
            NoteName::Fis => (3., 0.5),
            NoteName::Fisis => (3., 1.),
            NoteName::Geses => (4., -1.),
            NoteName::Ges => (4., -0.5),
            NoteName::G => (4., 0.),
            NoteName::Gis => (4., 0.5),
            NoteName::Gisis => (4., 1.),
            NoteName::Asas => (5., -1.),
            NoteName::As => (5., -0.5),
            NoteName::A => (5., 0.),
            NoteName::Ais => (5., 0.5),
            NoteName::Aisis => (5., 1.),
            NoteName::Heses => (6., -1.),
            NoteName::B => (6., -0.5),
            NoteName::H => (6., 0.),
            NoteName::His => (6., 0.5),
            NoteName::Hisis => (6., 1.0),
            NoteName::Ceses => (0., -1.),
            NoteName::Ces => (0., -0.5),
        }
    }

    pub fn to_playable_note(self) -> String {
        match self {
            NoteName::C => "C",
            NoteName::Cis => "Cs",
            NoteName::Cisis => "D",
            NoteName::Deses => "C",
            NoteName::Des => "Cs",
            NoteName::D => "D",
            NoteName::Dis => "Ds",
            NoteName::Disis => "E",
            NoteName::Eses => "D",
            NoteName::Es => "Ds",
            NoteName::E => "E",
            NoteName::Eis => "F",
            NoteName::Eisis => "Fs",
            NoteName::Feses => "Ds",
            NoteName::Fes => "E",
            NoteName::F => "F",
            NoteName::Fis => "Fs",
            NoteName::Fisis => "G",
            NoteName::Geses => "F",
            NoteName::Ges => "Fs",
            NoteName::G => "G",
            NoteName::Gis => "Gs",
            NoteName::Gisis => "A",
            NoteName::Asas => "G",
            NoteName::As => "Gs",
            NoteName::A => "A",
            NoteName::Ais => "As",
            NoteName::Aisis => "B",
            NoteName::Heses => "A",
            NoteName::B => "As",
            NoteName::H => "B",
            NoteName::His => "C",
            NoteName::Hisis => "Cs",
            NoteName::Ceses => "As",
            NoteName::Ces => "B",
        }
        .to_string()
    }
}

impl PartialEq for NoteName {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
            || self.to_c_dur_position_shift() == other.to_c_dur_position_shift()
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

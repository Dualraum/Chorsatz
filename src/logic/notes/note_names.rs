/// Names of notes that can be compared to each other.
#[derive(Debug, Clone, Copy, Eq, strum_macros::EnumString, Default)]
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
    Ceses,
    Ces,
}

impl NoteName {
    pub fn to_c_dur_position(self) -> usize {
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
            NoteName::F => 5,
            NoteName::Eisis => 6,
            NoteName::Eis => 5,
            NoteName::Feses => 3,
            NoteName::Fes => 4,
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
            NoteName::B => 10,
            NoteName::Heses => 9,
            NoteName::H => 11,
            NoteName::His => 0,
            NoteName::Ceses => 10,
            NoteName::Ces => 11,
        }
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

use super::{ChorError, NoteName};

#[derive(Debug, Clone, Copy)]
pub enum MultiNote {
    Triad(NoteName, NoteName, NoteName),
    Quatrain(NoteName, NoteName, NoteName, NoteName),
}

impl std::str::FromStr for MultiNote {
    type Err = ChorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // --------------------------------
            //              Triads
            // --------------------------------

            // Kreuztonarten
            // C
            "C" => Ok(MultiNote::Triad(NoteName::C, NoteName::E, NoteName::G)),
            "Cm" => Ok(MultiNote::Triad(NoteName::C, NoteName::Es, NoteName::G)),
            "Cdim" => Ok(MultiNote::Triad(NoteName::C, NoteName::Es, NoteName::Ges)),
            "Caug" => Ok(MultiNote::Triad(NoteName::C, NoteName::E, NoteName::Gis)),
            // G
            "G" => Ok(MultiNote::Triad(NoteName::G, NoteName::H, NoteName::D)),
            "Gm" => Ok(MultiNote::Triad(NoteName::G, NoteName::B, NoteName::D)),
            "Gdim" => Ok(MultiNote::Triad(NoteName::G, NoteName::B, NoteName::Des)),
            "Gaug" => Ok(MultiNote::Triad(NoteName::G, NoteName::H, NoteName::Dis)),
            // D
            "D" => Ok(MultiNote::Triad(NoteName::D, NoteName::Fis, NoteName::A)),
            "Dm" => Ok(MultiNote::Triad(NoteName::D, NoteName::F, NoteName::A)),
            "Ddim" => Ok(MultiNote::Triad(NoteName::D, NoteName::F, NoteName::As)),
            "Daug" => Ok(MultiNote::Triad(NoteName::D, NoteName::Fis, NoteName::Ais)),
            // A
            "A" => Ok(MultiNote::Triad(NoteName::A, NoteName::Cis, NoteName::E)),
            "Am" => Ok(MultiNote::Triad(NoteName::A, NoteName::C, NoteName::E)),
            "Adim" => Ok(MultiNote::Triad(NoteName::A, NoteName::C, NoteName::Es)),
            "Aaug" => Ok(MultiNote::Triad(NoteName::A, NoteName::Cis, NoteName::Eis)),
            // E
            "E" => Ok(MultiNote::Triad(NoteName::E, NoteName::Gis, NoteName::H)),
            "Em" => Ok(MultiNote::Triad(NoteName::E, NoteName::G, NoteName::H)),
            "Edim" => Ok(MultiNote::Triad(NoteName::E, NoteName::G, NoteName::B)),
            "Eaug" => Ok(MultiNote::Triad(NoteName::E, NoteName::Gis, NoteName::His)),
            // H
            "H" => Ok(MultiNote::Triad(NoteName::H, NoteName::Dis, NoteName::Fis)),
            "Hm" => Ok(MultiNote::Triad(NoteName::H, NoteName::D, NoteName::Fis)),
            "Hdim" => Ok(MultiNote::Triad(NoteName::H, NoteName::D, NoteName::F)),
            "Haug" => Ok(MultiNote::Triad(
                NoteName::H,
                NoteName::Dis,
                NoteName::Fisis,
            )),
            // Fis
            "Fis" => Ok(MultiNote::Triad(
                NoteName::Fis,
                NoteName::Ais,
                NoteName::Cis,
            )),
            "Fism" => Ok(MultiNote::Triad(NoteName::Fis, NoteName::A, NoteName::Cis)),
            "Fisdim" => Ok(MultiNote::Triad(NoteName::Fis, NoteName::A, NoteName::C)),
            "Fisaug" => Ok(MultiNote::Triad(
                NoteName::Fis,
                NoteName::Ais,
                NoteName::Cisis,
            )),

            // b-Tonarten
            // F
            "F" => Ok(MultiNote::Triad(NoteName::F, NoteName::A, NoteName::C)),
            "Fm" => Ok(MultiNote::Triad(NoteName::F, NoteName::As, NoteName::C)),
            "Fdim" => Ok(MultiNote::Triad(NoteName::F, NoteName::As, NoteName::Ces)),
            "Faug" => Ok(MultiNote::Triad(NoteName::F, NoteName::A, NoteName::Cis)),
            // B
            "B" => Ok(MultiNote::Triad(NoteName::B, NoteName::D, NoteName::F)),
            "Bm" => Ok(MultiNote::Triad(NoteName::B, NoteName::Des, NoteName::F)),
            "Bdim" => Ok(MultiNote::Triad(NoteName::B, NoteName::Des, NoteName::Fes)),
            "Baug" => Ok(MultiNote::Triad(NoteName::B, NoteName::D, NoteName::Fis)),
            // Es
            "Es" => Ok(MultiNote::Triad(NoteName::Es, NoteName::G, NoteName::B)),
            "Esm" => Ok(MultiNote::Triad(NoteName::Es, NoteName::Ges, NoteName::B)),
            "Esdim" => Ok(MultiNote::Triad(
                NoteName::Es,
                NoteName::Ges,
                NoteName::Heses,
            )),
            "Esaug" => Ok(MultiNote::Triad(NoteName::Es, NoteName::G, NoteName::H)),
            // As
            "As" => Ok(MultiNote::Triad(NoteName::As, NoteName::C, NoteName::Es)),
            "Asm" => Ok(MultiNote::Triad(NoteName::As, NoteName::Ces, NoteName::Es)),
            "Asdim" => Ok(MultiNote::Triad(
                NoteName::As,
                NoteName::Ces,
                NoteName::Eses,
            )),
            "Asaug" => Ok(MultiNote::Triad(NoteName::As, NoteName::C, NoteName::E)),
            // Des
            "Des" => Ok(MultiNote::Triad(NoteName::Des, NoteName::F, NoteName::As)),
            "Desm" => Ok(MultiNote::Triad(NoteName::Des, NoteName::Fes, NoteName::As)),
            "Desdim" => Ok(MultiNote::Triad(
                NoteName::Des,
                NoteName::Fes,
                NoteName::Asas,
            )),
            "Desaug" => Ok(MultiNote::Triad(NoteName::Des, NoteName::F, NoteName::A)),
            // Ges
            "Ges" => Ok(MultiNote::Triad(NoteName::Ges, NoteName::B, NoteName::Des)),
            "Gesm" => Ok(MultiNote::Triad(
                NoteName::Ges,
                NoteName::Heses,
                NoteName::Des,
            )),
            "Gesdim" => Ok(MultiNote::Triad(
                NoteName::Ges,
                NoteName::Heses,
                NoteName::Deses,
            )),
            "Gesaug" => Ok(MultiNote::Triad(NoteName::Ges, NoteName::B, NoteName::D)),
            //

            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cis" => Ok(MultiNote::Triad(
                NoteName::Cis,
                NoteName::Eis,
                NoteName::Gis,
            )),
            "Cism" => Ok(MultiNote::Triad(NoteName::Cis, NoteName::E, NoteName::Gis)),
            "Cisdim" => Ok(MultiNote::Triad(NoteName::Cis, NoteName::E, NoteName::G)),
            "Cisaug" => Ok(MultiNote::Triad(
                NoteName::Cis,
                NoteName::Eis,
                NoteName::Gisis,
            )),
            // Dis
            "Dis" => Ok(MultiNote::Triad(
                NoteName::Dis,
                NoteName::Fisis,
                NoteName::Ais,
            )),
            "Dism" => Ok(MultiNote::Triad(
                NoteName::Dis,
                NoteName::Fis,
                NoteName::Ais,
            )),
            "Disdim" => Ok(MultiNote::Triad(NoteName::Dis, NoteName::Fis, NoteName::A)),
            "Disaug" => Ok(MultiNote::Triad(
                NoteName::Dis,
                NoteName::Fisis,
                NoteName::Aisis,
            )),
            "Gis" => Ok(MultiNote::Triad(
                NoteName::Gis,
                NoteName::His,
                NoteName::Dis,
            )),
            "Gism" => Ok(MultiNote::Triad(NoteName::Gis, NoteName::H, NoteName::Dis)),
            "Gisdim" => Ok(MultiNote::Triad(NoteName::Gis, NoteName::H, NoteName::D)),
            "Gisaug" => Ok(MultiNote::Triad(
                NoteName::Gis,
                NoteName::His,
                NoteName::Disis,
            )),
            // Ais
            "Ais" => Ok(MultiNote::Triad(
                NoteName::Ais,
                NoteName::Cisis,
                NoteName::Eis,
            )),
            "Aism" => Ok(MultiNote::Triad(
                NoteName::Ais,
                NoteName::Cis,
                NoteName::Eis,
            )),
            "Aisdim" => Ok(MultiNote::Triad(NoteName::Ais, NoteName::Cis, NoteName::E)),
            "Aisaug" => Ok(MultiNote::Triad(
                NoteName::Ais,
                NoteName::Cisis,
                NoteName::Eisis,
            )),

            // sus4 and sus2

            // Kreuztonarten
            // C
            "Csus2" => Ok(MultiNote::Triad(NoteName::C, NoteName::D, NoteName::G)),
            "Csus4" => Ok(MultiNote::Triad(NoteName::C, NoteName::F, NoteName::G)),
            // G
            "Gsus2" => Ok(MultiNote::Triad(NoteName::G, NoteName::A, NoteName::D)),
            "Gsus4" => Ok(MultiNote::Triad(NoteName::G, NoteName::C, NoteName::D)),
            // D
            "Dsus2" => Ok(MultiNote::Triad(NoteName::D, NoteName::E, NoteName::A)),
            "Dsus4" => Ok(MultiNote::Triad(NoteName::D, NoteName::G, NoteName::A)),
            // A
            "Asus2" => Ok(MultiNote::Triad(NoteName::A, NoteName::H, NoteName::E)),
            "Asus4" => Ok(MultiNote::Triad(NoteName::A, NoteName::D, NoteName::E)),
            // E
            "Esus2" => Ok(MultiNote::Triad(NoteName::E, NoteName::Fis, NoteName::H)),
            "Esus4" => Ok(MultiNote::Triad(NoteName::E, NoteName::A, NoteName::H)),
            // H
            "Hsus2" => Ok(MultiNote::Triad(NoteName::H, NoteName::Cis, NoteName::Fis)),
            "Hsus4" => Ok(MultiNote::Triad(NoteName::H, NoteName::E, NoteName::Fis)),
            // Fis
            "Fissus2" => Ok(MultiNote::Triad(
                NoteName::Fis,
                NoteName::Gis,
                NoteName::Cis,
            )),
            "Fissus4" => Ok(MultiNote::Triad(NoteName::Fis, NoteName::H, NoteName::Cis)),

            // b-Tonarten
            // F
            "Fsus2" => Ok(MultiNote::Triad(NoteName::F, NoteName::G, NoteName::C)),
            "Fsus4" => Ok(MultiNote::Triad(NoteName::F, NoteName::B, NoteName::C)),
            // B
            "Bsus2" => Ok(MultiNote::Triad(NoteName::B, NoteName::C, NoteName::F)),
            "Bsus4" => Ok(MultiNote::Triad(NoteName::B, NoteName::Es, NoteName::F)),
            // Es
            "Essus2" => Ok(MultiNote::Triad(NoteName::Es, NoteName::F, NoteName::B)),
            "Essus4" => Ok(MultiNote::Triad(NoteName::Es, NoteName::As, NoteName::B)),
            // As
            "Assus2" => Ok(MultiNote::Triad(NoteName::As, NoteName::B, NoteName::Es)),
            "Assus4" => Ok(MultiNote::Triad(NoteName::As, NoteName::Des, NoteName::Es)),
            // Des
            "Dessus2" => Ok(MultiNote::Triad(NoteName::Des, NoteName::Es, NoteName::As)),
            "Dessus4" => Ok(MultiNote::Triad(NoteName::Des, NoteName::Ges, NoteName::As)),
            // Ges
            "Gessus2" => Ok(MultiNote::Triad(NoteName::Ges, NoteName::As, NoteName::Des)),
            "Gessus4" => Ok(MultiNote::Triad(NoteName::Ges, NoteName::H, NoteName::Des)),
            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cissus2" => Ok(MultiNote::Triad(
                NoteName::Cis,
                NoteName::Dis,
                NoteName::Gis,
            )),
            "Cissus4" => Ok(MultiNote::Triad(
                NoteName::Cis,
                NoteName::Fis,
                NoteName::Gis,
            )),
            // Dis
            "Dissus2" => Ok(MultiNote::Triad(
                NoteName::Dis,
                NoteName::Eis,
                NoteName::Ais,
            )),
            "Dissus4" => Ok(MultiNote::Triad(
                NoteName::Dis,
                NoteName::Gis,
                NoteName::Ais,
            )),
            //Gis
            "Gissus2" => Ok(MultiNote::Triad(
                NoteName::Gis,
                NoteName::Ais,
                NoteName::Dis,
            )),
            "Gissus4" => Ok(MultiNote::Triad(
                NoteName::Gis,
                NoteName::Cis,
                NoteName::Dis,
            )),
            // Ais
            "Aissus2" => Ok(MultiNote::Triad(
                NoteName::Ais,
                NoteName::His,
                NoteName::Eis,
            )),
            "Aissus4" => Ok(MultiNote::Triad(
                NoteName::Ais,
                NoteName::Dis,
                NoteName::Eis,
            )),

            // --------------------------------
            //            Quadrains
            // --------------------------------
            // Kreuztonarten
            //C
            "Cmaj7" => Ok(MultiNote::Quatrain(
                NoteName::C,
                NoteName::E,
                NoteName::G,
                NoteName::H,
            )),
            "C7" => Ok(MultiNote::Quatrain(
                NoteName::C,
                NoteName::E,
                NoteName::G,
                NoteName::B,
            )),
            "Cm7" => Ok(MultiNote::Quatrain(
                NoteName::C,
                NoteName::Es,
                NoteName::G,
                NoteName::B,
            )),
            "Cmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::C,
                NoteName::Es,
                NoteName::G,
                NoteName::H,
            )),
            "Cdim7" => Ok(MultiNote::Quatrain(
                NoteName::C,
                NoteName::Es,
                NoteName::Ges,
                NoteName::Heses,
            )),
            "Caug7" => Ok(MultiNote::Quatrain(
                NoteName::C,
                NoteName::E,
                NoteName::Gis,
                NoteName::H,
            )),
            // G
            "G7" => Ok(MultiNote::Quatrain(
                NoteName::G,
                NoteName::H,
                NoteName::D,
                NoteName::F,
            )),
            "Gmaj7" => Ok(MultiNote::Quatrain(
                NoteName::G,
                NoteName::H,
                NoteName::D,
                NoteName::Fis,
            )),
            "Gm7" => Ok(MultiNote::Quatrain(
                NoteName::G,
                NoteName::B,
                NoteName::D,
                NoteName::F,
            )),
            "Gmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::G,
                NoteName::B,
                NoteName::D,
                NoteName::Fis,
            )),
            "Gdim7" => Ok(MultiNote::Quatrain(
                NoteName::G,
                NoteName::B,
                NoteName::Des,
                NoteName::Fes,
            )),
            "Gaug7" => Ok(MultiNote::Quatrain(
                NoteName::G,
                NoteName::H,
                NoteName::Dis,
                NoteName::Fis,
            )),
            // D
            "D7" => Ok(MultiNote::Quatrain(
                NoteName::D,
                NoteName::Fis,
                NoteName::A,
                NoteName::C,
            )),
            "Dmaj7" => Ok(MultiNote::Quatrain(
                NoteName::D,
                NoteName::Fis,
                NoteName::A,
                NoteName::Cis,
            )),
            "Dm7" => Ok(MultiNote::Quatrain(
                NoteName::D,
                NoteName::F,
                NoteName::A,
                NoteName::C,
            )),
            "Dmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::D,
                NoteName::F,
                NoteName::A,
                NoteName::Cis,
            )),
            "Ddim7" => Ok(MultiNote::Quatrain(
                NoteName::D,
                NoteName::F,
                NoteName::As,
                NoteName::Ces,
            )),
            "Daug7" => Ok(MultiNote::Quatrain(
                NoteName::D,
                NoteName::Fis,
                NoteName::Ais,
                NoteName::Cis,
            )),
            // A
            "A7" => Ok(MultiNote::Quatrain(
                NoteName::A,
                NoteName::Cis,
                NoteName::E,
                NoteName::G,
            )),
            "Amaj7" => Ok(MultiNote::Quatrain(
                NoteName::A,
                NoteName::Cis,
                NoteName::E,
                NoteName::Gis,
            )),
            "Am7" => Ok(MultiNote::Quatrain(
                NoteName::A,
                NoteName::C,
                NoteName::E,
                NoteName::G,
            )),
            "Ammaj7" => Ok(MultiNote::Quatrain(
                NoteName::A,
                NoteName::C,
                NoteName::E,
                NoteName::Gis,
            )),
            "Adim7" => Ok(MultiNote::Quatrain(
                NoteName::A,
                NoteName::C,
                NoteName::Es,
                NoteName::Ges,
            )),
            "Aaug7" => Ok(MultiNote::Quatrain(
                NoteName::A,
                NoteName::Cis,
                NoteName::Eis,
                NoteName::Gis,
            )),
            // E
            "E7" => Ok(MultiNote::Quatrain(
                NoteName::E,
                NoteName::Gis,
                NoteName::H,
                NoteName::D,
            )),
            "Emaj7" => Ok(MultiNote::Quatrain(
                NoteName::E,
                NoteName::Gis,
                NoteName::H,
                NoteName::Dis,
            )),
            "Em7" => Ok(MultiNote::Quatrain(
                NoteName::E,
                NoteName::G,
                NoteName::H,
                NoteName::D,
            )),
            "Emmaj7" => Ok(MultiNote::Quatrain(
                NoteName::E,
                NoteName::G,
                NoteName::H,
                NoteName::Dis,
            )),
            "Edim7" => Ok(MultiNote::Quatrain(
                NoteName::E,
                NoteName::G,
                NoteName::B,
                NoteName::Des,
            )),
            "Eaug7" => Ok(MultiNote::Quatrain(
                NoteName::E,
                NoteName::Gis,
                NoteName::His,
                NoteName::Dis,
            )),
            // H
            "H7" => Ok(MultiNote::Quatrain(
                NoteName::H,
                NoteName::Dis,
                NoteName::Fis,
                NoteName::A,
            )),
            "Hmaj7" => Ok(MultiNote::Quatrain(
                NoteName::H,
                NoteName::Dis,
                NoteName::Fis,
                NoteName::Ais,
            )),
            "Hm7" => Ok(MultiNote::Quatrain(
                NoteName::H,
                NoteName::D,
                NoteName::Fis,
                NoteName::A,
            )),
            "Hmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::H,
                NoteName::D,
                NoteName::Fis,
                NoteName::Ais,
            )),
            "Hdim7" => Ok(MultiNote::Quatrain(
                NoteName::H,
                NoteName::D,
                NoteName::F,
                NoteName::As,
            )),
            "Haug7" => Ok(MultiNote::Quatrain(
                NoteName::H,
                NoteName::Dis,
                NoteName::Fisis,
                NoteName::Ais,
            )),
            // Fis
            "Fis7" => Ok(MultiNote::Quatrain(
                NoteName::Fis,
                NoteName::Ais,
                NoteName::Cis,
                NoteName::E,
            )),
            "Fismaj7" => Ok(MultiNote::Quatrain(
                NoteName::Fis,
                NoteName::Ais,
                NoteName::Cis,
                NoteName::Eis,
            )),
            "Fism7" => Ok(MultiNote::Quatrain(
                NoteName::Fis,
                NoteName::A,
                NoteName::Cis,
                NoteName::E,
            )),
            "Fismmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Fis,
                NoteName::A,
                NoteName::Cis,
                NoteName::Eis,
            )),
            "Fisdim7" => Ok(MultiNote::Quatrain(
                NoteName::Fis,
                NoteName::A,
                NoteName::C,
                NoteName::Es,
            )),
            "Fisaug7" => Ok(MultiNote::Quatrain(
                NoteName::Fis,
                NoteName::Ais,
                NoteName::Cisis,
                NoteName::Eis,
            )),

            //b-Tonarten

            // F
            "F7" => Ok(MultiNote::Quatrain(
                NoteName::F,
                NoteName::A,
                NoteName::C,
                NoteName::Es,
            )),
            "Fmaj7" => Ok(MultiNote::Quatrain(
                NoteName::F,
                NoteName::A,
                NoteName::C,
                NoteName::E,
            )),
            "Fm7" => Ok(MultiNote::Quatrain(
                NoteName::F,
                NoteName::As,
                NoteName::C,
                NoteName::Es,
            )),
            "Fmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::F,
                NoteName::As,
                NoteName::C,
                NoteName::E,
            )),
            "Fdim7" => Ok(MultiNote::Quatrain(
                NoteName::F,
                NoteName::As,
                NoteName::Ces,
                NoteName::Eses,
            )),
            "Faug7" => Ok(MultiNote::Quatrain(
                NoteName::F,
                NoteName::A,
                NoteName::Cis,
                NoteName::E,
            )),
            // B
            "B7" => Ok(MultiNote::Quatrain(
                NoteName::B,
                NoteName::D,
                NoteName::F,
                NoteName::As,
            )),
            "Bmaj7" => Ok(MultiNote::Quatrain(
                NoteName::B,
                NoteName::D,
                NoteName::F,
                NoteName::A,
            )),
            "Bm7" => Ok(MultiNote::Quatrain(
                NoteName::B,
                NoteName::Des,
                NoteName::F,
                NoteName::As,
            )),
            "Bmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::B,
                NoteName::Des,
                NoteName::F,
                NoteName::A,
            )),
            "Bdim7" => Ok(MultiNote::Quatrain(
                NoteName::B,
                NoteName::Des,
                NoteName::Fes,
                NoteName::Asas,
            )),
            "Baug7" => Ok(MultiNote::Quatrain(
                NoteName::B,
                NoteName::D,
                NoteName::Fis,
                NoteName::A,
            )),
            // Es
            "Es7" => Ok(MultiNote::Quatrain(
                NoteName::Es,
                NoteName::G,
                NoteName::B,
                NoteName::Des,
            )),
            "Esmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Es,
                NoteName::G,
                NoteName::B,
                NoteName::D,
            )),
            "Esm7" => Ok(MultiNote::Quatrain(
                NoteName::Es,
                NoteName::Ges,
                NoteName::B,
                NoteName::Des,
            )),
            "Esmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Es,
                NoteName::Ges,
                NoteName::B,
                NoteName::D,
            )),
            "Esdim7" => Ok(MultiNote::Quatrain(
                NoteName::Es,
                NoteName::Ges,
                NoteName::Heses,
                NoteName::Deses,
            )),
            "Esaug7" => Ok(MultiNote::Quatrain(
                NoteName::Es,
                NoteName::G,
                NoteName::H,
                NoteName::D,
            )),
            // As
            "As7" => Ok(MultiNote::Quatrain(
                NoteName::As,
                NoteName::C,
                NoteName::Es,
                NoteName::Ges,
            )),
            "Asmaj7" => Ok(MultiNote::Quatrain(
                NoteName::As,
                NoteName::C,
                NoteName::Es,
                NoteName::G,
            )),
            "Asm7" => Ok(MultiNote::Quatrain(
                NoteName::As,
                NoteName::Ces,
                NoteName::Es,
                NoteName::Ges,
            )),
            "Asmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::As,
                NoteName::Ces,
                NoteName::Es,
                NoteName::G,
            )),
            "Asdim7" => Ok(MultiNote::Quatrain(
                NoteName::As,
                NoteName::Ces,
                NoteName::Eses,
                NoteName::Geses,
            )),
            "Asaug7" => Ok(MultiNote::Quatrain(
                NoteName::As,
                NoteName::C,
                NoteName::E,
                NoteName::G,
            )),
            // Des
            "Des7" => Ok(MultiNote::Quatrain(
                NoteName::Des,
                NoteName::F,
                NoteName::As,
                NoteName::Ces,
            )),
            "Desmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Des,
                NoteName::F,
                NoteName::As,
                NoteName::C,
            )),
            "Desm7" => Ok(MultiNote::Quatrain(
                NoteName::Des,
                NoteName::Fes,
                NoteName::As,
                NoteName::Ces,
            )),
            "Desmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Des,
                NoteName::Fes,
                NoteName::As,
                NoteName::C,
            )),
            "Desdim7" => Ok(MultiNote::Quatrain(
                NoteName::Des,
                NoteName::Fes,
                NoteName::Asas,
                NoteName::Ceses,
            )),
            "Desaug7" => Ok(MultiNote::Quatrain(
                NoteName::Des,
                NoteName::F,
                NoteName::A,
                NoteName::C,
            )),
            // Ges
            "Ges7" => Ok(MultiNote::Quatrain(
                NoteName::Ges,
                NoteName::B,
                NoteName::Des,
                NoteName::Fes,
            )),
            "Gesmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Ges,
                NoteName::B,
                NoteName::Des,
                NoteName::F,
            )),
            "Gesm7" => Ok(MultiNote::Quatrain(
                NoteName::Ges,
                NoteName::Heses,
                NoteName::Des,
                NoteName::Fes,
            )),
            "Gesmmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Ges,
                NoteName::Heses,
                NoteName::Des,
                NoteName::F,
            )),
            "Gesdim7" => Ok(MultiNote::Quatrain(
                NoteName::Ges,
                NoteName::Heses,
                NoteName::Deses,
                NoteName::Feses,
            )),
            "Gesaug7" => Ok(MultiNote::Quatrain(
                NoteName::Ges,
                NoteName::B,
                NoteName::D,
                NoteName::F,
            )),

            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cis7" => Ok(MultiNote::Quatrain(
                NoteName::Cis,
                NoteName::Eis,
                NoteName::Gis,
                NoteName::H,
            )),
            "Cismaj7" => Ok(MultiNote::Quatrain(
                NoteName::Cis,
                NoteName::Eis,
                NoteName::Gis,
                NoteName::His,
            )),
            "Cism7" => Ok(MultiNote::Quatrain(
                NoteName::Cis,
                NoteName::E,
                NoteName::Gis,
                NoteName::H,
            )),
            "Cismmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Cis,
                NoteName::E,
                NoteName::Gis,
                NoteName::His,
            )),
            "Cisdim7" => Ok(MultiNote::Quatrain(
                NoteName::Cis,
                NoteName::E,
                NoteName::G,
                NoteName::B,
            )),
            "Cisaug7" => Ok(MultiNote::Quatrain(
                NoteName::Cis,
                NoteName::Eis,
                NoteName::Gisis,
                NoteName::His,
            )),
            // Dis
            "Dis7" => Ok(MultiNote::Quatrain(
                NoteName::Dis,
                NoteName::Fisis,
                NoteName::Ais,
                NoteName::Cis,
            )),
            "Dismaj7" => Ok(MultiNote::Quatrain(
                NoteName::Dis,
                NoteName::Fisis,
                NoteName::Ais,
                NoteName::Cisis,
            )),
            "Dism7" => Ok(MultiNote::Quatrain(
                NoteName::Dis,
                NoteName::Fis,
                NoteName::Ais,
                NoteName::Cis,
            )),
            "Dismmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Dis,
                NoteName::Fis,
                NoteName::Ais,
                NoteName::Cisis,
            )),
            "Disdim7" => Ok(MultiNote::Quatrain(
                NoteName::Dis,
                NoteName::Fis,
                NoteName::A,
                NoteName::C,
            )),
            "Disaug7" => Ok(MultiNote::Quatrain(
                NoteName::Dis,
                NoteName::Fisis,
                NoteName::Aisis,
                NoteName::Cisis,
            )),
            "Gis7" => Ok(MultiNote::Quatrain(
                NoteName::Gis,
                NoteName::His,
                NoteName::Dis,
                NoteName::Fis,
            )),
            "Gismaj7" => Ok(MultiNote::Quatrain(
                NoteName::Gis,
                NoteName::His,
                NoteName::Dis,
                NoteName::Fisis,
            )),
            "Gism7" => Ok(MultiNote::Quatrain(
                NoteName::Gis,
                NoteName::H,
                NoteName::Dis,
                NoteName::Fis,
            )),
            "Gismmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Gis,
                NoteName::H,
                NoteName::Dis,
                NoteName::Fisis,
            )),
            "Gisdim7" => Ok(MultiNote::Quatrain(
                NoteName::Gis,
                NoteName::H,
                NoteName::D,
                NoteName::F,
            )),
            "Gisaug7" => Ok(MultiNote::Quatrain(
                NoteName::Gis,
                NoteName::His,
                NoteName::Disis,
                NoteName::Fisis,
            )),
            // Ais
            "Ais7" => Ok(MultiNote::Quatrain(
                NoteName::Ais,
                NoteName::Cisis,
                NoteName::Eis,
                NoteName::Gis,
            )),
            "Aismaj7" => Ok(MultiNote::Quatrain(
                NoteName::Ais,
                NoteName::Cisis,
                NoteName::Eis,
                NoteName::Gisis,
            )),
            "Aism7" => Ok(MultiNote::Quatrain(
                NoteName::Ais,
                NoteName::Cis,
                NoteName::Eis,
                NoteName::Gis,
            )),
            "Aismmaj7" => Ok(MultiNote::Quatrain(
                NoteName::Ais,
                NoteName::Cis,
                NoteName::Eis,
                NoteName::Gisis,
            )),
            "Aisdim7" => Ok(MultiNote::Quatrain(
                NoteName::Ais,
                NoteName::Cis,
                NoteName::E,
                NoteName::G,
            )),
            "Aisaug7" => Ok(MultiNote::Quatrain(
                NoteName::Ais,
                NoteName::Cisis,
                NoteName::Eisis,
                NoteName::Gisis,
            )),

            // --------------------------------
            //              Other
            // --------------------------------
            _ => Err(ChorError::NotAValidMultinote(s.to_string())),
        }
    }
}

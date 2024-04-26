use super::{ChorError, MultiNote, NoteName};

/// A wrapper class to allow parsing strings of german notes.
#[derive(Debug, Clone, Copy)]
pub struct MultiNoteGerman {
    /// The actual multinote parsed into.
    inner: super::MultiNote,
}

impl MultiNoteGerman {
    /// Return the contained MultiNote.
    pub fn to_multinote(self) -> super::MultiNote {
        self.inner
    }
}

impl std::str::FromStr for MultiNoteGerman {
    type Err = ChorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = match s {
            // --------------------------------
            //              Triads
            // --------------------------------

            // Kreuztonarten
            // C
            "C" => MultiNote::Triad(NoteName::C, NoteName::E, NoteName::G),
            "Cm" => MultiNote::Triad(NoteName::C, NoteName::Ef, NoteName::G),
            "Cdim" => MultiNote::Triad(NoteName::C, NoteName::Ef, NoteName::Gf),
            "Caug" => MultiNote::Triad(NoteName::C, NoteName::E, NoteName::Gs),
            // G
            "G" => MultiNote::Triad(NoteName::G, NoteName::B, NoteName::D),
            "Gm" => MultiNote::Triad(NoteName::G, NoteName::Bf, NoteName::D),
            "Gdim" => MultiNote::Triad(NoteName::G, NoteName::Bf, NoteName::Df),
            "Gaug" => MultiNote::Triad(NoteName::G, NoteName::B, NoteName::Ds),
            // D
            "D" => MultiNote::Triad(NoteName::D, NoteName::Fs, NoteName::A),
            "Dm" => MultiNote::Triad(NoteName::D, NoteName::F, NoteName::A),
            "Ddim" => MultiNote::Triad(NoteName::D, NoteName::F, NoteName::Af),
            "Daug" => MultiNote::Triad(NoteName::D, NoteName::Fs, NoteName::As),
            // A
            "A" => MultiNote::Triad(NoteName::A, NoteName::Cs, NoteName::E),
            "Am" => MultiNote::Triad(NoteName::A, NoteName::C, NoteName::E),
            "Adim" => MultiNote::Triad(NoteName::A, NoteName::C, NoteName::Ef),
            "Aaug" => MultiNote::Triad(NoteName::A, NoteName::Cs, NoteName::Es),
            // E
            "E" => MultiNote::Triad(NoteName::E, NoteName::Gs, NoteName::B),
            "Em" => MultiNote::Triad(NoteName::E, NoteName::G, NoteName::B),
            "Edim" => MultiNote::Triad(NoteName::E, NoteName::G, NoteName::Bf),
            "Eaug" => MultiNote::Triad(NoteName::E, NoteName::Gs, NoteName::Bs),
            // H
            "H" => MultiNote::Triad(NoteName::B, NoteName::Ds, NoteName::Fs),
            "Hm" => MultiNote::Triad(NoteName::B, NoteName::D, NoteName::Fs),
            "Hdim" => MultiNote::Triad(NoteName::B, NoteName::D, NoteName::F),
            "Haug" => MultiNote::Triad(NoteName::B, NoteName::Ds, NoteName::Fss),
            // Fis
            "Fis" => MultiNote::Triad(NoteName::Fs, NoteName::As, NoteName::Cs),
            "Fism" => MultiNote::Triad(NoteName::Fs, NoteName::A, NoteName::Cs),
            "Fisdim" => MultiNote::Triad(NoteName::Fs, NoteName::A, NoteName::C),
            "Fisaug" => MultiNote::Triad(NoteName::Fs, NoteName::As, NoteName::Css),

            // b-Tonarten
            // F
            "F" => MultiNote::Triad(NoteName::F, NoteName::A, NoteName::C),
            "Fm" => MultiNote::Triad(NoteName::F, NoteName::Af, NoteName::C),
            "Fdim" => MultiNote::Triad(NoteName::F, NoteName::Af, NoteName::Cf),
            "Faug" => MultiNote::Triad(NoteName::F, NoteName::A, NoteName::Cs),
            // B
            "B" => MultiNote::Triad(NoteName::Bf, NoteName::D, NoteName::F),
            "Bm" => MultiNote::Triad(NoteName::Bf, NoteName::Df, NoteName::F),
            "Bdim" => MultiNote::Triad(NoteName::Bf, NoteName::Df, NoteName::Ff),
            "Baug" => MultiNote::Triad(NoteName::Bf, NoteName::D, NoteName::Fs),
            // Es
            "Es" => MultiNote::Triad(NoteName::Ef, NoteName::G, NoteName::Bf),
            "Esm" => MultiNote::Triad(NoteName::Ef, NoteName::Gf, NoteName::Bf),
            "Esdim" => MultiNote::Triad(NoteName::Ef, NoteName::Gf, NoteName::Bff),
            "Esaug" => MultiNote::Triad(NoteName::Ef, NoteName::G, NoteName::B),
            // As
            "As" => MultiNote::Triad(NoteName::Af, NoteName::C, NoteName::Ef),
            "Asm" => MultiNote::Triad(NoteName::Af, NoteName::Cf, NoteName::Ef),
            "Asdim" => MultiNote::Triad(NoteName::Af, NoteName::Cf, NoteName::Eff),
            "Asaug" => MultiNote::Triad(NoteName::Af, NoteName::C, NoteName::E),
            // Des
            "Des" => MultiNote::Triad(NoteName::Df, NoteName::F, NoteName::Af),
            "Desm" => MultiNote::Triad(NoteName::Df, NoteName::Ff, NoteName::Af),
            "Desdim" => MultiNote::Triad(NoteName::Df, NoteName::Ff, NoteName::Aff),
            "Desaug" => MultiNote::Triad(NoteName::Df, NoteName::F, NoteName::A),
            // Ges
            "Ges" => MultiNote::Triad(NoteName::Gf, NoteName::Bf, NoteName::Df),
            "Gesm" => MultiNote::Triad(NoteName::Gf, NoteName::Bff, NoteName::Df),
            "Gesdim" => MultiNote::Triad(NoteName::Gf, NoteName::Bff, NoteName::Dff),
            "Gesaug" => MultiNote::Triad(NoteName::Gf, NoteName::Bf, NoteName::D),
            //

            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cis" => MultiNote::Triad(NoteName::Cs, NoteName::Es, NoteName::Gs),
            "Cism" => MultiNote::Triad(NoteName::Cs, NoteName::E, NoteName::Gs),
            "Cisdim" => MultiNote::Triad(NoteName::Cs, NoteName::E, NoteName::G),
            "Cisaug" => MultiNote::Triad(NoteName::Cs, NoteName::Es, NoteName::Gss),
            // Dis
            "Dis" => MultiNote::Triad(NoteName::Ds, NoteName::Fss, NoteName::As),
            "Dism" => MultiNote::Triad(NoteName::Ds, NoteName::Fs, NoteName::As),
            "Disdim" => MultiNote::Triad(NoteName::Ds, NoteName::Fs, NoteName::A),
            "Disaug" => MultiNote::Triad(NoteName::Ds, NoteName::Fss, NoteName::Ass),
            "Gis" => MultiNote::Triad(NoteName::Gs, NoteName::Bs, NoteName::Ds),
            "Gism" => MultiNote::Triad(NoteName::Gs, NoteName::B, NoteName::Ds),
            "Gisdim" => MultiNote::Triad(NoteName::Gs, NoteName::B, NoteName::D),
            "Gisaug" => MultiNote::Triad(NoteName::Gs, NoteName::Bs, NoteName::Dss),
            // Ais
            "Ais" => MultiNote::Triad(NoteName::As, NoteName::Css, NoteName::Es),
            "Aism" => MultiNote::Triad(NoteName::As, NoteName::Cs, NoteName::Es),
            "Aisdim" => MultiNote::Triad(NoteName::As, NoteName::Cs, NoteName::E),
            "Aisaug" => MultiNote::Triad(NoteName::As, NoteName::Css, NoteName::Ess),

            // sus4 and sus2

            // Kreuztonarten
            // C
            "Csus2" => MultiNote::Triad(NoteName::C, NoteName::D, NoteName::G),
            "Csus4" => MultiNote::Triad(NoteName::C, NoteName::F, NoteName::G),
            // G
            "Gsus2" => MultiNote::Triad(NoteName::G, NoteName::A, NoteName::D),
            "Gsus4" => MultiNote::Triad(NoteName::G, NoteName::C, NoteName::D),
            // D
            "Dsus2" => MultiNote::Triad(NoteName::D, NoteName::E, NoteName::A),
            "Dsus4" => MultiNote::Triad(NoteName::D, NoteName::G, NoteName::A),
            // A
            "Asus2" => MultiNote::Triad(NoteName::A, NoteName::B, NoteName::E),
            "Asus4" => MultiNote::Triad(NoteName::A, NoteName::D, NoteName::E),
            // E
            "Esus2" => MultiNote::Triad(NoteName::E, NoteName::Fs, NoteName::B),
            "Esus4" => MultiNote::Triad(NoteName::E, NoteName::A, NoteName::B),
            // H
            "Hsus2" => MultiNote::Triad(NoteName::B, NoteName::Cs, NoteName::Fs),
            "Hsus4" => MultiNote::Triad(NoteName::B, NoteName::E, NoteName::Fs),
            // Fis
            "Fissus2" => MultiNote::Triad(NoteName::Fs, NoteName::Gs, NoteName::Cs),
            "Fissus4" => MultiNote::Triad(NoteName::Fs, NoteName::B, NoteName::Cs),

            // b-Tonarten
            // F
            "Fsus2" => MultiNote::Triad(NoteName::F, NoteName::G, NoteName::C),
            "Fsus4" => MultiNote::Triad(NoteName::F, NoteName::Bf, NoteName::C),
            // B
            "Bsus2" => MultiNote::Triad(NoteName::Bf, NoteName::C, NoteName::F),
            "Bsus4" => MultiNote::Triad(NoteName::Bf, NoteName::Ef, NoteName::F),
            // Es
            "Essus2" => MultiNote::Triad(NoteName::Ef, NoteName::F, NoteName::Bf),
            "Essus4" => MultiNote::Triad(NoteName::Ef, NoteName::Af, NoteName::Bf),
            // As
            "Assus2" => MultiNote::Triad(NoteName::Af, NoteName::Bf, NoteName::Ef),
            "Assus4" => MultiNote::Triad(NoteName::Af, NoteName::Df, NoteName::Ef),
            // Des
            "Dessus2" => MultiNote::Triad(NoteName::Df, NoteName::Ef, NoteName::Af),
            "Dessus4" => MultiNote::Triad(NoteName::Df, NoteName::Gf, NoteName::Af),
            // Ges
            "Gessus2" => MultiNote::Triad(NoteName::Gf, NoteName::Af, NoteName::Df),
            "Gessus4" => MultiNote::Triad(NoteName::Gf, NoteName::B, NoteName::Df),
            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cissus2" => MultiNote::Triad(NoteName::Cs, NoteName::Ds, NoteName::Gs),
            "Cissus4" => MultiNote::Triad(NoteName::Cs, NoteName::Fs, NoteName::Gs),
            // Dis
            "Dissus2" => MultiNote::Triad(NoteName::Ds, NoteName::Es, NoteName::As),
            "Dissus4" => MultiNote::Triad(NoteName::Ds, NoteName::Gs, NoteName::As),
            //Gis
            "Gissus2" => MultiNote::Triad(NoteName::Gs, NoteName::As, NoteName::Ds),
            "Gissus4" => MultiNote::Triad(NoteName::Gs, NoteName::Cs, NoteName::Ds),
            // Ais
            "Aissus2" => MultiNote::Triad(NoteName::As, NoteName::Bs, NoteName::Es),
            "Aissus4" => MultiNote::Triad(NoteName::As, NoteName::Ds, NoteName::Es),

            // --------------------------------
            //            Quadrains
            // --------------------------------
            // Kreuztonarten
            //C
            "Cmaj7" => MultiNote::Quatrain(NoteName::C, NoteName::E, NoteName::G, NoteName::B),
            "C7" => MultiNote::Quatrain(NoteName::C, NoteName::E, NoteName::G, NoteName::Bf),
            "Cm7" => MultiNote::Quatrain(NoteName::C, NoteName::Ef, NoteName::G, NoteName::Bf),
            "Cmmaj7" => MultiNote::Quatrain(NoteName::C, NoteName::Ef, NoteName::G, NoteName::B),
            "Cdim7" => MultiNote::Quatrain(NoteName::C, NoteName::Ef, NoteName::Gf, NoteName::Bff),
            "Caug7" => MultiNote::Quatrain(NoteName::C, NoteName::E, NoteName::Gs, NoteName::B),
            "Cm7b5" => MultiNote::Quatrain(NoteName::C, NoteName::Ef, NoteName::Gf, NoteName::Bf),
            "C6" => MultiNote::Quatrain(NoteName::C, NoteName::E, NoteName::G, NoteName::A),
            "Cm6" => MultiNote::Quatrain(NoteName::C, NoteName::Ef, NoteName::G, NoteName::A),

            // G
            "G7" => MultiNote::Quatrain(NoteName::G, NoteName::B, NoteName::D, NoteName::F),
            "Gmaj7" => MultiNote::Quatrain(NoteName::G, NoteName::B, NoteName::D, NoteName::Fs),
            "Gm7" => MultiNote::Quatrain(NoteName::G, NoteName::Bf, NoteName::D, NoteName::F),
            "Gmmaj7" => MultiNote::Quatrain(NoteName::G, NoteName::Bf, NoteName::D, NoteName::Fs),
            "Gdim7" => MultiNote::Quatrain(NoteName::G, NoteName::Bf, NoteName::Df, NoteName::Ff),
            "Gaug7" => MultiNote::Quatrain(NoteName::G, NoteName::B, NoteName::Ds, NoteName::Fs),
            "Gm7b5" => MultiNote::Quatrain(NoteName::G, NoteName::Bf, NoteName::Df, NoteName::F),
            "G6" => MultiNote::Quatrain(NoteName::G, NoteName::B, NoteName::D, NoteName::E),
            "Gm6" => MultiNote::Quatrain(NoteName::G, NoteName::Bf, NoteName::D, NoteName::E),
            // D
            "D7" => MultiNote::Quatrain(NoteName::D, NoteName::Fs, NoteName::A, NoteName::C),
            "Dmaj7" => MultiNote::Quatrain(NoteName::D, NoteName::Fs, NoteName::A, NoteName::Cs),
            "Dm7" => MultiNote::Quatrain(NoteName::D, NoteName::F, NoteName::A, NoteName::C),
            "Dmmaj7" => MultiNote::Quatrain(NoteName::D, NoteName::F, NoteName::A, NoteName::Cs),
            "Ddim7" => MultiNote::Quatrain(NoteName::D, NoteName::F, NoteName::Af, NoteName::Cf),
            "Daug7" => MultiNote::Quatrain(NoteName::D, NoteName::Fs, NoteName::As, NoteName::Cs),
            "Dm7b5" => MultiNote::Quatrain(NoteName::D, NoteName::F, NoteName::Af, NoteName::C),
            "D6" => MultiNote::Quatrain(NoteName::D, NoteName::Fs, NoteName::A, NoteName::B),
            "Dm6" => MultiNote::Quatrain(NoteName::D, NoteName::F, NoteName::A, NoteName::B),
            // A
            "A7" => MultiNote::Quatrain(NoteName::A, NoteName::Cs, NoteName::E, NoteName::G),
            "Amaj7" => MultiNote::Quatrain(NoteName::A, NoteName::Cs, NoteName::E, NoteName::Gs),
            "Am7" => MultiNote::Quatrain(NoteName::A, NoteName::C, NoteName::E, NoteName::G),
            "Ammaj7" => MultiNote::Quatrain(NoteName::A, NoteName::C, NoteName::E, NoteName::Gs),
            "Adim7" => MultiNote::Quatrain(NoteName::A, NoteName::C, NoteName::Ef, NoteName::Gf),
            "Aaug7" => MultiNote::Quatrain(NoteName::A, NoteName::Cs, NoteName::Es, NoteName::Gs),
            "Am7b5" => MultiNote::Quatrain(NoteName::A, NoteName::C, NoteName::Ef, NoteName::G),
            "A6" => MultiNote::Quatrain(NoteName::A, NoteName::Cs, NoteName::E, NoteName::Fs),
            "Am6" => MultiNote::Quatrain(NoteName::A, NoteName::C, NoteName::E, NoteName::Fs),
            // E
            "E7" => MultiNote::Quatrain(NoteName::E, NoteName::Gs, NoteName::B, NoteName::D),
            "Emaj7" => MultiNote::Quatrain(NoteName::E, NoteName::Gs, NoteName::B, NoteName::Ds),
            "Em7" => MultiNote::Quatrain(NoteName::E, NoteName::G, NoteName::B, NoteName::D),
            "Emmaj7" => MultiNote::Quatrain(NoteName::E, NoteName::G, NoteName::B, NoteName::Ds),
            "Edim7" => MultiNote::Quatrain(NoteName::E, NoteName::G, NoteName::Bf, NoteName::Df),
            "Eaug7" => MultiNote::Quatrain(NoteName::E, NoteName::Gs, NoteName::Bs, NoteName::Ds),
            "Em7b5" => MultiNote::Quatrain(NoteName::E, NoteName::G, NoteName::Bf, NoteName::D),
            "E6" => MultiNote::Quatrain(NoteName::E, NoteName::Gs, NoteName::B, NoteName::Cs),
            "Em6" => MultiNote::Quatrain(NoteName::E, NoteName::G, NoteName::B, NoteName::Cs),
            // H
            "H7" => MultiNote::Quatrain(NoteName::B, NoteName::Ds, NoteName::Fs, NoteName::A),
            "Hmaj7" => MultiNote::Quatrain(NoteName::B, NoteName::Ds, NoteName::Fs, NoteName::As),
            "Hm7" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::Fs, NoteName::A),
            "Hmmaj7" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::Fs, NoteName::As),
            "Hdim7" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::F, NoteName::Af),
            "Haug7" => MultiNote::Quatrain(NoteName::B, NoteName::Ds, NoteName::Fss, NoteName::As),
            "Hm7b5" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::F, NoteName::A),
            "H6" => MultiNote::Quatrain(NoteName::B, NoteName::Ds, NoteName::Fs, NoteName::Gs),
            "Hm6" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::Fs, NoteName::Gs),
            // Fis
            "Fis7" => MultiNote::Quatrain(NoteName::Fs, NoteName::As, NoteName::Cs, NoteName::E),
            "Fismaj7" => {
                MultiNote::Quatrain(NoteName::Fs, NoteName::As, NoteName::Cs, NoteName::Es)
            }
            "Fism7" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::Cs, NoteName::E),
            "Fismmaj7" => {
                MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::Cs, NoteName::Es)
            }
            "Fisdim7" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::C, NoteName::Ef),
            "Fisaug7" => {
                MultiNote::Quatrain(NoteName::Fs, NoteName::As, NoteName::Css, NoteName::Es)
            }
            "Fism7b5" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::C, NoteName::E),
            "Fis6" => MultiNote::Quatrain(NoteName::Fs, NoteName::As, NoteName::Cs, NoteName::Ds),
            "Fism6" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::Cs, NoteName::Ds),

            //b-Tonarten

            // F
            "F7" => MultiNote::Quatrain(NoteName::F, NoteName::A, NoteName::C, NoteName::Ef),
            "Fmaj7" => MultiNote::Quatrain(NoteName::F, NoteName::A, NoteName::C, NoteName::E),
            "Fm7" => MultiNote::Quatrain(NoteName::F, NoteName::Af, NoteName::C, NoteName::Ef),
            "Fmmaj7" => MultiNote::Quatrain(NoteName::F, NoteName::Af, NoteName::C, NoteName::E),
            "Fdim7" => MultiNote::Quatrain(NoteName::F, NoteName::Af, NoteName::Cf, NoteName::Eff),
            "Faug7" => MultiNote::Quatrain(NoteName::F, NoteName::A, NoteName::Cs, NoteName::E),
            "Fm7b5" => MultiNote::Quatrain(NoteName::F, NoteName::Af, NoteName::Cf, NoteName::Ef),
            "F6" => MultiNote::Quatrain(NoteName::F, NoteName::A, NoteName::C, NoteName::D),
            "Fm6" => MultiNote::Quatrain(NoteName::F, NoteName::Af, NoteName::C, NoteName::D),
            // B
            "B7" => MultiNote::Quatrain(NoteName::Bf, NoteName::D, NoteName::F, NoteName::Af),
            "Bmaj7" => MultiNote::Quatrain(NoteName::Bf, NoteName::D, NoteName::F, NoteName::A),
            "Bm7" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::F, NoteName::Af),
            "Bmmaj7" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::F, NoteName::A),
            "Bdim7" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::Ff, NoteName::Aff),
            "Baug7" => MultiNote::Quatrain(NoteName::Bf, NoteName::D, NoteName::Fs, NoteName::A),
            "Bm7b5" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::Ff, NoteName::Af),
            "B6" => MultiNote::Quatrain(NoteName::Bf, NoteName::D, NoteName::F, NoteName::G),
            "Bm6" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::F, NoteName::G),
            // Es
            "Es7" => MultiNote::Quatrain(NoteName::Ef, NoteName::G, NoteName::Bf, NoteName::Df),
            "Esmaj7" => MultiNote::Quatrain(NoteName::Ef, NoteName::G, NoteName::Bf, NoteName::D),
            "Esm7" => MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bf, NoteName::Df),
            "Esmmaj7" => MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bf, NoteName::D),
            "Esdim7" => {
                MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bff, NoteName::Dff)
            }
            "Esaug7" => MultiNote::Quatrain(NoteName::Ef, NoteName::G, NoteName::B, NoteName::D),
            "Esm7b5" => {
                MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bff, NoteName::Df)
            }
            "Es6" => MultiNote::Quatrain(NoteName::Ef, NoteName::G, NoteName::Bf, NoteName::C),
            "Esm6" => MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bf, NoteName::C),
            // As
            "As7" => MultiNote::Quatrain(NoteName::Af, NoteName::C, NoteName::Ef, NoteName::Gf),
            "Asmaj7" => MultiNote::Quatrain(NoteName::Af, NoteName::C, NoteName::Ef, NoteName::G),
            "Asm7" => MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Ef, NoteName::Gf),
            "Asmmaj7" => MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Ef, NoteName::G),
            "Asdim7" => {
                MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Eff, NoteName::Gff)
            }
            "Asaug7" => MultiNote::Quatrain(NoteName::Af, NoteName::C, NoteName::E, NoteName::G),
            "Asm7b5" => {
                MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Eff, NoteName::Gf)
            }
            "As6" => MultiNote::Quatrain(NoteName::Af, NoteName::C, NoteName::Ef, NoteName::F),
            "Asm6" => MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Ef, NoteName::F),
            // Des
            "Des7" => MultiNote::Quatrain(NoteName::Df, NoteName::F, NoteName::Af, NoteName::Cf),
            "Desmaj7" => MultiNote::Quatrain(NoteName::Df, NoteName::F, NoteName::Af, NoteName::C),
            "Desm7" => MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Af, NoteName::Cf),
            "Desmmaj7" => {
                MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Af, NoteName::C)
            }
            "Desdim7" => {
                MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Aff, NoteName::Css)
            }
            "Desaug7" => MultiNote::Quatrain(NoteName::Df, NoteName::F, NoteName::A, NoteName::C),
            "Desm7b5" => {
                MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Aff, NoteName::Cf)
            }
            "Des6" => MultiNote::Quatrain(NoteName::Df, NoteName::F, NoteName::Af, NoteName::Bf),
            "Desm6" => MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Af, NoteName::Bf),
            // Ges
            "Ges7" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bf, NoteName::Df, NoteName::Ff),
            "Gesmaj7" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bf, NoteName::Df, NoteName::F),
            "Gesm7" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Df, NoteName::Ff),
            "Gesmmaj7" => {
                MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Df, NoteName::F)
            }
            "Gesdim7" => {
                MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Dff, NoteName::Fff)
            }
            "Gesaug7" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bf, NoteName::D, NoteName::F),
            "Gesm7b5" => {
                MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Dff, NoteName::Ff)
            }
            "Ges6" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bf, NoteName::Df, NoteName::Ef),
            "Gesm6" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Df, NoteName::Ef),
            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cis7" => MultiNote::Quatrain(NoteName::Cs, NoteName::Es, NoteName::Gs, NoteName::B),
            "Cismaj7" => {
                MultiNote::Quatrain(NoteName::Cs, NoteName::Es, NoteName::Gs, NoteName::Bs)
            }
            "Cism7" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::Gs, NoteName::B),
            "Cismmaj7" => {
                MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::Gs, NoteName::Bs)
            }
            "Cisdim7" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::G, NoteName::Bf),
            "Cisaug7" => {
                MultiNote::Quatrain(NoteName::Cs, NoteName::Es, NoteName::Gss, NoteName::Bs)
            }
            "Cism7b5" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::G, NoteName::B),
            "Cis6" => MultiNote::Quatrain(NoteName::Cs, NoteName::Es, NoteName::Gs, NoteName::As),
            "Cism6" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::Gs, NoteName::As),
            // Dis
            "Dis7" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fss, NoteName::As, NoteName::Cs),
            "Dismaj7" => {
                MultiNote::Quatrain(NoteName::Ds, NoteName::Fss, NoteName::As, NoteName::Css)
            }
            "Dism7" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::As, NoteName::Cs),
            "Dismmaj7" => {
                MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::As, NoteName::Css)
            }
            "Disdim7" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::A, NoteName::C),
            "Disaug7" => {
                MultiNote::Quatrain(NoteName::Ds, NoteName::Fss, NoteName::Ass, NoteName::Css)
            }
            "Dism7b5" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::A, NoteName::Cs),
            "Dis6" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fss, NoteName::As, NoteName::Bs),
            "Dism6" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::As, NoteName::Bs),
            //Gis
            "Gis7" => MultiNote::Quatrain(NoteName::Gs, NoteName::Bs, NoteName::Ds, NoteName::Fs),
            "Gismaj7" => {
                MultiNote::Quatrain(NoteName::Gs, NoteName::Bs, NoteName::Ds, NoteName::Fss)
            }
            "Gism7" => MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::Ds, NoteName::Fs),
            "Gismmaj7" => {
                MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::Ds, NoteName::Fss)
            }
            "Gisdim7" => MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::D, NoteName::F),
            "Gisaug7" => {
                MultiNote::Quatrain(NoteName::Gs, NoteName::Bs, NoteName::Dss, NoteName::Fss)
            }
            "Gism7b5" => MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::D, NoteName::Fs),
            "Gis6" => MultiNote::Quatrain(NoteName::Gs, NoteName::Bs, NoteName::Ds, NoteName::Es),
            "Gism6" => MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::Ds, NoteName::Es),
            // Ais
            "Ais7" => MultiNote::Quatrain(NoteName::As, NoteName::Css, NoteName::Es, NoteName::Gs),
            "Aismaj7" => {
                MultiNote::Quatrain(NoteName::As, NoteName::Css, NoteName::Es, NoteName::Gss)
            }
            "Aism7" => MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::Es, NoteName::Gs),
            "Aismmaj7" => {
                MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::Es, NoteName::Gss)
            }
            "Aisdim7" => MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::E, NoteName::G),
            "Aisaug7" => {
                MultiNote::Quatrain(NoteName::As, NoteName::Css, NoteName::Ess, NoteName::Gss)
            }
            "Aism7b5" => MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::E, NoteName::Gs),
            "Ais6" => MultiNote::Quatrain(NoteName::As, NoteName::Css, NoteName::Es, NoteName::Fss),
            "Aism6" => MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::Es, NoteName::Fss),

            // --------------------------------
            //              Other
            // --------------------------------
            _ => {
                return Err(ChorError::NotAValidMultinote(s.to_string()));
            }
        };
        Ok(Self { inner })
    }
}

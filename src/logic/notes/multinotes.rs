use super::{ChorError, NoteName};

#[derive(Debug, Clone, Copy)]
pub enum MultiNote {
    Triad(NoteName, NoteName, NoteName),
    Quatrain(NoteName, NoteName, NoteName, NoteName),
}

impl std::str::FromStr for MultiNote {
    type Err = ChorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
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
            "B" => MultiNote::Triad(NoteName::B, NoteName::Ds, NoteName::Fs),
            "Bm" => MultiNote::Triad(NoteName::B, NoteName::D, NoteName::Fs),
            "Bdim" => MultiNote::Triad(NoteName::B, NoteName::D, NoteName::F),
            "Baug" => MultiNote::Triad(NoteName::B, NoteName::Ds, NoteName::Fss),
            // Fis
            "Fs" => MultiNote::Triad(NoteName::Fs, NoteName::As, NoteName::Cs),
            "Fsm" => MultiNote::Triad(NoteName::Fs, NoteName::A, NoteName::Cs),
            "Fsdim" => MultiNote::Triad(NoteName::Fs, NoteName::A, NoteName::C),
            "Fsaug" => MultiNote::Triad(NoteName::Fs, NoteName::As, NoteName::Css),

            // b-Tonarten
            // F
            "F" => MultiNote::Triad(NoteName::F, NoteName::A, NoteName::C),
            "Fm" => MultiNote::Triad(NoteName::F, NoteName::Af, NoteName::C),
            "Fdim" => MultiNote::Triad(NoteName::F, NoteName::Af, NoteName::Cf),
            "Faug" => MultiNote::Triad(NoteName::F, NoteName::A, NoteName::Cs),
            // B
            "Bf" => MultiNote::Triad(NoteName::Bf, NoteName::D, NoteName::F),
            "Bfm" => MultiNote::Triad(NoteName::Bf, NoteName::Df, NoteName::F),
            "Bfdim" => MultiNote::Triad(NoteName::Bf, NoteName::Df, NoteName::Ff),
            "Bfaug" => MultiNote::Triad(NoteName::Bf, NoteName::D, NoteName::Fs),
            // Es
            "Ef" => MultiNote::Triad(NoteName::Ef, NoteName::G, NoteName::Bf),
            "Efm" => MultiNote::Triad(NoteName::Ef, NoteName::Gf, NoteName::Bf),
            "Efdim" => MultiNote::Triad(NoteName::Ef, NoteName::Gf, NoteName::Bff),
            "Efaug" => MultiNote::Triad(NoteName::Ef, NoteName::G, NoteName::B),
            // As
            "Af" => MultiNote::Triad(NoteName::Af, NoteName::C, NoteName::Ef),
            "Afm" => MultiNote::Triad(NoteName::Af, NoteName::Cf, NoteName::Ef),
            "Afdim" => MultiNote::Triad(NoteName::Af, NoteName::Cf, NoteName::Eff),
            "Afaug" => MultiNote::Triad(NoteName::Af, NoteName::C, NoteName::E),
            // Des
            "Df" => MultiNote::Triad(NoteName::Df, NoteName::F, NoteName::Af),
            "Dfm" => MultiNote::Triad(NoteName::Df, NoteName::Ff, NoteName::Af),
            "Dfdim" => MultiNote::Triad(NoteName::Df, NoteName::Ff, NoteName::Aff),
            "Dfaug" => MultiNote::Triad(NoteName::Df, NoteName::F, NoteName::A),
            // Ges
            "Gf" => MultiNote::Triad(NoteName::Gf, NoteName::Bf, NoteName::Df),
            "Gfm" => MultiNote::Triad(NoteName::Gf, NoteName::Bff, NoteName::Df),
            "Gfdim" => MultiNote::Triad(NoteName::Gf, NoteName::Bff, NoteName::Dff),
            "Gfaug" => MultiNote::Triad(NoteName::Gf, NoteName::Bf, NoteName::D),
            //

            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cs" => MultiNote::Triad(NoteName::Cs, NoteName::Es, NoteName::Gs),
            "Csm" => MultiNote::Triad(NoteName::Cs, NoteName::E, NoteName::Gs),
            "Csdim" => MultiNote::Triad(NoteName::Cs, NoteName::E, NoteName::G),
            "Csaug" => MultiNote::Triad(NoteName::Cs, NoteName::Es, NoteName::Gss),
            // Dis
            "Ds" => MultiNote::Triad(NoteName::Ds, NoteName::Fss, NoteName::As),
            "Dsm" => MultiNote::Triad(NoteName::Ds, NoteName::Fs, NoteName::As),
            "Dsdim" => MultiNote::Triad(NoteName::Ds, NoteName::Fs, NoteName::A),
            "Dsaug" => MultiNote::Triad(NoteName::Ds, NoteName::Fss, NoteName::Ass),
            "Gs" => MultiNote::Triad(NoteName::Gs, NoteName::Bs, NoteName::Ds),
            "Gsm" => MultiNote::Triad(NoteName::Gs, NoteName::B, NoteName::Ds),
            "Gsdim" => MultiNote::Triad(NoteName::Gs, NoteName::B, NoteName::D),
            "Gsaug" => MultiNote::Triad(NoteName::Gs, NoteName::Bs, NoteName::Dss),
            // Ais
            "As" => MultiNote::Triad(NoteName::As, NoteName::Css, NoteName::Es),
            "Asm" => MultiNote::Triad(NoteName::As, NoteName::Cs, NoteName::Es),
            "Asdim" => MultiNote::Triad(NoteName::As, NoteName::Cs, NoteName::E),
            "Asaug" => MultiNote::Triad(NoteName::As, NoteName::Css, NoteName::Ess),

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
            "Fssus2" => MultiNote::Triad(NoteName::Fs, NoteName::Gs, NoteName::Cs),
            "Fssus4" => MultiNote::Triad(NoteName::Fs, NoteName::B, NoteName::Cs),

            // b-Tonarten
            // F
            "Fsus2" => MultiNote::Triad(NoteName::F, NoteName::G, NoteName::C),
            "Fsus4" => MultiNote::Triad(NoteName::F, NoteName::Bf, NoteName::C),
            // B
            "Bfsus2" => MultiNote::Triad(NoteName::Bf, NoteName::C, NoteName::F),
            "Bfsus4" => MultiNote::Triad(NoteName::Bf, NoteName::Ef, NoteName::F),
            // Es
            "Efsus2" => MultiNote::Triad(NoteName::Ef, NoteName::F, NoteName::Bf),
            "Efsus4" => MultiNote::Triad(NoteName::Ef, NoteName::Af, NoteName::Bf),
            // As
            "Afsus2" => MultiNote::Triad(NoteName::Af, NoteName::Bf, NoteName::Ef),
            "Afsus4" => MultiNote::Triad(NoteName::Af, NoteName::Df, NoteName::Ef),
            // Des
            "Dfsus2" => MultiNote::Triad(NoteName::Df, NoteName::Ef, NoteName::Af),
            "Dfsus4" => MultiNote::Triad(NoteName::Df, NoteName::Gf, NoteName::Af),
            // Ges
            "Gfsus2" => MultiNote::Triad(NoteName::Gf, NoteName::Af, NoteName::Df),
            "Gfsus4" => MultiNote::Triad(NoteName::Gf, NoteName::B, NoteName::Df),
            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cssus2" => MultiNote::Triad(NoteName::Cs, NoteName::Ds, NoteName::Gs),
            "Cssus4" => MultiNote::Triad(NoteName::Cs, NoteName::Fs, NoteName::Gs),
            // Dis
            "Dssus2" => MultiNote::Triad(NoteName::Ds, NoteName::Es, NoteName::As),
            "Dssus4" => MultiNote::Triad(NoteName::Ds, NoteName::Gs, NoteName::As),
            //Gis
            "Gssus2" => MultiNote::Triad(NoteName::Gs, NoteName::As, NoteName::Ds),
            "Gssus4" => MultiNote::Triad(NoteName::Gs, NoteName::Cs, NoteName::Ds),
            // Ais
            "Assus2" => MultiNote::Triad(NoteName::As, NoteName::Bs, NoteName::Es),
            "Assus4" => MultiNote::Triad(NoteName::As, NoteName::Ds, NoteName::Es),

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
            "B7" => MultiNote::Quatrain(NoteName::B, NoteName::Ds, NoteName::Fs, NoteName::A),
            "Bmaj7" => MultiNote::Quatrain(NoteName::B, NoteName::Ds, NoteName::Fs, NoteName::As),
            "Bm7" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::Fs, NoteName::A),
            "Bmmaj7" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::Fs, NoteName::As),
            "Bdim7" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::F, NoteName::Af),
            "Baug7" => MultiNote::Quatrain(NoteName::B, NoteName::Ds, NoteName::Fss, NoteName::As),
            "Bm7b5" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::F, NoteName::A),
            "B6" => MultiNote::Quatrain(NoteName::B, NoteName::Ds, NoteName::Fs, NoteName::Gs),
            "Bm6" => MultiNote::Quatrain(NoteName::B, NoteName::D, NoteName::Fs, NoteName::Gs),
            // Fis
            "Fs7" => MultiNote::Quatrain(NoteName::Fs, NoteName::As, NoteName::Cs, NoteName::E),
            "Fsmaj7" => MultiNote::Quatrain(NoteName::Fs, NoteName::As, NoteName::Cs, NoteName::Es),
            "Fsm7" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::Cs, NoteName::E),
            "Fsmmaj7" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::Cs, NoteName::Es),
            "Fsdim7" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::C, NoteName::Ef),
            "Fsaug7" => {
                MultiNote::Quatrain(NoteName::Fs, NoteName::As, NoteName::Css, NoteName::Es)
            }
            "Fsm7b5" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::C, NoteName::E),
            "Fs6" => MultiNote::Quatrain(NoteName::Fs, NoteName::As, NoteName::Cs, NoteName::Ds),
            "Fsm6" => MultiNote::Quatrain(NoteName::Fs, NoteName::A, NoteName::Cs, NoteName::Ds),

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
            "Bf7" => MultiNote::Quatrain(NoteName::Bf, NoteName::D, NoteName::F, NoteName::Af),
            "Bfmaj7" => MultiNote::Quatrain(NoteName::Bf, NoteName::D, NoteName::F, NoteName::A),
            "Bfm7" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::F, NoteName::Af),
            "Bfmmaj7" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::F, NoteName::A),
            "Bfdim7" => {
                MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::Ff, NoteName::Aff)
            }
            "Bfaug7" => MultiNote::Quatrain(NoteName::Bf, NoteName::D, NoteName::Fs, NoteName::A),
            "Bfm7b5" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::Ff, NoteName::Af),
            "Bf6" => MultiNote::Quatrain(NoteName::Bf, NoteName::D, NoteName::F, NoteName::G),
            "Bfm6" => MultiNote::Quatrain(NoteName::Bf, NoteName::Df, NoteName::F, NoteName::G),
            // Es
            "Ef7" => MultiNote::Quatrain(NoteName::Ef, NoteName::G, NoteName::Bf, NoteName::Df),
            "Efmaj7" => MultiNote::Quatrain(NoteName::Ef, NoteName::G, NoteName::Bf, NoteName::D),
            "Efm7" => MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bf, NoteName::Df),
            "Efmmaj7" => MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bf, NoteName::D),
            "Efdim7" => {
                MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bff, NoteName::Dff)
            }
            "Efaug7" => MultiNote::Quatrain(NoteName::Ef, NoteName::G, NoteName::B, NoteName::D),
            "Efm7b5" => {
                MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bff, NoteName::Df)
            }
            "Ef6" => MultiNote::Quatrain(NoteName::Ef, NoteName::G, NoteName::Bf, NoteName::C),
            "Efm6" => MultiNote::Quatrain(NoteName::Ef, NoteName::Gf, NoteName::Bf, NoteName::C),
            // As
            "Af7" => MultiNote::Quatrain(NoteName::Af, NoteName::C, NoteName::Ef, NoteName::Gf),
            "Afmaj7" => MultiNote::Quatrain(NoteName::Af, NoteName::C, NoteName::Ef, NoteName::G),
            "Afm7" => MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Ef, NoteName::Gf),
            "Afmmaj7" => MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Ef, NoteName::G),
            "Afdim7" => {
                MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Eff, NoteName::Gff)
            }
            "Afaug7" => MultiNote::Quatrain(NoteName::Af, NoteName::C, NoteName::E, NoteName::G),
            "Afm7b5" => {
                MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Eff, NoteName::Gf)
            }
            "Af6" => MultiNote::Quatrain(NoteName::Af, NoteName::C, NoteName::Ef, NoteName::F),
            "Afm6" => MultiNote::Quatrain(NoteName::Af, NoteName::Cf, NoteName::Ef, NoteName::F),
            // Des
            "Df7" => MultiNote::Quatrain(NoteName::Df, NoteName::F, NoteName::Af, NoteName::Cf),
            "Dfmaj7" => MultiNote::Quatrain(NoteName::Df, NoteName::F, NoteName::Af, NoteName::C),
            "Dfm7" => MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Af, NoteName::Cf),
            "Dfmmaj7" => MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Af, NoteName::C),
            "Dfdim7" => {
                MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Aff, NoteName::Css)
            }
            "Dfaug7" => MultiNote::Quatrain(NoteName::Df, NoteName::F, NoteName::A, NoteName::C),
            "Dfm7b5" => {
                MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Aff, NoteName::Cf)
            }
            "Df6" => MultiNote::Quatrain(NoteName::Df, NoteName::F, NoteName::Af, NoteName::Bf),
            "Dfm6" => MultiNote::Quatrain(NoteName::Df, NoteName::Ff, NoteName::Af, NoteName::Bf),
            // Ges
            "Gf7" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bf, NoteName::Df, NoteName::Ff),
            "Gfmaj7" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bf, NoteName::Df, NoteName::F),
            "Gfm7" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Df, NoteName::Ff),
            "Gfmmaj7" => {
                MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Df, NoteName::F)
            }
            "Gfdim7" => {
                MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Dff, NoteName::Fff)
            }
            "Gfaug7" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bf, NoteName::D, NoteName::F),
            "Gfm7b5" => {
                MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Dff, NoteName::Ff)
            }
            "Gf6" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bf, NoteName::Df, NoteName::Ef),
            "Gfm6" => MultiNote::Quatrain(NoteName::Gf, NoteName::Bff, NoteName::Df, NoteName::Ef),
            // enharmonisch gleiche sinnvolle Tonarten
            // Cis
            "Cs7" => MultiNote::Quatrain(NoteName::Cs, NoteName::Es, NoteName::Gs, NoteName::B),
            "Csmaj7" => MultiNote::Quatrain(NoteName::Cs, NoteName::Es, NoteName::Gs, NoteName::Bs),
            "Csm7" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::Gs, NoteName::B),
            "Csmmaj7" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::Gs, NoteName::Bs),
            "Csdim7" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::G, NoteName::Bf),
            "Csaug7" => {
                MultiNote::Quatrain(NoteName::Cs, NoteName::Es, NoteName::Gss, NoteName::Bs)
            }
            "Csm7b5" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::G, NoteName::B),
            "Cs6" => MultiNote::Quatrain(NoteName::Cs, NoteName::Es, NoteName::Gs, NoteName::As),
            "Csm6" => MultiNote::Quatrain(NoteName::Cs, NoteName::E, NoteName::Gs, NoteName::As),
            // Dis
            "Ds7" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fss, NoteName::As, NoteName::Cs),
            "Dsmaj7" => {
                MultiNote::Quatrain(NoteName::Ds, NoteName::Fss, NoteName::As, NoteName::Css)
            }
            "Dsm7" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::As, NoteName::Cs),
            "Dsmmaj7" => {
                MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::As, NoteName::Css)
            }
            "Dsdim7" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::A, NoteName::C),
            "Dsaug7" => {
                MultiNote::Quatrain(NoteName::Ds, NoteName::Fss, NoteName::Ass, NoteName::Css)
            }
            "Dsm7b5" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::A, NoteName::Cs),
            "Ds6" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fss, NoteName::As, NoteName::Bs),
            "Dsm6" => MultiNote::Quatrain(NoteName::Ds, NoteName::Fs, NoteName::As, NoteName::Bs),
            //Gis
            "Gs7" => MultiNote::Quatrain(NoteName::Gs, NoteName::Bs, NoteName::Ds, NoteName::Fs),
            "Gsmaj7" => {
                MultiNote::Quatrain(NoteName::Gs, NoteName::Bs, NoteName::Ds, NoteName::Fss)
            }
            "Gsm7" => MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::Ds, NoteName::Fs),
            "Gsmmaj7" => {
                MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::Ds, NoteName::Fss)
            }
            "Gsdim7" => MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::D, NoteName::F),
            "Gsaug7" => {
                MultiNote::Quatrain(NoteName::Gs, NoteName::Bs, NoteName::Dss, NoteName::Fss)
            }
            "Gsm7b5" => MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::D, NoteName::Fs),
            "Gs6" => MultiNote::Quatrain(NoteName::Gs, NoteName::Bs, NoteName::Ds, NoteName::Es),
            "Gsm6" => MultiNote::Quatrain(NoteName::Gs, NoteName::B, NoteName::Ds, NoteName::Es),
            // Ais
            "As7" => MultiNote::Quatrain(NoteName::As, NoteName::Css, NoteName::Es, NoteName::Gs),
            "Asmaj7" => {
                MultiNote::Quatrain(NoteName::As, NoteName::Css, NoteName::Es, NoteName::Gss)
            }
            "Asm7" => MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::Es, NoteName::Gs),
            "Asmmaj7" => {
                MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::Es, NoteName::Gss)
            }
            "Asdim7" => MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::E, NoteName::G),
            "Asaug7" => {
                MultiNote::Quatrain(NoteName::As, NoteName::Css, NoteName::Ess, NoteName::Gss)
            }
            "Asm7b5" => MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::E, NoteName::Gs),
            "As6" => MultiNote::Quatrain(NoteName::As, NoteName::Css, NoteName::Es, NoteName::Fss),
            "Asm6" => MultiNote::Quatrain(NoteName::As, NoteName::Cs, NoteName::Es, NoteName::Fss),

            // --------------------------------
            //              Other
            // --------------------------------
            _ => {
                return Err(ChorError::NotAValidMultinote(s.to_string()));
            }
        };
        Ok(res)
    }
}

use super::languages;
use leptos::*;

#[component]
pub fn Info() -> impl IntoView {
    // Get the used language
    let language = use_context::<ReadSignal<languages::Language>>()
        .unwrap_or_else(|| create_signal(languages::Language::English).0);

    match language() {
        languages::Language::German => {
            view! {
                <p>"Es stehen die folgenden Akkorde zur Eingabe zur Verfügung:"</p>
                <table>
                    <tr>
                        <th>"Typ"</th>
                        <th>"Schreibweise"</th>
                        <th>"Beispiele"</th>
                    </tr>
                    <tr>
                        <td class="first">"Dur-Dreiklang"</td>
                        <td>"Startton"</td>
                        <td>"D, Dis"</td>
                    </tr>
                    <tr>
                        <td class="first">"Moll-Dreiklang"</td>
                        <td>"Startton+ 'm'"</td>
                        <td>"Gm, Dism"</td>
                    </tr>
                    <tr>
                        <td class="first">"Verminderter Dreiklang"</td>
                        <td>"Startton + 'dim'"</td>
                        <td>"Ddim, Disdim"</td>
                    </tr>
                    <tr>
                        <td class="first">"Übermäßiger Dreiklang"</td>
                        <td>"Startton + 'aug'"</td>
                        <td>"Eaug, Disaug"</td>
                    </tr>
                    <tr>
                        <td class="first">"Sus2-Dreiklang"</td>
                        <td>"Startton + 'sus2'"</td>
                        <td>"Asus2, Dissus2"</td>
                    </tr>
                    <tr>
                        <td class="first">"Sus4-Dreiklang"</td>
                        <td>"Startton + 'sus4'"</td>
                        <td>"Asus4, Dissus4"</td>
                    </tr>
                    <tr>
                        <td class="first">"Dominantseptakkord"</td>
                        <td>"Startton + '7'"</td>
                        <td>"B7, Fis7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Major-Septakkord"</td>
                        <td>"Startton + 'maj7'"</td>
                        <td>"Hmaj7, Asmaj7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Mollseptakkord"</td>
                        <td>"Startton + 'm7'"</td>
                        <td>"Gm7, Desm7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Mollseptakkord mit großer Septime"</td>
                        <td>"Startton + 'mmaj7'"</td>
                        <td>"Ammaj7, Aismmaj7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Halbverminderter Septakkord"</td>
                        <td>"Startton + '7b5'"</td>
                        <td>"D7b5, Es7b5"</td>
                    </tr>
                    <tr>
                        <td class="first">"Verminderter Septakkord"</td>
                        <td>"Startton + 'dim7'"</td>
                        <td>"Ddim7, Aisdim7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Übermäßiger Septakkord"</td>
                        <td>"Startton + 'aug7'"</td>
                        <td>"Caug7, Esaug7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Durdreiklang mit Sixte ajoutée"</td>
                        <td>"Startton + '6'"</td>
                        <td>"A6, Cis6"</td>
                    </tr>
                    <tr>
                        <td class="first">"Molldreiklang mit Sixte ajoutée"</td>
                        <td>"Startton + 'm7'"</td>
                        <td>"Em7, Gesm7"</td>
                    </tr>
                </table>
                <p>"Als Starttöne stehen hierbei " <b class="header">"C, G, D, A, E, H, Fis, Cis, Gis, Dis, Ais, F, B, Es, As, Des, Ges"</b> " zur Verfügung."</p>
                <p>
                    "Weiterhin ist es möglich, den Basston eines Akkordes durch Anhängen von '/Basston' manuell festzusetzen, z.B. 'C/E'."
                </p>
                <p>
                    "Verschiedene Akkorde sind durch Leerzeichen, Kommas oder Semikolons zu trennen, hierbei werden ungültige Eingaben ignoriert."
                </p>
            }
        }
        languages::Language::English => {
            view! {
                <p>
                    "The input syntax can also be viewed in the "
                    <b class="header"><a class="header" href="https://github.com/Dualraum/Chorsatz/wiki">"wiki"</a></b>
                    "."
                </p>
                <p>"The following notations for chord input are supported."</p>
                <table>
                    <tr>
                        <th>"Type"</th>
                        <th>"Syntax"</th>
                        <th>"Examples"</th>
                    </tr>
                    <tr>
                        <td class="first">"Major triad"</td>
                        <td>"Initial note"</td>
                        <td>"D, Dis"</td>
                    </tr>
                    <tr>
                        <td class="first">"Minor triad"</td>
                        <td>"Initial note+ 'm'"</td>
                        <td>"Gf, Dsm"</td>
                    </tr>
                    <tr>
                        <td class="first">"Diminished Triad"</td>
                        <td>"Initial note+ 'dim'"</td>
                        <td>"Ddim, Dsdim"</td>
                    </tr>
                    <tr>
                        <td class="first">"Augmented Triad"</td>
                        <td>"Initial note+ 'aug'"</td>
                        <td>"Eaug, Dsaug"</td>
                    </tr>
                    <tr>
                        <td class="first">"Sus2 Triad"</td>
                        <td>"Initial note+ 'sus2'"</td>
                        <td>"Asus2, Dssus2"</td>
                    </tr>
                    <tr>
                        <td class="first">"Sus4 Triad"</td>
                        <td>"Initial note+ 'sus4'"</td>
                        <td>"Asus4, Dssus4"</td>
                    </tr>
                    <tr>
                        <td class="first">"Dominant seventh chord"</td>
                        <td>"Initial note+ '7'"</td>
                        <td>"Bf7, Fs7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Major seventh chord"</td>
                        <td>"Initial note+ 'maj7'"</td>
                        <td>"Bmaj7, Afmaj7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Minor seventh chord"</td>
                        <td>"Initial note+ 'm7'"</td>
                        <td>"Gm7, Dfm7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Minor seventh chord with major seventh"</td>
                        <td>"Initial note+ 'mmaj7'"</td>
                        <td>"Ammaj7, Asmmaj7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Semi-diminished seventh chord"</td>
                        <td>"Initial note+ '7b5'"</td>
                        <td>"D7b5, Ef7b5"</td>
                    </tr>
                    <tr>
                        <td class="first">"Diminished seventh chord"</td>
                        <td>"Initial note+ 'dim7'"</td>
                        <td>"Ddim7, Asdim7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Augmented seventh chord"</td>
                        <td>"Initial note+ 'aug7'"</td>
                        <td>"Caug7, Efaug7"</td>
                    </tr>
                    <tr>
                        <td class="first">"Major triad with Sixte ajoutée"</td>
                        <td>"Initial note+ '6'"</td>
                        <td>"A6, Cs6"</td>
                    </tr>
                    <tr>
                        <td class="first">"Minor triad with Sixte ajoutée"</td>
                        <td>"Initial note+ 'm7'"</td>
                        <td>"Em7, Gfm7"</td>
                    </tr>
                </table>
                <p>"Valid initial notes are " <b class="header">"C, G, D, A, E, H, Fs, Cs, Gs, Ds, As, F, Hf, Ef, Af, Df, Gf"</b> "."</p>
                <p>
                    "Furthermore, the bass note of a chord can be manually set by a slash '/' and the desired bass note postfixed to the chord, e.g. 'Ddim/E'."
                </p>
                <p>
                    "Invalid chords are simply ignored. Chords have to be separated by spaces, commas or semicolons."
                </p>
            }
        }
    }
}

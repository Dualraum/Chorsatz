use leptos::*;

#[component]
pub fn Info() -> impl IntoView {
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
                <td>"Startton + 'm'"</td>
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
            "Verschiedene Akkorde sind durch Leerzeichen zu trennen, hierbei werden ungültige Eingaben ignoriert."
        </p>

    }
}

use leptos::*;

#[component]
pub fn Options() -> impl IntoView {
    view! {
        <table>
            <tr>
                <th colspan="4">"Selektionskriterien"</th>
            </tr>
            <tr>
                // Soprano-Alt tone difference
                <td> <input class="number" type="number" min="0" step="0.5" prop:value="6"/> </td>
                <td> <p>"Maximale Differenz Sopran/Alt"</p> </td>
                // Alt-Tenor tone Difference
                <td> <input class="number" type="number" min="0" step="0.5" prop:value="6"/> </td>
                <td> <p>"Maximale Differenz Alt/Tenor"</p> </td>
            </tr>
            <tr>
                // Tenor-Bass tone difference
                <td> <input class="number" type="number" min="0" step="0.5" prop:value="6"/> </td>
                <td> <p>"Maximale Differenz Tenor/Bass"</p> </td>
                // Soprano-Bass tone Difference
                <td> <input class="number" type="number" min="0" step="0.5" prop:value="6"/> </td>
                <td> <p>"Maximale Differenz Sopran/Bass"</p> </td>
            </tr>
            <tr>
                // Allow Crossings
                <td> <input type="checkbox" prop:value=false/> </td>
                <td> <p>"Tonkreuzungen erlauben"</p> </td>
                // Letting lie
                <td> <input type="checkbox" prop:value=true/> </td>
                <td> <p>"Liegenlassen erzwingen"</p> </td>
            </tr>
            <tr>
                <td> <input type="checkbox" prop:value=true/> </td>
                <td> <p>"Bass-Gegenbewegung Erzwingen"</p> </td>
            </tr>
            <tr>
                <td colspan=2> <input type="text" prop:value="5 8"/> </td>
                <td colspan=2> <p>"Verbotene Parallelen"</p> </td>
            </tr>
            <tr>
                // Soprano jump
                <td> <input class="number" type="number" min="0" step="0.5" prop:value="6"/> </td>
                <td> <p>"Maximaler Sopransprung"</p> </td>
                // Alto jump
                <td> <input class="number" type="number" min="0" step="0.5" prop:value="6"/> </td>
                <td> <p>"Maximaler Altsprung"</p> </td>
            </tr>
            <tr>
                // Tenor Jump
                <td> <input class="number" type="number" min="0" step="0.5" prop:value="6"/> </td>
                <td> <p>"Maximaler Tenorsprung"</p> </td>
                // Bass Jump
                <td> <input class="number" type="number" min="0" step="0.5" prop:value="6"/> </td>
                <td> <p>"Maximaler Basssprung"</p> </td>
            </tr>
            <tr>
                <th colspan="4">"Ordnungskriterien"</th>
            </tr>
            <tr>
                // Sum of Absolutes weight
                <td> <input class="number" type="number" min="0" step="0.1" prop:value="0.8"/> </td>
                <td> <p>"Relatives Gewicht Betragssumme"</p> </td>
                // Absolute of Sums weight
                <td> <input class="number" type="number" min="0" step="0.1" prop:value="1.0"/> </td>
                <td> <p>"Relatives Gewicht Summenbetrag"</p> </td>
            </tr>
            <tr>
                // Soprano-Alt-Weigth
                <td> <input class="number" type="number" min="0" step="0.1" prop:value="0.4"/> </td>
                <td> <p>"Relatives Gewicht geschlossene Lage"</p> </td>
            </tr>
            <tr>
                <td> <input class="number" type="number" min="0" step="0.1" prop:value="1.2"/> </td>
                <td> <p>"Malus für hohen Startsopran"</p> </td>

                <td> <input class="number" type="text" prop:value="Cis2"/> </td>
                <td> <p>"Grenzton für hohen Startsopran"</p> </td>
            </tr>
            <tr>
                <td> <input class="number" type="number" min="0" step="0.1" prop:value="1.2"/> </td>
                <td> <p>"Malus für tiefen Startbass"</p> </td>

                <td> <input class="number" type="text" prop:value="Ais1"/> </td>
                <td> <p>"Grenzton für tiefen Startbass"</p> </td>
            </tr>

        </table>
    }
}

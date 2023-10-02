use leptos::*;

use crate::logic::Config;

#[component]
pub fn Options(config: ReadSignal<Config>, set_config: WriteSignal<Config>) -> impl IntoView {
    view! {
        <table>
            <tr>
                <th colspan="4">"Selektionskriterien"</th>
            </tr>
            <tr>
                // Soprano-Alt tone difference
                <td> <input class="number" type="number" min="0" step="0.5" prop:value={move || config().max_diff_sopran_alt}/> </td>
                <td>"Maximale Differenz Sopran/Alt"</td>
                // Alt-Tenor tone Difference
                <td> <input class="number" type="number" min="0" step="0.5" prop:value={move || config().max_diff_alt_tenor}/> </td>
                <td>"Maximale Differenz Alt/Tenor"</td>
            </tr>
            <tr>
                // Tenor-Bass tone difference
                <td> <input class="number" type="number" min="0" step="0.5" prop:value={move || config().max_diff_tenor_bass}/> </td>
                <td>"Maximale Differenz Tenor/Bass"</td>
                // Soprano-Bass tone Difference
                <td> <input class="number" type="number" min="0" step="0.5" prop:value={move || config().max_diff_sopran_bass}/> </td>
                <td>"Maximale Differenz Sopran/Bass"</td>
            </tr>
            <tr>
                // Allow Crossings
                <td> <input type="checkbox" prop:checked={move || config().allow_crossings}/> </td>
                <td>"Tonkreuzungen erlauben"</td>
                // Letting lie
                <td> <input type="checkbox" prop:checked={move || config().force_letting_lie}/> </td>
                <td>"Liegenlassen erzwingen"</td>
            </tr>
            <tr>
                <td> <input type="checkbox" prop:checked={move || config().force_base_countermovement}/> </td>
                <td>"Bass-Gegenbewegung Erzwingen"</td>
            </tr>
            <tr>
                <td colspan=2> <input type="text" prop:value="5 8"/> </td>
                <td colspan=2> <p>"Verbotene Parallelen"</p> </td>
            </tr>
            <tr>
                // Soprano jump
                <td> <input class="number" type="number" min="0" step="0.5" prop:value={move || config().max_jump_sopran}/> </td>
                <td>"Maximaler Sopransprung"</td>
                // Alto jump
                <td> <input class="number" type="number" min="0" step="0.5" prop:value={move || config().max_jump_alt}/> </td>
                <td>"Maximaler Altsprung"</td>
            </tr>
            <tr>
                // Tenor Jump
                <td> <input class="number" type="number" min="0" step="0.5" prop:value={move || config().max_jump_tenor}/> </td>
                <td>"Maximaler Tenorsprung"</td>
                // Bass Jump
                <td> <input class="number" type="number" min="0" step="0.5" prop:value={move || config().max_jump_bass}/> </td>
                <td>"Maximaler Basssprung"</td>
            </tr>
            <tr>
                <th colspan="4">"Ordnungskriterien"</th>
            </tr>
            <tr>
                // Sum of Absolutes weight
                <td> <input class="number" type="number" min="0" step="0.1" prop:value={move || config().sub_of_abs_weight}/> </td>
                <td>"Relatives Gewicht Betragssumme"</td>
                // Absolute of Sums weight
                <td> <input class="number" type="number" min="0" step="0.1" prop:value={move || config().abs_of_sum_weight}/> </td>
                <td>"Relatives Gewicht Summenbetrag"</td>
            </tr>
            <tr>
                // Soprano-Alt-Weigth
                <td> <input class="number" type="number" min="0" step="0.1" prop:value={move || config().soprano_alt_diff_weight}/> </td>
                <td>"Relatives Gewicht geschlossene Lage"</td>
            </tr>
            <tr>
                <td> <input class="number" type="number" min="0" step="0.1" prop:value={move || config().exposure_penalty_sopran}/> </td>
                <td>"Malus für hohen Startsopran"</td>

                <td> <input class="number" type="text" prop:value={move || config().exposure_threshold_sopran.to_string()}/> </td>
                <td>"Grenzton für hohen Startsopran"</td>
            </tr>
            <tr>
                <td> <input class="number" type="number" min="0" step="0.1" prop:value={move || config().exposure_penalty_bass}/> </td>
                <td>"Malus für tiefen Startbass"</td>

                <td> <input class="number" type="text" prop:value={move || config().exposure_threshold_bass.to_string()}/> </td>
                <td>"Grenzton für tiefen Startbass"</td>
            </tr>

        </table>
    }
}

use itertools::Itertools;
use leptos::*;

use crate::logic::Config;

#[component]
pub fn Options(config: ReadSignal<Config>, set_config: WriteSignal<Config>) -> impl IntoView {
    view! {
        <table>
            <tr>
                <th colspan="4">"Auswahlkriterien"</th>
            </tr>
            <tr>
                // Soprano-Alt tone difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_sopran_alt}
                    on:change=move |ev|{
                        set_config.update(|config| config.max_diff_sopran_alt = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                    }
                /> </td>
                <td>"Maximale Differenz Sopran/Alt"</td>
                // Alt-Tenor tone Difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_alt_tenor}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_alt_tenor = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Differenz Alt/Tenor"</td>
            </tr>
            <tr>
                // Tenor-Bass tone difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_tenor_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_tenor_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Differenz Tenor/Bass"</td>
                // Soprano-Bass tone Difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_sopran_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_sopran_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Differenz Sopran/Bass"</td>
            </tr>
            <tr>
                // Allow Crossings
                <td> <input type="checkbox" prop:checked={move || config().allow_crossings}
                on:change=move |ev|{
                    set_config.update(|config| config.allow_crossings = event_target_checked(&ev));
                }
                /> </td>
                <td>"Tonkreuzungen erlauben"</td>
                // Allow Equality in Bass/Tenor
                <Show
                    when=move ||{!config().allow_crossings}
                    fallback=|| view!{
                        <td></td> <td></td>
                    }
                >
                    <td>
                        <input type="checkbox" prop:checked={move || config().allow_bass_tenor_equal}
                            on:change=move |ev|{
                                set_config.update(|config| config.allow_bass_tenor_equal = event_target_checked(&ev));
                            }
                        />
                    </td>
                    <td>"Bass/Tenor-Gleichheit erlauben"</td>
                </Show>
            </tr>
            <tr>
                // Letting lie
                <td> <input type="checkbox" prop:checked={move || config().force_letting_lie}
                on:change=move |ev|{
                    set_config.update(|config| config.force_letting_lie = event_target_checked(&ev));
                }
                /> </td>
                <td>"Liegenlassen erzwingen"</td>
                // Force counter movement
                <td> <input type="checkbox" prop:checked={move || config().force_base_countermovement}
                on:change=move |ev|{
                    set_config.update(|config| config.force_base_countermovement = event_target_checked(&ev));
                }
                /> </td>
                <td>"Bass-Gegenbewegung Erzwingen"</td>
            </tr>
            <tr>
                <td colspan=2> <input type="text" prop:value="5 8"
                    on:change=move |ev|{
                        set_config.update(|config| config.forbidden_parallels = event_target_value(&ev).split(' ').flat_map(str::parse::<f32>).collect_vec());
                    }
                /> </td>
                <td colspan=2> <p>"Verbotene Parallelen"</p> </td>
            </tr>
            <tr>
                // Soprano jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_sopran}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_sopran = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximaler Sopransprung"</td>
                // Alto jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_alt}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_alt = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximaler Altsprung"</td>
            </tr>
            <tr>
                // Tenor Jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_tenor}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_tenor = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximaler Tenorsprung"</td>
                // Bass Jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximaler Basssprung"</td>
            </tr>
            <tr>
                <th colspan="4">"Ordnungskriterien"</th>
            </tr>
            <tr>
                // Sum of Absolutes weight
                <td> <input type="number" step="0.1" prop:value={move || format!("{:.1}", config().sub_of_abs_weight)}
                on:change=move |ev|{
                    set_config.update(|config| config.sub_of_abs_weight = event_target_value(&ev).parse::<f32>().unwrap_or(0.));
                }
                /> </td>
                <td>"Relatives Gewicht Betragssumme"</td>
                // Absolute of Sums weight
                <td> <input type="number" step="0.1" prop:value={move || format!("{:.1}", config().abs_of_sum_weight)}
                on:change=move |ev|{
                    set_config.update(|config| config.abs_of_sum_weight = event_target_value(&ev).parse::<f32>().unwrap_or(0.));
                }
                /> </td>
                <td>"Relatives Gewicht Summenbetrag"</td>
            </tr>
            <tr>
                // Soprano-Alt-Weigth
                <td> <input type="number" step="0.1" prop:value={move || format!("{:.1}", config().soprano_alt_diff_weight)}
                on:change=move |ev|{
                    set_config.update(|config| config.soprano_alt_diff_weight = event_target_value(&ev).parse::<f32>().unwrap_or(0.));
                }
                /> </td>
                <td>"Relatives Gewicht geschlossene Lage"</td>
            </tr>
            <tr>
                <td> <input type="number" min="0" step="0.1" prop:value={move || format!("{:.1}", config().exposure_penalty_sopran)}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_penalty_sopran = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Malus für hohen Startsopran"</td>

                <td> <input type="text" prop:value={move || config().exposure_threshold_sopran.to_string()}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_threshold_sopran = event_target_value(&ev).parse::<crate::logic::notes::OctavedNote>().unwrap_or_default());
                }
                /> </td>
                <td>"Grenzton für hohen Startsopran"</td>
            </tr>
            <tr>
                <td> <input type="number" min="0" step="0.1" prop:value={move || format!("{:.1}", config().exposure_penalty_bass)}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_penalty_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Malus für hohen Startbass"</td>

                <td> <input type="text" prop:value={move || config().exposure_threshold_bass.to_string()}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_threshold_bass = event_target_value(&ev).parse::<crate::logic::notes::OctavedNote>().unwrap_or_default());
                }
                /> </td>
                <td>"Grenzton für hohen Startbass"</td>
            </tr>

        </table>
    }
}

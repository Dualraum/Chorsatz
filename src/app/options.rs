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
                <td>"Maximale Stimmdifferenz Sopran-Alt"</td>
                // Alt-Tenor tone Difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_alt_tenor}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_alt_tenor = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Stimmdifferenz Alt-Tenor"</td>
            </tr>
            <tr>
                // Tenor-Bass tone difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_tenor_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_tenor_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Stimmdifferenz Tenor-Bass"</td>
                // Soprano-Bass tone Difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_sopran_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_sopran_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Stimmdifferenz Sopran-Bass"</td>
            </tr>
            <tr>
                // Allow Crossings
                <td> <input type="checkbox" prop:checked={move || config().allow_crossings}
                on:change=move |ev|{
                    set_config.update(|config| config.allow_crossings = event_target_checked(&ev));
                }
                /> </td>
                <td>"Stimmkreuzungen erlauben"</td>
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
                    <td>"Gleiche Töne in Bass & Tenor erlauben"</td>
                </Show>
            </tr>
            <tr>
                // Letting lie
                <td> <input type="checkbox" prop:checked={move || config().force_letting_lie}
                on:change=move |ev|{
                    set_config.update(|config| config.force_letting_lie = event_target_checked(&ev));
                }
                /> </td>
                <td>"Aufeinanderfolgende gleiche Töne liegen lassen"</td>
                // Force counter movement
                <td> <input type="checkbox" prop:checked={move || config().force_base_countermovement}
                on:change=move |ev|{
                    set_config.update(|config| config.force_base_countermovement = event_target_checked(&ev));
                }
                /> </td>
                <td>"Gegenbewegung zum Bass"</td>
            </tr>
            <tr>
                <td colspan=2> <input type="text" prop:value="5,8"
                    on:change=move |ev|{
                        set_config.update(|config| config.forbidden_parallels = event_target_value(&ev).replace(' ', "").split(',').flat_map(str::parse::<f32>).collect_vec());
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
                <td>"Maximale Sprungweite im Sopran"</td>
                // Alto jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_alt}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_alt = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Sprungweite im Alt"</td>
            </tr>
            <tr>
                // Tenor Jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_tenor}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_tenor = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Sprungweite im Tenor"</td>
                // Bass Jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Maximale Sprungweite im Bass"</td>
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
                <td>"Relatives Gewicht geringe Bewegungen"</td>
                // Absolute of Sums weight
                <td> <input type="number" step="0.1" prop:value={move || format!("{:.1}", config().abs_of_sum_weight)}
                on:change=move |ev|{
                    set_config.update(|config| config.abs_of_sum_weight = event_target_value(&ev).parse::<f32>().unwrap_or(0.));
                }
                /> </td>
                <td>"Relatives Gewicht Gegenbewegungen"</td>
            </tr>
            <tr>
                // Soprano-Alt-Weigth
                <td> <input type="number" step="0.1" prop:value={move || format!("{:.1}", config().soprano_alt_diff_weight)}
                on:change=move |ev|{
                    set_config.update(|config| config.soprano_alt_diff_weight = event_target_value(&ev).parse::<f32>().unwrap_or(0.));
                }
                /> </td>
                <td>"Relatives Gewicht enge Lage"</td>
            </tr>
            <tr>
                <td> <input type="number" min="0" step="0.1" prop:value={move || format!("{:.1}", config().exposure_penalty_sopran)}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_penalty_sopran = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Malus für hohen Startton im Sopran"</td>

                <td> <input type="text" prop:value={move || config().exposure_threshold_sopran.to_string()}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_threshold_sopran = event_target_value(&ev).parse::<crate::logic::notes::OctavedNote>().unwrap_or_default());
                }
                /> </td>
                <td>"Grenzton für hohen Startton im Sopran"</td>
            </tr>
            <tr>
                <td> <input type="number" min="0" step="0.1" prop:value={move || format!("{:.1}", config().exposure_penalty_bass)}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_penalty_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>"Malus für hohen Startton im Bass"</td>

                <td> <input type="text" prop:value={move || config().exposure_threshold_bass.to_string()}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_threshold_bass = event_target_value(&ev).parse::<crate::logic::notes::OctavedNote>().unwrap_or_default());
                }
                /> </td>
                <td>"Grenzton für hohen Starttonull im Bass"</td>
            </tr>

        </table>
    }
}

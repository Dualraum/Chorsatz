use itertools::Itertools;
use leptos::*;

use crate::logic::Config;

use crate::app::languages;

#[component]
pub fn Options(config: ReadSignal<Config>, set_config: WriteSignal<Config>) -> impl IntoView {
    // Get used language
    let language = use_context::<ReadSignal<languages::Language>>()
        .unwrap_or_else(|| create_signal(languages::Language::English).0);

    view! {
        <table>
            <tr>
                <th colspan="4">{move || languages::get_string_set(language()).options_content[0]}</th>
            </tr>
            <tr>
                // Soprano-Alt tone difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_sopran_alt}
                    on:change=move |ev|{
                        set_config.update(|config| config.max_diff_sopran_alt = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                    }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[1]}</td>
                // Alt-Tenor tone Difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_alt_tenor}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_alt_tenor = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[2]}</td>
            </tr>
            <tr>
                // Tenor-Bass tone difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_tenor_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_tenor_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[3]}</td>
                // Soprano-Bass tone Difference
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_diff_sopran_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_diff_sopran_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[4]}</td>
            </tr>
            <tr>
                // Allow Crossings
                <td> <input type="checkbox" prop:checked={move || config().allow_crossings}
                on:change=move |ev|{
                    set_config.update(|config| config.allow_crossings = event_target_checked(&ev));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[5]}</td>
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
                    <td>{move || languages::get_string_set(language()).options_content[6]}</td>
                </Show>
            </tr>
            <tr>
                // Letting lie
                <td> <input type="checkbox" prop:checked={move || config().force_letting_lie}
                on:change=move |ev|{
                    set_config.update(|config| config.force_letting_lie = event_target_checked(&ev));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[7]}</td>
                // Force counter movement
                <td> <input type="checkbox" prop:checked={move || config().force_base_countermovement}
                on:change=move |ev|{
                    set_config.update(|config| config.force_base_countermovement = event_target_checked(&ev));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[8]}</td>
            </tr>
            <tr>
                <td colspan=2> <input type="text" prop:value="5,8"
                    on:change=move |ev|{
                        set_config.update(|config| config.forbidden_parallels = event_target_value(&ev).replace(' ', "").split(',').flat_map(str::parse::<f32>).collect_vec());
                    }
                /> </td>
                <td colspan=2> <p>{move || languages::get_string_set(language()).options_content[9]}</p> </td>
            </tr>
            <tr>
                // Soprano jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_sopran}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_sopran = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[10]}</td>
                // Alto jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_alt}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_alt = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[11]}</td>
            </tr>
            <tr>
                // Tenor Jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_tenor}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_tenor = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[12]}</td>
                // Bass Jump
                <td> <input type="number" min="0" step="0.5" prop:value={move || config().max_jump_bass}
                on:change=move |ev|{
                    set_config.update(|config| config.max_jump_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[13]}</td>
            </tr>
            <tr>
                <th colspan="4">{move || languages::get_string_set(language()).options_content[14]}</th>
            </tr>
            <tr>
                // Sum of Absolutes weight
                <td> <input type="number" step="0.1" prop:value={move || format!("{:.1}", config().sub_of_abs_weight)}
                on:change=move |ev|{
                    set_config.update(|config| config.sub_of_abs_weight = event_target_value(&ev).parse::<f32>().unwrap_or(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[15]}</td>
                // Absolute of Sums weight
                <td> <input type="number" step="0.1" prop:value={move || format!("{:.1}", config().abs_of_sum_weight)}
                on:change=move |ev|{
                    set_config.update(|config| config.abs_of_sum_weight = event_target_value(&ev).parse::<f32>().unwrap_or(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[16]}</td>
            </tr>
            <tr>
                // Soprano-Alt-Weigth
                <td> <input type="number" step="0.1" prop:value={move || format!("{:.1}", config().soprano_alt_diff_weight)}
                on:change=move |ev|{
                    set_config.update(|config| config.soprano_alt_diff_weight = event_target_value(&ev).parse::<f32>().unwrap_or(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[17]}</td>
            </tr>
            <tr>
                <td> <input type="number" min="0" step="0.1" prop:value={move || format!("{:.1}", config().exposure_penalty_sopran)}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_penalty_sopran = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[18]}</td>

                <td> <input type="text" prop:value={move || config().exposure_threshold_sopran.to_string()}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_threshold_sopran = event_target_value(&ev).parse::<crate::logic::notes::OctavedNote>().unwrap_or_default());
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[19]}</td>
            </tr>
            <tr>
                <td> <input type="number" min="0" step="0.1" prop:value={move || format!("{:.1}", config().exposure_penalty_bass)}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_penalty_bass = event_target_value(&ev).parse::<f32>().unwrap_or(0.).max(0.));
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[20]}</td>

                <td> <input type="text" prop:value={move || config().exposure_threshold_bass.to_string()}
                on:change=move |ev|{
                    set_config.update(|config| config.exposure_threshold_bass = match language(){
                        languages::Language::German => event_target_value(&ev).parse::<crate::logic::notes::MultiNoteGerman>().unwrap_or_default().to_multinote().into(),
                        languages::Language::English =>event_target_value(&ev).parse::<crate::logic::notes::OctavedNote>().unwrap_or_default(),
                    });
                }
                /> </td>
                <td>{move || languages::get_string_set(language()).options_content[21]}</td>
            </tr>

        </table>
    }
}

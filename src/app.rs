use std::fmt::Debug;

use itertools::Itertools;
use leptos::*;

use crate::logic;

/// Contains the code to interact with the web_sys AudioAPI.
mod audio_sys;
/// Contains logic to create svgs from results.
mod svg;

mod note_info;
mod options;

mod result_view;
use result_view::SatbResultView;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MenuState {
    None,
    Options,
    Help,
}

#[component]
pub fn App() -> impl IntoView {
    let (config, set_config) = create_signal(logic::Config::default());

    let (result, set_result) = create_signal(Vec::new());

    let (shown_result, set_shown_result) = create_signal(5);

    let (input, set_input) = create_signal(String::new());

    let (menu_state, set_menu_state) = create_signal(MenuState::None);

    let (ctx, _set_ctx) =
        create_signal::<web_sys::AudioContext>(web_sys::AudioContext::new().unwrap());

    let audio_buffers = create_local_resource(ctx, audio_sys::fetch_all);

    view! {
        <h1>"Chorsatz"</h1>
        <div class = "outer_block">
            <div>
                <p> "Chorsatz ist eine Webapplikation zur automatischen Erstellung von vierstimmigen SATB-Stimmsätzen aus einer Reihenfolge von vorgegebenen Akkorden unter Beachtung der klassischen Stimmführungsregeln." </p>
            </div>
        </div>
        <div class="outer_block">
            <div class="ver">
            <div class="hor">
                <input
                    type="text"
                    class = "main_input"
                    placeholder="Akkorde hier eingeben..."
                    on:input=move |ev| {
                        set_input(event_target_value(&ev));
                    }
                    on:keypress=move |ev| {
                        // trigger only if enter is pressed
                        if ev.key_code() == 13{
                            set_result({
                                crate::logic::generate_satb(
                                    &event_target_value(&ev)
                                    .split(' ')
                                    .filter_map(|note_str| note_str.parse().ok())
                                    .collect_vec(),
                                    &config(),
                                ).into_iter().enumerate().collect_vec()
                            });
                            set_menu_state(MenuState::None);
                            set_shown_result(5);
                        }

                    }
                    prop:value=input
                ></input>
                <button id="generate"
                    on:click=move |_| {

                        set_result(
                        crate::logic::generate_satb(
                            &input()
                            .split(' ')
                            .filter_map(|note_str| note_str.parse().ok())
                            .collect_vec(),
                            &config(),
                        ).into_iter()
                        .enumerate().collect_vec());
                        set_menu_state(MenuState::None);
                        set_shown_result(5);
                    }
                >"Generieren"</button>
                <button id="options" class=move || if menu_state() == MenuState::Options { "active" } else { "" }
                    on:click=move |_|{
                        set_menu_state.update(|opt| {
                            *opt = match opt {
                                MenuState::Options => MenuState::None,
                                MenuState::None | MenuState::Help => MenuState::Options,
                            }
                        });
                    }
                >"Optionen"</button>
                <button id="accords" class=move || if menu_state() == MenuState::Help { "active" } else { "" }
                    on:click=move|_|{
                        set_menu_state.update(|opt| {
                            *opt = match opt {
                                MenuState::Help => MenuState::None,
                                MenuState::None | MenuState::Options => MenuState::Help,
                            }
                        });
                    }
                >"Eingabehilfe"</button>
            </div>

            {
                move || match menu_state() {
                    MenuState::None => None,
                    MenuState::Help => Some(view!{<note_info::Info/>}.into_view()),
                    MenuState::Options => Some(view!{<options::Options config=config set_config=set_config/>}.into_view())
                }
            }
            </div>
        </div>

        <div class="outer_block">
            <div>
                <h2>"Ergebnisse"</h2>
                <p class="deemph">"Eine kleinere Bewertung kennzeichnet eine optimalere Lösung. Da eine rein algorithmische Bewertung nicht perfekt ist, werden mehrere Ergebnisse zur Auswahl angezeigt."</p>
                <Show
                    when={move || result.with(|r| !r.is_empty())}
                    fallback=|| view!{}
                >
                    <p>"Es werden die besten " {move || shown_result().min(result.with(|r| r.len()))} " Ergebnisse aus " {move || result.with(|r| r.len())} " berechneten Lösungen angezeigt."</p>
                </Show>
                {
                    move || result()
                        .into_iter()
                        .take(shown_result())
                        .map(|(index, (res, score))|
                            view!{
                                <Show
                                    when={move || index < shown_result()}
                                    fallback=|| view!{}
                                >
                                <SatbResultView result=res.clone() res_score=score index=index audio_buffers=audio_buffers ctx=ctx/>
                                </Show>
                            }
                        ).collect_view()
                }
                <Show
                    when=move || {shown_result() < result.with(|r| r.len())}
                    fallback=|| view!{}
                >
                    <div class = "satbr_outer show_more_outer">
                        <button
                            id="show_more"
                            on:click=move |_| {
                                set_shown_result.update(|sr|{
                                    *sr +=5;
                                });
                            }
                        >
                            "Mehr Ergebnisse anzeigen..."
                        </button>
                    </div>
                </Show>
            </div>

        </div>

        <div class="outer_block">
            <div>
            <p>
                <b class="header">"Autoren:"</b>
                " Minona Schäfer & Linus Mußmächer"
                <br/>
                "Mit Dank an Biljana Wittstock"
            </p>
            <p>
                <b class="header"><a href="https://github.com/Dualraum/Chorsatz">"Github-Repository"</a></b>
                " - "
                <b class="header"><a class="header" href="https://github.com/Dualraum/Chorsatz/blob/main/howto/Chorsatz.pdf">"Bedienungshinweise"</a></b>
            </p>
            </div>
        </div>
    }
}

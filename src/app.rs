use std::fmt::Debug;

use itertools::Itertools;
use leptos::*;

use crate::logic::{self, notes::SatbTemplate};

/// Contains the code to interact with the web_sys AudioAPI.
mod audio_sys;
/// Contains logic to create svgs from results.
mod svg;

mod note_info;
mod options;

mod result_view;
use result_view::SatbResultView;

pub mod languages;

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

    let (ctx, set_ctx) = create_signal::<_>(None);

    let audio_buffers = create_local_resource(ctx, audio_sys::fetch_all);

    let (language, set_language) = create_signal(languages::Language::English);

    provide_context(language);

    view! {
        <h1>"Chorsatz"</h1>
        <div class = "outer_block">
            <div>
                <p> {move || languages::get_string_set(language()).intro} </p>
            </div>
            // Language selector
            <select name="language" id="language"
                on:change=move |ev|{
                    match event_target_value(&ev).as_str(){
                       "German" =>{set_language(languages::Language::German);}
                        _ =>{set_language(languages::Language::English);}
                    }
                }
            >
                <option value="English">English</option>
                <option value="German">Deutsch</option>
            </select>
        </div>
        <div class="outer_block">
            <div class="ver">
            <div class="hor">
                <input
                    type="text"
                    class = "main_input"
                    placeholder={move || languages::get_string_set(language()).input_default}
                    on:input=move |ev| {
                        set_input(event_target_value(&ev));
                            if ctx().is_none(){
                                set_ctx(web_sys::AudioContext::new().ok());

                            }
                    }
                    on:keypress=move |ev| {
                        // trigger only if enter is pressed
                        if ev.key_code() == 13{
                            // set result
                            set_result({
                                crate::logic::generate_satb(
                                    &event_target_value(&ev)
                                    .split([' ', ',', ';'])
                                    .filter_map(|note_str| SatbTemplate::from_str_language(note_str, language()).ok() )
                                    .collect_vec(),
                                    &config(),
                                ).into_iter().enumerate().collect_vec()
                            });
                            // set menu state
                            set_menu_state(MenuState::None);
                            set_shown_result(5);
                            // If the audio context does not yet exist, use this user interaction to create it
                            // (audio context can only be created during user interaction)
                            if ctx().is_none(){
                                set_ctx(web_sys::AudioContext::new().ok());

                            }
                        }

                    }
                    prop:value=input
                ></input>
                <button id="generate"
                    on:click=move |_| {
                            // set result
                            set_result({
                                crate::logic::generate_satb(
                                    &input()
                                    .split([' ', ',', ';'])
                                    .filter_map(|note_str| SatbTemplate::from_str_language(note_str, language()).ok() )
                                    .collect_vec(),
                                    &config(),
                                ).into_iter().enumerate().collect_vec()
                            });
                            // set menu state
                            set_menu_state(MenuState::None);
                            set_shown_result(5);
                            // If the audio context does not yet exist, use this user interaction to create it
                            // (audio context can only be created during user interaction)
                            if ctx().is_none(){
                                set_ctx(web_sys::AudioContext::new().ok());
                            }
                    }
                >{move || languages::get_string_set(language()).generate}</button>
                <button id="options" class=move || if menu_state() == MenuState::Options { "active" } else { "" }
                    on:click=move |_|{
                        set_menu_state.update(|opt| {
                            *opt = match opt {
                                MenuState::Options => MenuState::None,
                                MenuState::None | MenuState::Help => MenuState::Options,
                            }
                        });
                    }
                >{move || languages::get_string_set(language()).options}</button>
                <button id="accords" class=move || if menu_state() == MenuState::Help { "active" } else { "" }
                    on:click=move|_|{
                        set_menu_state.update(|opt| {
                            *opt = match opt {
                                MenuState::Help => MenuState::None,
                                MenuState::None | MenuState::Options => MenuState::Help,
                            }
                        });
                    }
                >{move || languages::get_string_set(language()).help}</button>
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
            <div class="result_container">
                <h2>{move || languages::get_string_set(language()).results_title}</h2>
                <p class="deemph">{move || languages::get_string_set(language()).results_algo_explation}</p>
                <Show
                    when={move || result.with(|r| !r.is_empty())}
                    fallback=|| view!{}
                >
                    <p>{
                        move ||
                        {( languages::get_string_set(language()).results_total)(
                            shown_result().min(result.with(|r| r.len())),
                            result.with(|r| r.len())
                        )}
                    }</p>
                </Show>
                {
                    move || {
                        // Check if an audio context exists and was created correctly.
                        if let Some(ctx) = ctx(){

                            let (ctx, _) = leptos::create_signal(ctx);


                            result()
                            .into_iter()
                            // .take(shown_result())
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

                        } else {
                            view!{

                            }.into_view()
                        }
                    }
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
                            {move || languages::get_string_set(language()).more}
                        </button>
                    </div>
                </Show>
            </div>

        </div>

        <div class="outer_block">
            <div>
            <p>
                <b class="header">{move || languages::get_string_set(language()).authors}</b>
                ": Minona Schäfer & Linus Mußmächer"
                <br/>
                {move || languages::get_string_set(language()).thanks}
                " Biljana Wittstock".
            </p>
            <p>
                <b class="header"><a class="header" href="https://github.com/Dualraum/Chorsatz">"Repository"</a></b>
                " - "
                <b class="header"><a class="header" href="https://github.com/Dualraum/Chorsatz/wiki">"Wiki"</a></b>
            </p>
            </div>
        </div>
    }
}

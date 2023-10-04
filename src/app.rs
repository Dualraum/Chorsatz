use itertools::Itertools;
use leptos::*;

use crate::logic;

mod note_info;
mod options;

mod result_view;
use result_view::SatbResultView;

#[component]
pub fn App() -> impl IntoView {
    let (config, set_config) = create_signal(logic::Config::default());

    let (result, set_result) = create_signal(Vec::new());

    let (result_amount, set_result_amount) = create_signal(5);

    let (input, set_input) = create_signal(String::new());

    let (options, set_options) = create_signal(false);
    let options_class = move || if options() { "col_open" } else { "col_closed" };
    let options_active = move || if options() { "active" } else { "" };
    let (accords, set_accords) = create_signal(false);
    let accords_class = move || if accords() { "col_open" } else { "col_closed" };
    let accords_active = move || if accords() { "active" } else { "" };

    view! { //class = styler_class,
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
                        set_options(false);
                        set_accords(false);
                    }
                >"Generieren"</button>
                <button id="options" class={options_active}
                    on:click=move |_|{
                        set_options.update(|opt| *opt = !*opt);
                        if options() {
                            set_accords(false);
                        }
                    }
                >"Optionen"</button>
                <button id="accords" class={accords_active}
                    on:click=move|_|{
                        set_accords.update(|acc| *acc = !*acc);
                        if accords() {
                            set_options(false);
                        }
                    }
                >"Eingabehilfe"</button>
            </div>
            <div class={options_class}>
                    <options::Options config=config set_config=set_config/>
            </div>
            <div class={accords_class}>
                    <note_info::Info/>
            </div>
            </div>
        </div>


        <div class="outer_block">
            <div>
                <h2>"Ergebnisse:"</h2>
                <p class="deemph">"Eine kleinere Bewertung kennzeichnet eine optimalere Lösung. Da eine rein algorithmische Bewertung nicht perfekt ist, werden mehrere Ergebnisse zur Auswahl angezeigt."</p>
                <p>"Es werden die besten " {result_amount} " Ergebnisse aus " {move || result().len()} " berechneten Lösungen angezeigt."</p>
                {
                    move || result().into_iter().map(|(index, (res, score))| view!{
                        <div class=move || if index < result_amount() { "col_open" } else { "col_closed" }>
                            <SatbResultView result=res res_score=score index=index/>
                        </div>
                    }).collect_view()
                }
                // <For
                //     each=result
                //     key=|(_index,(_res, score))| *score as i32
                //     view=move |(index, (res, score))| {
                //         view!{
                //             <div class=move || if index < result_amount() { "col_open" } else { "col_closed" }>
                //             <SatbResultView result=res res_score=score index=index/>
                //             </div>
                //         }
                //     }
                // />
            </div>
        </div>

        <div class="outer_block">
            <div>
            <p>
                <b class="header">"Autoren:"</b> " Minona Schäfer & Linus Mußmächer"
                " - " <b class="header"><a href="https://github.com/Dualraum/Chorsatz">Github</a></b>
                " - " <b class="header"><a class="header" href="https://github.com/Dualraum/Chorsatz/blob/main/howto/Akkordsatzprogramm.pdf">Anleitung</a></b>
            </p>
            </div>
        </div>
    }
}

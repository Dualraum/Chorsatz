use itertools::Itertools;
use leptos::*;

use crate::logic;

mod result_view;

use result_view::SatbResultView;
#[component]
pub fn App() -> impl IntoView {
    let (result, set_result) = create_signal(Vec::<(Vec<logic::notes::SatbBlock>, f32)>::new());

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
            <div>
            <div class="hor">
                <input
                    type="text"
                    placeholder="Akkorde hier eingeben..."
                    on:input=move |ev| {
                        set_input(event_target_value(&ev));
                    }
                    on:change=move |ev| {
                        set_result({
                            crate::logic::generate_satb(
                                &event_target_value(&ev)
                                .split(' ')
                                .filter_map(|note| logic::notes::create_multinote(note).ok())
                                .collect_vec()
                            ).into_iter().take(5).collect_vec()
                        });
                    }
                    prop:value=""
                ></input>
                <button id="generate"
                    on:click=move |_| {
                        crate::logic::generate_satb(
                            &input()
                            .split(' ')
                            .filter_map(|note| logic::notes::create_multinote(note).ok())
                            .collect_vec()
                        ).into_iter().take(5).collect_vec();
                    }
                >"Generieren"</button>
                <button id="options" class={options_active}
                    on:click=move |_|{
                        set_options(!options());
                        if options() {
                            set_accords(false);
                        }
                    }
                >"Optionen"</button>
                <button id="accords" class={accords_active}
                    on:click=move|_|{
                        set_accords(!accords());
                        if accords() {
                            set_options(false);
                        }
                    }
                >"Akkorde"</button>
            </div>
            <div class={options_class}>
                    <p>
                        "Hier werden in Zukunft mögliche Optionen zur Generierung stehen."
                    </p>
            </div>
            <div class={accords_class}>
                    <p>
                        "Hier wird in Zukunft eine Tabelle der Akkordabkürzungen stehen."
                    </p>
            </div>
            <p class="deemph">
                "Verfügbar sind sämtliche Dur- und Moll-Dreiklänge sowie verminderte, übermäßige, sus2 und sus4  Versionen dieser Akkorde.
                Außerdem stehen Dominantseptakkorde, Majorseptakkorde, Mollseptakkorde, Mollseptakkorde mit großer Septime sowie verminderte und übermäßige Septakkorde zur Verfügung.
                Verschiedene Akkorde sind durch Leerzeichen zu trennen, hierbei werden ungültige Eingaben ignoriert."
            </p>
            </div>
        </div>


        <div class="outer_block">
            <div>
                <h2>"Ergebnisse:"</h2>
                <p class="deemph">"Eine kleinere Bewertung kennzeichnet eine optimalere Lösung. Da eine rein algorithmische Bewertung nicht perfekt ist, werden mehrere Ergebnisse zur Auswahl angezeigt."</p>
                <For
                    each=result
                    key=|(_res,score)| *score as i32
                    view=move |(res, score)| {
                        view!{<SatbResultView result=res res_score=score/>}
                    }
                />
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

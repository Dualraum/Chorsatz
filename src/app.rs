use itertools::Itertools;
use leptos::*;

use crate::logic;

mod note_info;
mod options;

mod result_view;
use result_view::SatbResultView;

type ScoredResult = (Vec<logic::notes::SatbBlock>, f32);

#[component]
pub fn App() -> impl IntoView {
    let (config, set_config) = create_signal(logic::Config::default());

    let (result, set_result) = create_signal(Vec::new());

    let (result_amount, set_result_amount) = create_signal(5);

    let (input, set_input) = create_signal(String::new());

    let (options, set_options) = create_signal(false);
    let (accords, set_accords) = create_signal(false);

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
                            set_options(false);
                            set_accords(false);
                            set_result_amount(5);
                            draw_stuff(result);
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
                        set_result_amount(5);
                        draw_stuff(result);
                    }
                >"Generieren"</button>
                <button id="options" class=move || if options() { "active" } else { "" }
                    on:click=move |_|{
                        set_options.update(|opt| *opt = !*opt);
                        if options() {
                            set_accords(false);
                        }
                    }
                >"Optionen"</button>
                <button id="accords" class=move || if accords() { "active" } else { "" }
                    on:click=move|_|{
                        set_accords.update(|acc| *acc = !*acc);
                        if accords() {
                            set_options(false);
                        }
                    }
                >"Eingabehilfe"</button>
            </div>
            <div class=move || if options() { "col_open" } else { "col_closed" }>
                    <options::Options config=config set_config=set_config/>
            </div>
            <div class=move || if accords() { "col_open" } else { "col_closed" }>
                    <note_info::Info/>
            </div>
            </div>
        </div>

        <div class="outer_block">
            <div>
                <h2>"Ergebnisse:"</h2>
                <p class="deemph">"Eine kleinere Bewertung kennzeichnet eine optimalere Lösung. Da eine rein algorithmische Bewertung nicht perfekt ist, werden mehrere Ergebnisse zur Auswahl angezeigt."</p>
                <Show
                    when={move || !result().is_empty()}
                    fallback=|| view!{}
                >
                    <p>"Es werden die besten " {move || result_amount().min(result().len())} " Ergebnisse aus " {move || result().len()} " berechneten Lösungen angezeigt."</p>
                </Show>
                {
                    // Alternative: Remove inner Show and add take(result_amount()) in the iter chain. Will work!
                    // Pros: Less <--DynChild> in HTML
                    // Cons: Needs to recalculate entire vector on every "show-more"-click, and scrolls down to bottom instead of staying.
                    move || result().into_iter().map(|(index, (res, score))| view!{
                        <Show when=move || {index < result_amount()} fallback=|| view!{}>
                            <SatbResultView result=res.clone() res_score=score index=index/>
                        </Show>
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
                <Show
                    when=move || {result_amount() < result().len()}
                    fallback=|| view!{}
                >
                    <div class = "satbr_outer show_more_outer">
                        <button
                            id="show_more"
                            on:click=move |_| {
                                set_result_amount.update(|ra| *ra+=5);
                                draw_stuff(result);
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
                <b class="header">"Autoren:"</b> " Minona Schäfer & Linus Mußmächer"// mit freundlicher Unterstützung von Biljana <Nachname>"
            </p>
            <p>
                <b class="header"><a href="https://github.com/Dualraum/Chorsatz">"Github-Repository"</a></b>
                " - "
                <b class="header"><a class="header" href="https://github.com/Dualraum/Chorsatz/blob/main/howto/Akkordsatzprogramm.pdf">"Bedienungshinweise"</a></b>
            </p>
            </div>
        </div>
    }
}

use wasm_bindgen::prelude::*;

fn draw_stuff(result: ReadSignal<Vec<(usize, ScoredResult)>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    for (index, _blocks) in result() {
        let canvas = document
            .get_element_by_id(&format!("canvas{}", index))
            .unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // ----------- DRAW SMILEY -------------

        context.begin_path();

        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context
            .arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI)
            .unwrap();

        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context
            .arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        context.move_to(95.0, 65.0);
        context
            .arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();

        context.stroke();
    }
}

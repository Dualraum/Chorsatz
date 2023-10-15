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

    let (shown_result, set_shown_result) = create_signal(Vec::new());

    let (input, set_input) = create_signal(String::new());

    let (options, set_options) = create_signal(false);
    let (accords, set_accords) = create_signal(false);

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
                            set_options(false);
                            set_accords(false);
                            set_shown_result.update(|sr| {
                                *sr = result();
                                sr.truncate(5);
                            });
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
                        set_shown_result.update(|sr| {
                            *sr = result();
                            sr.truncate(5);
                        });
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
                    when={move || result.with(|r| !r.is_empty())}
                    fallback=|| view!{}
                >
                    <p>"Es werden die besten " {move || shown_result.with(|sr| sr.len()).min(result.with(|r| r.len()))} " Ergebnisse aus " {move || result.with(|r| r.len())} " berechneten Lösungen angezeigt."</p>
                </Show>
                {
                    move || shown_result().into_iter().map(|(index, (res, score))| view!{
                        <SatbResultView result=res.clone() res_score=score index=index/>
                    }).collect_view()
                }
                <Show
                    when=move || {shown_result.with(|sr| sr.len()) < result.with(|r| r.len())}
                    fallback=|| view!{}
                >
                    <div class = "satbr_outer show_more_outer">
                        <button
                            id="show_more"
                            on:click=move |_| {
                                set_shown_result.update(|sr|{
                                    let old_len = sr.len();
                                    *sr = result();
                                    sr.truncate(old_len + 5);
                                });
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

        <div class="storage">
            <img src="assets/full_staff.svg" id="full_staff"/>
            <img src="assets/quarter_up.svg" id="quarter_up"/>
            <img src="assets/quarter_down.svg" id="quarter_down"/>
            <img src="assets/staff_lines.svg" id="staff_lines"/>
        </div>
    }
}

use wasm_bindgen::prelude::*;

fn draw_stuff(result: ReadSignal<Vec<(usize, ScoredResult)>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    for (index, (blocks, _score)) in result() {
        let canvas = document
            .get_element_by_id(&format!("canvas{}", index))
            .expect("Could not find canvas.")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Could not cast canvas.");

        let context = canvas
            .get_context("2d")
            .expect("Could not unwrap 2d context.")
            .expect("Could not unwrap 2d context.")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Could not cast 2d context.");

        // ----------- DRAW STAFF --------------

        let staff = document
            .get_element_by_id("full_staff")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();

        // draw staff
        context
            .draw_image_with_html_image_element_and_dw_and_dh(&staff, 0., 0., 60., 140.)
            .unwrap();

        let staff = wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlImageElement>(
            document.get_element_by_id("staff_lines").unwrap(),
        )
        .unwrap();

        for i in 0..blocks.len() {
            context
                .draw_image_with_html_image_element_and_dw_and_dh(
                    &staff,
                    60. + i as f64 * 40.,
                    0.,
                    40.,
                    140.,
                )
                .unwrap();
        }

        // draw notes

        for (index, block) in blocks.iter().enumerate() {
            block.draw(&document, &context, 80. + 40. * index as f64, 110.);
        }
    }
}

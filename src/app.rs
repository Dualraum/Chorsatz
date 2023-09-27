use itertools::Itertools;
use leptos::*;

use crate::logic;

mod result_view;

use result_view::SatbResultView;
#[component]
pub fn App() -> impl IntoView {
    //let styler_class = stylers::style_sheet! {"./src/app/app.css"};

    let (result, set_result) = create_signal(Vec::<(Vec<logic::notes::SatbBlock>, f32)>::new());

    view! { //class = styler_class,
        <h1>"Chorsatz"</h1>
        <div class = "outer_block">
            <div>
                <p> "Chorsatz ist eine Webapplikation zur automatischen Erstellung von vierstimmigen SATB-Stimmsätzen aus einer Reihenfolge von vorgegebenen Akkorden unter Beachtung der klassischen Stimmführungsregeln." </p>
            </div>
        </div>

        <div class="outer_block">
            <div>
                <p class="deemph">"Akkorde durch Leerzeichen trennen."</p>
                <input
                    type="text"
                    placeholder="Akkorde hier eingeben..."
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
            </div>
        </div>


        <div class="outer_block">
            <div>
                <h2>"Ergebnisse:"</h2>
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
            </p>
            </div>
        </div>
    }
}

use itertools::Itertools;
use leptos::*;

use crate::logic;

#[component]
pub fn App() -> impl IntoView {
    //let styler_class = stylers::style_sheet! {"./src/app/app.css"};

    let (_result, set_result) = create_signal(Vec::<logic::notes::SatbBlock>::new());

    let (text_result, set_text_result) = create_signal("".to_string());

    view! { //class = styler_class,
        <h1>"Chorsatz"</h1>
        <div class = "visible">
            <div>
                <p> "Chorsatz ist eine Webapplikation zur automatischen Erstellung von vierstimmigen SATB-Stimmsätzen aus einer Reihenfolge von vorgegebenen Akkorden unter Beachtung der klassischen Stimmführungsregeln." </p>
            </div>
        </div>

        <div class="visible">
            <div>
                <h2>Eingabe:</h2>
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
                            ).first().unwrap().0.clone()
                        });

                        set_text_result(
                            crate::logic::generate_satb(
                                &event_target_value(&ev)
                                .split(' ')
                                .filter_map(|note| logic::notes::create_multinote(note).ok())
                                .collect_vec()
                            ).first().unwrap().0.iter().map(|block| block.to_string()).join(" - ")
                        );
                    }
                    prop:value=""
                ></input>
            </div>
        </div>

        <div class="visible">
            <div>
                <h2>Ausgabe:</h2>
                <p>{text_result}</p>
            </div>
        </div>
    }
}

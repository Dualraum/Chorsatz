use leptos::*;

use crate::logic::notes::SatbBlock;

#[component]
pub fn SatbResultView(result: Vec<SatbBlock>, res_score: f32) -> impl IntoView {
    view! {
        <div class = "satbr_outer">
            <h3>"Ergebnis"</h3>
            <p>"Bewertung: "{res_score as i32}</p>
            <div class = "satbr_inner">
                <div class = "satbb">
                    <p class="header">"Sopran"</p>
                    <p class="header">"Alt"</p>
                    <p class="header">"Tenor"</p>
                    <p class="header">"Bass"</p>
                </div>
                {result.iter().map(satb_block_view).collect_view()}
            </div>
        </div>
    }
}

fn satb_block_view(block: &SatbBlock) -> impl IntoView {
    view! {
        <div class = "satbb">
            <p>{block.0.to_string()}</p>
            <p>{block.1.to_string()}</p>
            <p>{block.2.to_string()}</p>
            <p>{block.3.to_string()}</p>
        </div>
    }
}

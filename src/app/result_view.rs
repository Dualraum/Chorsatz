use leptos::*;

use crate::logic::notes::SatbBlock;

#[component]
pub fn SatbResultView(result: Vec<SatbBlock>, res_score: f32, index: usize) -> impl IntoView {
    view! {
        <div class = "satbr_outer">
            <h3>"Ergebnis " {index+1}</h3>
            <p>"Bewertung: "<b class="marked">{res_score as i32}</b></p>
            <div class = "satbr_inner">
                <div class = "satbb">
                    <p class="header2">"Akkord"</p>
                    <p class="header">"Sopran"</p>
                    <p class="header">"Alt"</p>
                    <p class="header">"Tenor"</p>
                    <p class="header">"Bass"</p>
                </div>
                {result.iter().enumerate().map(satb_block_view).collect_view()}
            </div>
        </div>
    }
}

fn satb_block_view(block: (usize, &SatbBlock)) -> impl IntoView {
    view! {
        <div class = "satbb">
            <p class="header2">{block.0+1}</p>
            <p>{block.1.0.to_string()}</p>
            <p>{block.1.1.to_string()}</p>
            <p>{block.1.2.to_string()}</p>
            <p>{block.1.3.to_string()}</p>
        </div>
    }
}

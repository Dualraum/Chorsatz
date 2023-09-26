use leptos::*;

use crate::logic::notes::SatbBlock;

//#[component]
pub fn satb_result_view(result: &[SatbBlock]) -> impl IntoView {
    view! {
        <div class = "satbr">
            {result.iter().map(satb_block_view).collect_view()}
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

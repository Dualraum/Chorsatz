use js_sys::encode_uri_component;
use leptos::*;

use crate::logic::notes::SatbBlock;

#[component]
pub fn SatbResultView(result: Vec<SatbBlock>, res_score: f32, index: usize) -> impl IntoView {
    view! {
        <div class = "satbr_outer">
            <div class="row">
                <div class="col_lef">
                    <h3>"Ergebnis " {index+1}</h3>
                </div>
                <div class="col_rig">
                    <button id="sound" class="right"
                        on:click=move|_|{
                            if let Ok(sound) = web_sys::HtmlAudioElement::new_with_src("assets/notes/a-4.mp3"){
                                let _  = sound.play();
                            }
                        }
                    >"Abspielen"</button>
                </div>
            </div>
            <p>
                "Bewertung: "
                <b class="marked">{res_score as i32}</b>
                <br/>
                "Downloads:  "
                <a
                    class="dl"
                    href={format!("data:text/plain;charset=utf-8,{}", encode_uri_component(&crate::logic::generate_hum_file(&result)))}
                    download={format!("SATB-Result{}.hum", index+1)}
                >
                    ".hum"
                </a>
                "    "
                <a
                    class="dl"
                    href={format!("data:text/plain;charset=utf-8,{}", encode_uri_component(&crate::logic::generate_hum_file(&result)))}
                    download={format!("SATB-Result{}.ly", index+1)}
                >
                    ".ly (WIP)"
                </a>
                "    "
                <a
                    class="dl"
                    href={format!("data:text/plain;charset=utf-8,{}", encode_uri_component(&format!("{:?}", crate::app::svg::result_svg(&result).into_view())))}
                    download={format!("SATB-Result{}.svg", index+1)}
                >
                    ".svg"
                </a>
            </p>
            <div class = "satbr_inner">
                <div class = "satbb">
                    <p class="header2">"Akkord"</p>
                    <p class="header">"Sopran"</p>
                    <p class="header">"Alt"</p>
                    <p class="header">"Tenor"</p>
                    <p class="header">"Bass"</p>
                </div>
                {result.iter().enumerate().map(satb_block_view).collect_view()}
                {crate::app::svg::result_svg(&result)}
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

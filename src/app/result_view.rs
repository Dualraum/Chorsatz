use itertools::Itertools;
use js_sys::encode_uri_component;
use leptos::*;
use web_sys::AudioBuffer;

use crate::fetcher::buffer_to_src_node;
use crate::logic::notes::*;

#[component]
pub fn SatbResultView(
    result: Vec<SatbBlock>,
    res_score: f32,
    index: usize,
    audio_buffers: Resource<
        web_sys::AudioContext,
        std::collections::HashMap<OctavedNote, AudioBuffer>,
    >,
    ctx: ReadSignal<web_sys::AudioContext>,
) -> impl IntoView {
    // let _ = super::fetcher::buffer_to_src_node(ctx.get(), thing).unwrap().start();
    // map all the notes in the result to their appropriate mp3-files

    // Request the cached audio buffers
    let cached_buffers = audio_buffers().expect("Could not unpack audio buffers.");

    // Create a vector of quadruptlets of references to the correct buffers for each SATB-block
    let sound = result
        .iter()
        // flatten the map so it contains all notes in order
        .flat_map(|block| [block.0, block.1, block.2, block.3])
        // now map every not to an audio buffer
        .flat_map(|note| {
            Ok::<AudioBuffer, ChorError>(
                cached_buffers
                    .get(&note)
                    .ok_or(ChorError::NoMp3Error(note))?
                    .clone(),
            )
        })
        .collect_vec();
    // This vector is later moved into the closure owned by the 'play' button

    view! {
        <div class = "satbr_outer">
            <div class="row">
                <div class="col_lef">
                    <h3>"Ergebnis " {index+1}</h3>
                </div>
                <div class="col_rig">
                    <button id="sound" class="right"
                        on:click=move|_|{
                            // save the context and its time
                            let ctx = ctx();
                            let time = ctx.current_time();
                            // now play every note. Every 4 notes form a block, so the 'when'-time is increased every 4 notes
                            for (index, buffer) in sound.iter().enumerate(){
                                    let _ = buffer_to_src_node(&ctx,&buffer).unwrap().start_with_when(time + 1.5 * (index/4) as f64 );
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
                "    "
                <a
                    class="dl"
                    href={format!("data:text/plain;charset=utf-8,{}", encode_uri_component(&format!("{:?}", crate::app::svg::result_svg(&result).into_view())))}
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

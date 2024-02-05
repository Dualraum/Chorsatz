use itertools::Itertools;
use js_sys::encode_uri_component;
use leptos::*;
use web_sys::AudioBuffer;

use super::audio_sys;
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
    // --- First, request some resources for audio playback

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

    // --- Create a signal to track a highlighted note

    let (highlight, set_highlight) = create_signal::<usize>(result.len());
    provide_context(highlight);

    // --- Create a concatenated buffer to create the mp3 from ---

    let concat_buffer =
        audio_sys::concat_buffers(&ctx(), &sound).expect("Could not concat buffers.");

    let (show_table, set_show_table) = create_signal(false);

    // --- Now, create the view itself

    view! {
        <div class = "satbr_outer">
            <div class="row">
                <div class="col_lef">
                    <h3>"Ergebnis " {index+1}</h3>
                </div>
                <div class="col_rig">
                    // These are in reversed order, as col_rig floats from the right
                    <button id="sound" class="right"
                        on:click=move|_|{
                            set_highlight(0);
                            // save the context and its time
                            let ctx = ctx();
                            let time = ctx.current_time();
                            // now play every note. Every 4 notes form a block, so the 'when'-time is increased every 4 notes
                            for (index, buffer) in sound.iter().enumerate(){
                                    let _ = audio_sys::buffer_to_src_node(&ctx,&buffer).expect("Could not convert buffer to source node.").start_with_when(time + 1.5 * (index/4) as f64 );
                                    if index % 4 == 0{
                                        set_timeout(move || set_highlight(index/4 + 1), std::time::Duration::from_secs_f32((index/4 + 1) as f32 * 1.5));
                                    }
                            }
                        }
                    >"Abspielen"</button>
                    <a
                        class="dl"
                        href={
                            // Convert concatenated audio buffer to a blob.
                            let blob = audio_sys::buffer_to_blob(&concat_buffer).expect("Could not convert buffer to blob.");
                            // Create object URL to download the blob.
                            web_sys::Url::create_object_url_with_blob(&blob).expect("Could not create object URL for wav file.")
                        }
                        download={format!("SATB-Result{}.wav", index+1)}
                    >
                        "WAV"
                    </a>
                    <a
                        class="dl"
                        href={
                            // create the result svg and provide it as a plaintext download
                            format!("data:text/plain;charset=utf-8,{}", encode_uri_component(&format!("{:?}", view!{<crate::app::svg::ResultSvg result=result.clone()/>})))
                        }
                        download={format!("SATB-Result{}.svg", index+1)}
                    >
                        "SVG"
                    </a>
                </div>
            </div>
            <div class = "satbr_inner">
                <crate::app::svg::ResultSvg result=result.clone()/>
                <Show
                    when={show_table}
                    fallback=|| view!{}
                >
                    <div class = "satbb">
                        <p class="header2">"Akkord"</p>
                        <p class="header">"Sopran"</p>
                        <p class="header">"Alt"</p>
                        <p class="header">"Tenor"</p>
                        <p class="header">"Bass"</p>
                    </div>
                    {result.iter().enumerate().map(satb_block_view).collect_view()}
                </Show>
            </div>
            <div class="row">
                <div class="col_lef">
                        <div class="tooltip">
                            <span class="tooltiptext">"Bewertung dieses Ergebnisses berechnet aus der relativen Notenlage. Kleine Bewertungen kennzeichnen bessere Lösungen."</span>
                            <b class="score"> {res_score as i32}</b>
                        </div>
                </div>
                <div class="col_rig">
                    <button id="table" class="right"
                        on:click=move |_|{
                            set_show_table.update(|a| *a = !*a);
                        }
                    >"Notentabelle"</button>
                </div>
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

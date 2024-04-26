use js_sys::encode_uri_component;
use leptos::*;
use web_sys::AudioBuffer;

use super::audio_sys;
use crate::{app::languages, logic::notes::*};

#[component]
pub fn SatbResultView(
    result: Vec<SatbBlock>,
    res_score: f32,
    index: usize,
    audio_buffers: Resource<
        Option<web_sys::AudioContext>,
        std::collections::HashMap<OctavedNote, AudioBuffer>,
    >,
    ctx: ReadSignal<web_sys::AudioContext>,
) -> impl IntoView {
    // Get the used language
    let language = use_context::<ReadSignal<languages::Language>>()
        .unwrap_or_else(|| create_signal(languages::Language::English).0);

    // --- First, request some resources for audio playback

    // Request the cached audio buffers
    let sound = audio_buffers().and_then(|buffers| {
        // Create a vector of quadruptlets of references to the correct buffers for each SATB-block
        // This vector is later moved into the closure owned by the 'play' button
        result
            .iter()
            // flatten the map so it contains all notes in order
            .flat_map(|block| [block.0, block.1, block.2, block.3])
            // now map every not to an audio buffer
            .map(|note| buffers.get(&note).cloned())
            // only create a vec if all notes could be found
            .collect::<Option<Vec<_>>>()
    });

    // --- Create a signal to track a highlighted note

    let (highlight, set_highlight) = create_signal::<usize>(result.len());
    provide_context(highlight);

    // --- Create a concatenated buffer to create the mp3 from ---

    let concat_buffer = sound
        .as_ref()
        .and_then(|sound| audio_sys::concat_buffers(&ctx(), sound).ok());

    let (show_table, set_show_table) = create_signal(false);

    // --- Now, create the view itself

    view! {
        <div class = "satbr_outer">
            <div class="row">
                <div class="col_lef">
                    <h3>{move || languages::get_string_set(language()).result_title} " " {index+1}</h3>
                </div>
                <div class="col_rig">
                    // These are in reversed order, as col_rig floats from the right
                    <button id="sound" class="right"
                        on:click=move|_|{
                            // check if sounds could be retrieved
                            if let Some(sound) = &sound{
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
                        }
                    >{move || languages::get_string_set(language()).result_play}</button>
                    {
                        // Provide a download only if a concatenation buffer could be created.
                        if let Some(concat_buffer) = concat_buffer{
                            view!{
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
                            }.into_view()
                        } else {
                            view!{}.into_view()
                        }
                    }
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
                        <p class="header2">{move || languages::get_string_set(language()).table_chord}</p>
                        <p class="header">{move || languages::get_string_set(language()).table_voices[0]}</p>
                        <p class="header">{move || languages::get_string_set(language()).table_voices[1]}</p>
                        <p class="header">{move || languages::get_string_set(language()).table_voices[2]}</p>
                        <p class="header">{move || languages::get_string_set(language()).table_voices[3]}</p>
                    </div>
                    {result.iter().enumerate().map(satb_block_view).collect_view()}
                </Show>
            </div>
            <div class="row">
                <div class="col_lef">
                        <div class="tooltip">
                            <span class="tooltiptext">{move || languages::get_string_set(language()).score_tooltip}</span>
                            <b class="score"> {res_score as i32}</b>
                        </div>
                </div>
                <div class="col_rig">
                    <button id="table" class="right"
                        on:click=move |_|{
                            set_show_table.update(|a| *a = !*a);
                        }
                    >{move || languages::get_string_set(language()).result_table}</button>
                </div>
            </div>
        </div>
    }
}

fn satb_block_view(block: (usize, &SatbBlock)) -> impl IntoView {
    // Get the used language
    let language = use_context::<ReadSignal<languages::Language>>()
        .unwrap_or_else(|| create_signal(languages::Language::English).0);

    let formatter = |o_note: OctavedNote| match language() {
        languages::Language::English => o_note.to_string(),
        languages::Language::German => o_note.to_german_name(),
    };

    view! {
        <div class = "satbb">
            <p class="header2">{block.0+1}</p>
            <p>{formatter(block.1.0)}</p>
            <p>{formatter(block.1.1)}</p>
            <p>{formatter(block.1.2)}</p>
            <p>{formatter(block.1.3)}</p>
        </div>
    }
}

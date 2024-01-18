use itertools::Itertools;
use js_sys::encode_uri_component;
use leptos::*;
use web_sys::HtmlAudioElement;

use crate::logic::notes::SatbBlock;

#[component]
pub fn SatbResultView(result: Vec<SatbBlock>, res_score: f32, index: usize) -> impl IntoView {
    // map all the notes in the result to their appropriate mp3-files
    let sound = result
        .iter()
        .flat_map(|block| {
            Ok::<
                (
                    HtmlAudioElement,
                    HtmlAudioElement,
                    HtmlAudioElement,
                    HtmlAudioElement,
                ),
                wasm_bindgen::JsValue,
            >((
                block.0.to_mp3()?,
                block.1.to_mp3()?,
                block.2.to_mp3()?,
                block.3.to_mp3()?,
            ))
        })
        .collect_vec();

    let ctx = web_sys::AudioContext::new().unwrap();

    let x = crate::logic::notes::OctavedNote::new(crate::logic::notes::NoteName::A, 1);
    let m = x.to_audio_buffer(&ctx);
    let m = futures::executor::block_on(m);

    // let accords = result
    //     .iter()
    //     .flat_map(|block| {
    //         Ok::<web_sys::AudioBufferSourceNode, wasm_bindgen::JsValue>({
    //             let notes = async {
    //                 let note0 = block.0.to_audio_buffer(&ctx).await.ok();
    //                 let note1 = block.1.to_audio_buffer(&ctx).await.ok();
    //                 let note2 = block.2.to_audio_buffer(&ctx).await.ok();
    //                 let note3 = block.3.to_audio_buffer(&ctx).await.ok();

    //                 let source = ctx.create_buffer_source().unwrap();

    //                 // source.set_buffer(note0.as_ref());

    //                 source
    //             };

    //             futures::executor::block_on(notes)
    //         })
    //     })
    //     .collect_vec();

    view! {
        <div class = "satbr_outer">
            <div class="row">
                <div class="col_lef">
                    <h3>"Ergebnis " {index+1}</h3>
                </div>
                <div class="col_rig">
                    <button id="sound" class="right"
                        on:click=move|_|{
                            // for (index, mp3_block) in sound.iter().cloned().enumerate(){
                            //     set_timeout(move || {
                            //         let _ = mp3_block.0.play();
                            //         let _ = mp3_block.1.play();
                            //         let _ = mp3_block.2.play();
                            //         let _ = mp3_block.3.play();
                            //     }, std::time::Duration::from_secs_f32(2.0 * index as f32 + 0.3))
                            // }
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

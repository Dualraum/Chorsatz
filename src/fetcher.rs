use std::collections::HashMap;

use super::logic::notes::*;

use futures::future::join_all;
use js_sys::ArrayBuffer;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AudioBuffer, AudioBufferSourceNode, AudioContext, Request, RequestInit, RequestMode, Response,
};

async fn fetch_buffer(
    note: OctavedNote,
    ctx: &AudioContext,
) -> Result<(OctavedNote, AudioBuffer), JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("/assets/notes/{}.mp3", note.to_playable_note());

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let value = JsFuture::from(resp.array_buffer()?).await?;

    // Read out the JsValue to an ArrayBuffer
    let array = ArrayBuffer::from(value);

    // Decode the Array Buffer to an AudioBuffer
    let audio_buffer: AudioBuffer = JsFuture::from(ctx.decode_audio_data(&array)?)
        .await?
        .dyn_into()?;

    Ok((note, audio_buffer))
}

pub async fn fetch_all(ctx: AudioContext) -> HashMap<OctavedNote, AudioBuffer> {
    let mut futures = Vec::new();
    for note_name in NoteName::iter() {
        for oct in [-1, 0, 1, 2].iter().copied() {
            let note = OctavedNote::new(note_name, oct);
            futures.push(fetch_buffer(note, &ctx));
        }
    }

    join_all(futures).await.into_iter().flatten().collect()
}

pub fn buffer_to_src_node(
    ctx: AudioContext,
    buffer: &AudioBuffer,
) -> Result<AudioBufferSourceNode, JsValue> {
    // Put the AudioBuffer into an AudioSourceNode
    let src = ctx.create_buffer_source()?;
    src.set_buffer(Some(&buffer));
    src.connect_with_audio_node(&ctx.destination().into())?;

    // Return this AudioSourceNode
    Ok(src)
}

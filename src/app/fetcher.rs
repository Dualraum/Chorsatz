use std::{
    collections::HashMap,
    io::{BufWriter, Read},
};

use super::logic::notes::*;

use itertools::Itertools;

use futures::future::join_all;
use js_sys::ArrayBuffer;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AudioBuffer, AudioBufferSourceNode, AudioContext, Request, RequestInit, RequestMode, Response,
};

/// Fetches the mp3-File for a single note from the database and converts it into an AudioBuffer using the provided AudioContext.
async fn fetch_buffer(
    note: OctavedNote,
    ctx: &AudioContext,
) -> Result<(OctavedNote, AudioBuffer), JsValue> {
    // Create a CORS request option
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // Buidl the URL
    let url = format!("./assets/notes/{}.mp3", note.to_playable_note());

    // Create the requst
    let request = Request::new_with_str_and_init(&url, &opts)?;

    // Get the window and asynchronously make the request
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // Make sure we got a Response back and convert it into its type
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

/// Fetches the mp3-Files for all notes in the NoteName enum and the octaves [-1, 0, 1, 2] ([2, 3, 4, 5] in American notation) and puts them into a HashMap.
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

/// Takes a reference to an AudioBuffer and converts it into an AudioBufferSourceNode that is then connected to the provided AUdioContext and can be started exactly once.
pub fn buffer_to_src_node(
    ctx: &AudioContext,
    buffer: &AudioBuffer,
) -> Result<AudioBufferSourceNode, JsValue> {
    // Put the AudioBuffer into an AudioSourceNode
    let src = ctx.create_buffer_source()?;
    src.set_buffer(Some(&buffer));
    src.connect_with_audio_node(&ctx.destination().into())?;

    // Return this AudioSourceNode
    Ok(src)
}

pub fn concat_buffers(ctx: &AudioContext, buffers: &[AudioBuffer]) -> Result<AudioBuffer, JsValue> {
    let sum_buffer = buffers
        .iter()
        .map(|buffer| {
            let vec = buffer.get_channel_data(0).unwrap();
            vec
        })
        .tuples::<(_, _, _, _)>()
        .map(|(a, b, c, d)| {
            a.iter()
                .zip(b.iter())
                .zip(c.iter())
                .zip(d.iter())
                .map(|(((a, b), c), d)| *a + *b + *c + *d)
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    let res_buffer = ctx.create_buffer(1, sum_buffer.len() as u32, ctx.sample_rate())?;

    res_buffer.copy_to_channel(&sum_buffer, 0)?;

    Ok(res_buffer)
}

pub fn buffer_to_blob(ctx: &AudioContext, buffer: &AudioBuffer) -> Result<web_sys::Blob, JsValue> {
    let source = ctx.create_buffer_source()?;
    source.set_buffer(Some(buffer));
    source.connect_with_audio_node(&ctx.destination())?;

    let pcm = buffer.get_channel_data(0)?;

    let wav = pcm_to_mp3(&pcm, ctx.sample_rate());

    web_sys::Blob::new_with_u8_array_sequence_and_options(
        &JsValue::from(unsafe { js_sys::Uint8Array::view(&wav) }),
        web_sys::BlobPropertyBag::new().type_("audio/wav"),
    )
}

fn pcm_to_mp3(pcm_data: &[f32], sample_rate: f32) -> Vec<u8> {
    vec![0]
}

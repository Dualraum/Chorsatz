use std::collections::HashMap;

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

/// Concatenates an iterator of AudioBuffers.
/// The values of each group of 4 audio buffers are added up, and these groups are then appended to a single audio buffer in sequence (with the length each group is played for determined by the shortest element of a group).
/// Example: An iterator [A B C D E F G H] will be turned into a single buffer first containing A+B+C+D then E+F+G+H, with E+F+G+H starting to play after A,B,C or D (whichever is shortest) is finished.
pub fn concat_buffers(ctx: &AudioContext, buffers: &[AudioBuffer]) -> Result<AudioBuffer, JsValue> {
    let sum_buffer = buffers
        .iter()
        // Convert buffers to float-Vecs
        .map(|buffer| {
            let vec = buffer.get_channel_data(0).unwrap();
            vec
        })
        // Make groups of 4
        .tuples::<(_, _, _, _)>()
        // Add those groups up by zipping them
        .map(|(a, b, c, d)| {
            a.iter()
                .zip(b.iter())
                .zip(c.iter())
                .zip(d.iter())
                .map(|(((a, b), c), d)| *a + *b + *c + *d)
                .collect_vec()
        })
        // Concatenate all those added buffers into a concurrent large vec
        .flatten()
        .collect_vec();

    // Turn the float-Vec back into an AudioBuffer
    let res_buffer = ctx.create_buffer(1, sum_buffer.len() as u32, ctx.sample_rate())?;

    res_buffer.copy_to_channel(&sum_buffer, 0)?;

    Ok(res_buffer)
}

/// Converts an web_sys::AudioBuffer into a Blob that represents a .wav-File containing the audio.
pub fn buffer_to_blob(buffer: &AudioBuffer) -> Result<web_sys::Blob, JsValue> {
    // Write a standard .wav header and the f32 data from the AudioBuffer to a Vec of bytes (u8).
    let wav =
        wav_io::write_to_bytes(&wav_io::new_mono_header(), &buffer.get_channel_data(0)?).unwrap();

    // Complicated conversion to a js_sys::Array, directly taking a js_sys::Uint8Array (just the first line) does NOT work, see:https://stackoverflow.com/questions/69556755/web-sysurlcreate-object-url-with-blobblob-not-formatting-binary-data-co.
    let uint8arr = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&wav) }.into());
    let array = js_sys::Array::new();
    array.push(&uint8arr.buffer());

    // Convert this array of bytes into a blob and tell the system its a wav.
    web_sys::Blob::new_with_u8_array_sequence_and_options(
        &array,
        web_sys::BlobPropertyBag::new().type_("audio/wav"),
    )
}

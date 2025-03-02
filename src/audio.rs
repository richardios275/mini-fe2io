use std::io::Cursor;
use tokio::sync::mpsc::Receiver;
use reqwest::Client;
use rodio::{Decoder, Sink};
use anyhow::{Context, Error};

pub async fn audio_loop(mut rx: Receiver<String>, sink: Sink) -> Result<(), Error> {
    let client = Client::new();
    loop {
        let input = rx.recv().await
            .context("Audio channel closed")?;
        match input.as_str() {
            "died" => sink.set_volume(0.5),
            "left" => sink.stop(),
            _ => play_audio(input, &client, &sink).await?,
        }
    }
}

pub async fn play_audio(url: String, client: &Client, sink: &Sink) -> Result<(), Error> {
    println!("\nGot request to play audio {}", url);
    // Stop the current playback, if any
    sink.stop();

    // Get response from URL
    let response = client.get(url)
        .send()
        .await?
        .bytes()
        .await?;

    println!("Got response");

    // Play the downloaded audio
    let cursor = Cursor::new(response);
    let decoder = Decoder::new(cursor)?;
    sink.append(decoder);
    println!("Playback started");
    Ok(())
}
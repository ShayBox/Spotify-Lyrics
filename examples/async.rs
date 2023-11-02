use spotify_lyrics::SpotifyLyrics;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Anonymous - Can't get lyrics
    // let mut spotify_lyrics = SpotifyLyrics::builder()?.build().await?;

    // Authenticated - Manual
    // let cookie = String::from("");
    // let mut spotify_lyrics = SpotifyLyrics::builder()?.cookie(cookie).build().await?;

    // Authenticated - Browser
    use spotify_lyrics::Browser;
    let mut spotify_lyrics = SpotifyLyrics::builder()?
        .browser(Browser::All)?
        .build()
        .await?;

    let track_id = "0Vm2QYFSU2RWSPAReJR80D";
    let color_lyrics = spotify_lyrics.get_color_lyrics(track_id).await?;
    println!("{color_lyrics:#?}");

    Ok(())
}

use spotify_lyrics::{Browser, SpotifyLyrics};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Anonymous - Can't get lyrics
    // let mut spotify_lyrics = SpotifyLyrics::default();

    // Authenticated - Manually specify a cookie
    // let cookie = "";
    // let mut spotify_lyrics = SpotifyLyrics::from_cookie(cookie)?;

    // Authenticated - Automatically get cookie from users web browser
    let mut spotify_lyrics = SpotifyLyrics::from_browser(Browser::All)?;

    // Optionally check for first authentication errors
    spotify_lyrics.refresh_authorization().await?;

    let track_id = "0Vm2QYFSU2RWSPAReJR80D";
    let color_lyrics = spotify_lyrics.get_color_lyrics(track_id).await?;
    println!("{color_lyrics:#?}");

    Ok(())
}

use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{bail, Result};
#[cfg(feature = "is_sync")]
use reqwest::blocking::Client;
use reqwest::cookie::Jar;
#[cfg(not(feature = "is_sync"))]
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

const BASE_URL: &str = "https://spclient.wg.spotify.com";
const COOKIE_DOMAIN: &str = ".spotify.com";
const COOKIE_NAME: &str = "sp_dc";
const TOKEN_URL: &str = "https://open.spotify.com/get_access_token";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36";
/* ^ This could be fetched from a list at runtime but I don't suspect this will need to be changed ^ */

lazy_static::lazy_static! {
    static ref COOKIE_URL: Url = format!("https://open{COOKIE_DOMAIN}").parse().unwrap();
}

#[cfg(feature = "browser")]
#[derive(Debug)]
pub enum Browser {
    All,
    Brave,
    Chrome,
    Chromium,
    Edge,
    Firefox,
    InternetExplorer,
    LibreWolf,
    Opera,
    OperaGX,
    #[cfg(target_os = "macos")]
    Safari,
    Vivaldi,
}

#[derive(Debug, Default)]
pub struct SpotifyLyricsBuilder {
    cookie: String,
    client: Client,
    jar:    Arc<Jar>,
}

impl SpotifyLyricsBuilder {
    pub fn new() -> Result<Self> {
        let jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_store(true)
            .cookie_provider(jar.clone())
            .user_agent(USER_AGENT)
            .build()?;

        Ok(SpotifyLyricsBuilder {
            client,
            jar,
            ..Default::default()
        })
    }

    /// Manually supply your own cookie
    /// Optional - Spotify allows anonymous access tokens (default)
    pub fn cookie(mut self, cookie: String) -> Result<Self> {
        self.jar.add_cookie_str(&cookie, &COOKIE_URL);
        self.cookie = cookie;

        Ok(self)
    }

    /// Try to get the cookie from the users web browser
    /// Optional - Spotify allows anonymous access tokens (default)
    #[cfg(feature = "browser")]
    pub fn browser(self, browser: Browser) -> Result<Self> {
        use rookie::CookieToString;

        let get_cookies = match browser {
            Browser::All => rookie::load,
            Browser::Brave => rookie::brave,
            Browser::Chrome => rookie::chrome,
            Browser::Chromium => rookie::chromium,
            Browser::Edge => rookie::edge,
            Browser::Firefox => rookie::firefox,
            Browser::InternetExplorer => rookie::internet_explorer,
            Browser::LibreWolf => rookie::libre_wolf,
            Browser::Opera => rookie::opera,
            Browser::OperaGX => rookie::opera_gx,
            #[cfg(target_os = "macos")]
            Browser::Safari => rookie::safari,
            Browser::Vivaldi => rookie::vivaldi,
        };

        let domains = Some(vec![COOKIE_DOMAIN]);
        let Ok(cookies) = get_cookies(domains) else {
            bail!("Couldn't find any cookies in {browser:?} browser")
        };

        let cookie = cookies
            .into_iter()
            .filter(|cookie| cookie.name == COOKIE_NAME)
            .collect::<Vec<_>>()
            .to_string();

        self.cookie(cookie)
    }

    pub async fn build(self) -> Result<SpotifyLyrics> {
        let mut spotify_lyrics = SpotifyLyrics {
            client: self.client,
            ..Default::default()
        };

        spotify_lyrics.refresh_authorization().await?;

        Ok(spotify_lyrics)
    }
}

#[derive(Debug, Default)]
pub struct SpotifyLyrics {
    client:        Client,
    authorization: Authorization,
}

impl SpotifyLyrics {
    pub fn builder() -> Result<SpotifyLyricsBuilder> {
        SpotifyLyricsBuilder::new()
    }

    #[maybe_async::maybe_async]
    pub async fn refresh_authorization(&mut self) -> Result<()> {
        let response = self.client.get(TOKEN_URL).send().await?;
        self.authorization = response.json().await?;

        Ok(())
    }

    #[maybe_async::maybe_async]
    pub async fn get_authorization(&mut self) -> Result<Authorization> {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let expiration = Duration::from_millis(self.authorization.expiration_timestamp_ms);
        if current_time > expiration {
            info!("Refreshing authorization");
            self.refresh_authorization().await?;
        };

        Ok(self.authorization.clone())
    }

    #[maybe_async::maybe_async]
    pub async fn get_color_lyrics(&mut self, track_id: &str) -> Result<ColorLyrics> {
        let url = format!("{BASE_URL}/color-lyrics/v2/track/{track_id}?format=json");
        let authorization = self.get_authorization().await?;
        let access_token = format!("Bearer {}", authorization.access_token);
        let response = self
            .client
            .get(url)
            .header("Authorization", access_token)
            .header("App-Platform", "WebPlayer")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            bail!("Couldn't get color lyrics: {status}")
        };

        Ok(response.json().await?)
    }
}

/* Please feel free to create an issue or pull request to expand as needed */

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorization {
    pub client_id: String,
    pub access_token: String,
    #[serde(rename = "accessTokenExpirationTimestampMs")]
    pub expiration_timestamp_ms: u64,
    pub is_anonymous: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorLyrics {
    pub lyrics: Lyrics,
    pub colors: Colors,
    pub has_vocal_removal: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    pub sync_type: String,
    pub lines: Vec<Line>,
    pub provider: String,
    pub provider_lyrics_id: String,
    pub provider_display_name: String,
    pub sync_lyrics_uri: String,
    pub is_dense_typeface: bool,
    // pub alternatives: Vec<Value>,
    pub language: String,
    pub is_rtl_language: bool,
    pub fullscreen_action: String,
    pub show_upsell: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub start_time_ms: String,
    pub words:         String,
    // pub syllables: Vec<Value>,
    pub end_time_ms:   String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    pub background:     i64,
    pub text:           i64,
    pub highlight_text: i64,
}

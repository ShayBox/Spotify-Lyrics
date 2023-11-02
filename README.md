# Spotify Lyrics

Spotify Lyrics API Wrapper

Based on [spotify-lyrics-api](https://github.com/akashrchandran/spotify-lyrics-api), [syrics](https://github.com/akashrchandran/syrics), [lyricstify](https://github.com/lyricstify/lyricstify), and [verses](https://github.com/Maxuss/verses)

## [Examples](/examples)
- [async](/examples/async.rs) - Default feature
- [sync](/examples/sync.rs) - `is_sync` feature

## [Features](/Cargo.toml)
- `rustls-tls` - RustTLS (Default)
- `native-tls` - OpenSSL
- `browser` (Default)
  - Adds the `browser` method to the builder to get the cookie from local browser(s)
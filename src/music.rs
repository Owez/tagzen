//! Music file tagging for an artist and song approch, favouring em dashes

use crate::utils::ResponseModel;

/// Gives help by providing available endpoints (to [song] and [album])
#[get("/music")]
pub fn help() -> &'static str {
    "ROUTE /music\n\n\nAbout\n    Allows music tagging with a static/strong artist + albumn + song methodoloy\n    of tagging. Formatting uses an em dash to differentiate these layers.\n\nChild routes/endpoints\n    - /song: Tags a single song and allows optional context for artist/album"
}

/// Gives help for how to use the [song] path
#[get("/music/song")]
pub fn song_help() -> &'static str {
    "POST /music/song?<name>&<album>&<artist>\n\n\nAbout\n    Tags a single song path into the typical artist + album + song view. Some\n    optional url parameters may be passed like `album` and `artist` in order to\n    give explicit context for tagging the song."
}

/// Tags a single song into a song, album and artist. This is typically used for
/// playlists where songs are not in any exact order
#[post("/music/song?<name>&<album>&<artist>")]
pub fn song(name: String, album: Option<String>, artist: Option<String>) -> ResponseModel<()> {
    unimplemented!()
}

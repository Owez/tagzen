//! Music file tagging for an artist and song approch, favouring em dashes

use crate::utils::{cap_filename_ext, format_name, ResponseModel};

use serde::Serialize;

/// A single song that is pretty printed for tagging, used in the [song] path
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SingleSong {
    /// Original file path of this song before being formatted
    file_path: String,

    /// Name of the song on its own
    name: String,

    /// Completed/rendered name, using [Song::name] alongside [Song::artist] and
    /// [Song::album] with the use of em dashes to "pretty print" the song
    render: String,

    /// File extension of the song file (if any), stemming from [Song::file_path]
    ext: Option<String>,

    /// Optional artist if found. If not, this will default to "unknown" artist
    artist: Option<String>,

    /// Optional album name if found. If not, it will be removed from [Song::render]
    /// all together
    album: Option<String>,
}

impl SingleSong {
    /// Creates a new [Song] from given arguments. The `artist` and/or `albumn`
    /// will overwrite their respective positions in the [Song] structure
    pub fn new(
        file_path: impl AsRef<str>,
        artist: impl Into<Option<String>>,
        album: impl Into<Option<String>>,
    ) -> Self {
        let artist = artist.into();
        let album = album.into();

        let (filename, ext) = cap_filename_ext(file_path.as_ref());

        let name = format_name(filename);

        let render = format!(
            "{}{} — {}",
            match &artist {
                Some(a) => format_name(a),
                None => "Unknown artist".to_string(),
            },
            match &album {
                Some(album) => if album != &name {
                    format!(" — {}", format_name(album))
                } else {
                    String::new()
                },
                None => String::new(),
            },
            name
        );

        Self {
            file_path: file_path.as_ref().to_string(),
            name,
            render,
            ext,
            artist,
            album,
        }
    }
}

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
pub fn song(
    name: String,
    album: Option<String>,
    artist: Option<String>,
) -> ResponseModel<SingleSong> {
    ResponseModel::new(200, "Success", SingleSong::new(name, artist, album))
}

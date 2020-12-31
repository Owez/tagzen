//! TV tagging for season-episode based tagging

/// Regex for capturing episodes, global and case insensitive
const EPISODE_REGEX: &str = r"(?i)(e(p(isode)?)? *[0-9]+){1}";

/// Regex for capturing episodes, global and case insensitive
const SEASON_REGEX: &str = r"(?i)(s(eason)? *[0-9]+){1}";

/// Gets numbers from a known regex such as [EPISODE_REGEX] or [SEASON_REGEX]
const NUMBER_REGEX: &str = r"[0-9]+";

use crate::utils::ResponseModel;

use regex::{Match, Regex};
use serde::Serialize;
use std::fmt;

/// Attempts to capture `filename` used and `ext` used from a given `file_path`
/// by splitting
fn cap_filename_ext(file_path: &str) -> (String, Option<String>) {
    let split: Vec<&str> = file_path.split('.').collect();

    (
        split[0].to_string(),
        if split.len() > 1 {
            Some(format!(".{}", split[1..].join(".")))
        } else {
            None
        },
    )
}

/// Finds captured number from a given [Match]; ensure regex passed has numbers
fn cap_num(captured: Match<'_>) -> usize {
    Regex::new(NUMBER_REGEX)
        .expect("Could not make number regex")
        .find(captured.as_str())
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

/// Attempts to capture episode number of a given `filename` using regex
fn cap_episode(filename: &str) -> Result<usize, CaptureError> {
    match Regex::new(EPISODE_REGEX)
        .expect("Could not make episode regex")
        .find(filename)
    {
        Some(episode) => Ok(cap_num(episode)),
        None => Err(CaptureError::NoEpisodeRegex),
    }
}

/// Attempts to capture season number of a given `filename` using regex
fn cap_season(filename: &str) -> Result<usize, CaptureError> {
    match Regex::new(SEASON_REGEX)
        .expect("Could not make season regex")
        .find(filename)
    {
        Some(season) => Ok(cap_num(season)),
        None => Err(CaptureError::NoSeasonRegex),
    }
}

/// Context for captures which if provided, takes precidence over any parsed regex
///
/// Providing this is advised as it increases speed due to no reliance on regex
/// parsing of a given [Capture::file_path] string
#[derive(Debug, PartialEq, Clone)]
pub struct Context {
    /// Episode number, equates to [Capture::episode]
    pub episode: Option<usize>,

    /// Season number, equates to [Capture::season]
    pub season: Option<usize>,
}

/// Error enum encapsulating errors that may arrise on the creation of a [Capture]
/// structure
#[derive(Debug, PartialEq, Clone)]
pub enum CaptureError {
    /// No season context supplied and season number wasn't found in regex capture
    /// from [cap_season]
    NoSeasonRegex,

    /// No episode context supplied and season number wasn't found in regex capture
    /// from [cap_episode]
    NoEpisodeRegex,
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureError::NoSeasonRegex => write!(
                f,
                "No season number passed as context and it wasn't found in the file name either"
            ),
            CaptureError::NoEpisodeRegex => write!(
                f,
                "No episode number passed as context and it wasn't found in the file name either"
            ),
        }
    }
}

/// Single capture found using passed [Context] or with help from a given `cap_x`
/// function for regex capturing
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Capture {
    /// Original, pre-parsed file_path provided
    pub file_path: String,

    /// Parsed [Capture::file_path] without extension (if any). Similar to
    /// [std::path::PathBuf::file_stem]
    pub filename: String,

    /// Optional file extension found from parsing [Capture::file_path]
    pub ext: Option<String>,

    /// Episode number
    pub episode: usize,

    /// Season number
    pub season: usize,
}

impl Capture {
    /// Creates a new [Capture] from filepath and optional context to help it along
    pub fn new(file_path: String, context: Context) -> Result<Self, CaptureError> {
        let (filename, ext) = cap_filename_ext(&file_path);

        Ok(Self {
            file_path,
            ext,
            episode: match context.episode {
                Some(ep) => ep,
                None => cap_episode(&filename)?,
            },
            season: match context.season {
                Some(se) => se,
                None => cap_season(&filename)?,
            },
            filename,
        })
    }
}

/// Single file capture with [Context] providers
#[get("/tv/single?<name>&<episode>&<season>")]
pub fn single(
    name: String,
    episode: Option<usize>,
    season: Option<usize>,
) -> ResponseModel<Capture> {
    match Capture::new(name, Context { episode, season }) {
        Ok(cap) => ResponseModel::new(200, "Success", cap),
        Err(err) => ResponseModel::basic(400, err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1: (&str, usize, usize) = ("hello s01 e02 hi.mp4", 2, 1);
    const TEST_2: (&str, usize, usize) = ("xs01e20.epc", 20, 1);
    const TEST_3: (&str, usize, usize) = ("SEASON3EPISODE4", 4, 3);
    const TEST_4: (&str, usize, usize) = ("EPIsode2 and SEASOn 0002.exy", 2, 2);
    const TEST_5: (&str, usize, usize) = ("hiS01E04.ex", 4, 1);

    #[test]
    fn episode_simple() {
        assert_eq!(cap_episode(TEST_1.0), Ok(TEST_1.1));
        assert_eq!(cap_episode(TEST_2.0), Ok(TEST_2.1));
        assert_eq!(cap_episode(TEST_3.0), Ok(TEST_3.1));
        assert_eq!(cap_episode(TEST_4.0), Ok(TEST_4.1));
        assert_eq!(cap_episode(TEST_5.0), Ok(TEST_5.1));
    }

    #[test]
    fn season_simple() {
        assert_eq!(cap_season(TEST_1.0), Ok(TEST_1.2));
        assert_eq!(cap_season(TEST_2.0), Ok(TEST_2.2));
        assert_eq!(cap_season(TEST_3.0), Ok(TEST_3.2));
        assert_eq!(cap_season(TEST_4.0), Ok(TEST_4.2));
        assert_eq!(cap_season(TEST_5.0), Ok(TEST_5.2));
    }
}

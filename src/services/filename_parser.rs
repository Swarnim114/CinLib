// turns a filename into a Media (title, year, resolution, codec, source, season, episode).
// pure string parsing, no file reading here.

use crate::models::media::Media;
use std::path::Path;

// codec tags we can spot in a filename
const CODEC_WORDS: [&str; 5] = ["x264", "x265", "h264", "h265", "hevc"];

// source tags we can spot in a filename
const SOURCE_WORDS: [&str; 7] = [
    "bluray", "webrip", "webdl", "web-dl", "brrip", "dvdrip", "hdrip",
];

// other junk we just throw away (not worth keeping as a field)
const OTHER_JUNK_WORDS: [&str; 2] = ["aac", "yts"];

// builds a Media from a file path by pulling info out of the filename
pub fn from_filename(path: &Path) -> Media {
    let extension = get_extension(path);
    let stem = get_stem(path);

    // pass 1: look for tags everywhere, even inside [] and ()
    // (open_brackets turns bracket chars into spaces but keeps whats inside them)
    let opened = open_brackets(&stem);
    let tag_spaced = space_out(&opened);
    let (year, resolution, codec, source, season, episode) = extract_tags(&tag_spaced);

    // pass 2: build the title, this time [] and () and everything inside them is gone
    let no_brackets = strip_brackets(&stem);
    let title_spaced = space_out(&no_brackets);
    let title = build_title(&title_spaced);

    Media {
        path: path.to_path_buf(),
        title,
        year,
        resolution,
        codec,
        source,
        extension,
        season,
        episode,
    }
}

// replaces dots, underscores and dashes with spaces
fn space_out(text: &str) -> String {
    text.replace('.', " ").replace('_', " ").replace('-', " ")
}

// scans words and pulls out year, resolution, codec, source, season, episode if found
// returns the first match of each one
fn extract_tags(
    text: &str,
) -> (
    Option<u32>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<u32>,
    Option<u32>,
) {
    let mut year = None;
    let mut resolution = None;
    let mut codec = None;
    let mut source = None;
    let mut season = None;
    let mut episode = None;

    for word in text.split_whitespace() {
        let lower = word.to_lowercase();

        if resolution.is_none() && is_resolution_word(&lower) {
            resolution = Some(lower.clone());
        }

        if year.is_none() && is_year_word(&lower) {
            let parsed_year = lower.parse::<u32>();
            if parsed_year.is_ok() {
                year = Some(parsed_year.unwrap());
            }
        }

        if codec.is_none() && is_in_list(&lower, &CODEC_WORDS) {
            codec = Some(lower.clone());
        }

        if source.is_none() && is_in_list(&lower, &SOURCE_WORDS) {
            source = Some(lower.clone());
        }

        if season.is_none() {
            let parsed = parse_season_episode(&lower);
            if parsed.0.is_some() {
                season = parsed.0;
                episode = parsed.1;
            }
        }
    }

    (year, resolution, codec, source, season, episode)
}

// parses a word like "s01e03" into (Some(season), Some(episode))
// if the word doesnt match, returns (None, None)
fn parse_season_episode(word: &str) -> (Option<u32>, Option<u32>) {
    if !word.starts_with('s') {
        return (None, None);
    }

    let rest = &word[1..];
    let e_pos = rest.find('e');
    if e_pos.is_none() {
        return (None, None);
    }
    let e_pos = e_pos.unwrap();

    let season_text = &rest[..e_pos];
    let episode_text = &rest[e_pos + 1..];

    if season_text.len() == 0 || episode_text.len() == 0 {
        return (None, None);
    }
    if !is_all_digits(season_text) || !is_all_digits(episode_text) {
        return (None, None);
    }

    let season_number = season_text.parse::<u32>();
    let episode_number = episode_text.parse::<u32>();

    if season_number.is_err() || episode_number.is_err() {
        return (None, None);
    }

    (Some(season_number.unwrap()), Some(episode_number.unwrap()))
}

// scans words and builds the title, stops as soon as it sees the first tag
// (year, resolution, codec, source, season/episode) since everything after
// that in a real release name is junk (release group, hash, uploader, etc)
fn build_title(text: &str) -> String {
    let mut title_words = Vec::new();

    for word in text.split_whitespace() {
        let lower = word.to_lowercase();

        if is_tag_word(&lower) {
            break;
        }

        title_words.push(word);
    }

    title_words.join(" ").trim().to_string()
}

// checks if a word is any kind of tag we recognize (not the title anymore)
fn is_tag_word(lower: &str) -> bool {
    if is_resolution_word(lower) {
        return true;
    }
    if is_year_word(lower) {
        return true;
    }
    if is_in_list(lower, &CODEC_WORDS) {
        return true;
    }
    if is_in_list(lower, &SOURCE_WORDS) {
        return true;
    }
    if is_in_list(lower, &OTHER_JUNK_WORDS) {
        return true;
    }
    // short leftover numbers from things like 5.1 audio
    if is_all_digits(lower) && lower.len() <= 2 {
        return true;
    }
    // s01e03 style season/episode marker
    let parsed = parse_season_episode(lower);
    if parsed.0.is_some() {
        return true;
    }

    false
}

fn get_extension(path: &Path) -> String {
    let extension = path.extension();
    if extension.is_none() {
        return String::new();
    }
    let extension = extension.unwrap();

    let extension = extension.to_str();
    if extension.is_none() {
        return String::new();
    }

    extension.unwrap().to_string()
}

fn get_stem(path: &Path) -> String {
    let stem = path.file_stem();
    if stem.is_none() {
        return String::new();
    }
    let stem = stem.unwrap();

    let stem = stem.to_str();
    if stem.is_none() {
        return String::new();
    }

    stem.unwrap().to_string()
}

// turns [ ] ( ) into spaces but keeps whatever text was inside them
fn open_brackets(name: &str) -> String {
    let mut result = String::new();

    for c in name.chars() {
        if c == '[' || c == ']' || c == '(' || c == ')' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }

    result
}

// removes anything inside [] or () including the brackets themselves
fn strip_brackets(name: &str) -> String {
    let mut result = String::new();
    let mut inside_brackets = false;

    for c in name.chars() {
        if c == '[' || c == '(' {
            inside_brackets = true;
            continue;
        }
        if c == ']' || c == ')' {
            inside_brackets = false;
            continue;
        }
        if !inside_brackets {
            result.push(c);
        }
    }

    result
}

// checks if a word looks like a resolution tag, eg 1080p, 720p, 2160p
fn is_resolution_word(word: &str) -> bool {
    if !word.ends_with('p') {
        return false;
    }
    let digits = &word[..word.len() - 1];
    is_all_digits(digits) && digits.len() > 0
}

// checks if a word looks like a movie year, eg 1900 to 2099
fn is_year_word(word: &str) -> bool {
    if !is_all_digits(word) {
        return false;
    }
    if word.len() != 4 {
        return false;
    }
    word.starts_with("19") || word.starts_with("20")
}

fn is_in_list(word: &str, list: &[&str]) -> bool {
    for item in list.iter() {
        if word.contains(item) {
            return true;
        }
    }
    false
}

// checks if a string is only made of digit characters
fn is_all_digits(text: &str) -> bool {
    if text.len() == 0 {
        return false;
    }
    for c in text.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

// tests only run with `cargo test`, not included in the real app
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_movie_with_brackets() {
        let path = Path::new("Avengers.Infinity.War.2018.1080p.BluRay.x264-[YTS.AM].mp4");
        let media = from_filename(path);

        assert_eq!(media.title, "Avengers Infinity War");
        assert_eq!(media.year, Some(2018));
        assert_eq!(media.resolution, Some("1080p".to_string()));
        assert_eq!(media.codec, Some("x264".to_string()));
        assert_eq!(media.source, Some("bluray".to_string()));
        assert_eq!(media.extension, "mp4");
    }

    #[test]
    fn movie_with_no_year() {
        let path = Path::new("Some Random Movie.mkv");
        let media = from_filename(path);

        assert_eq!(media.title, "Some Random Movie");
        assert_eq!(media.year, None);
    }

    #[test]
    fn movie_with_no_brackets() {
        let path = Path::new("The.Promised.Land.2023.1080p.BluRay.x264.AAC5.1-YTS.MX.mp4");
        let media = from_filename(path);

        assert_eq!(media.title, "The Promised Land");
        assert_eq!(media.year, Some(2023));
    }

    #[test]
    fn series_episode() {
        let path = Path::new("Breaking.Bad.S01E03.1080p.BluRay.x264-GROUP.mkv");
        let media = from_filename(path);

        assert_eq!(media.title, "Breaking Bad");
        assert_eq!(media.season, Some(1));
        assert_eq!(media.episode, Some(3));
    }

    #[test]
    fn tags_hidden_inside_brackets() {
        let path = Path::new("[Erai-raws] Look Back - Movie [1080p][HEVC][F80D4C5D].mkv");
        let media = from_filename(path);

        assert_eq!(media.title, "Look Back Movie");
        assert_eq!(media.resolution, Some("1080p".to_string()));
        assert_eq!(media.codec, Some("hevc".to_string()));
    }

    #[test]
    fn multi_word_title_no_junk_after() {
        let path = Path::new("The Grand Budapest Hotel.mkv");
        let media = from_filename(path);

        assert_eq!(media.title, "The Grand Budapest Hotel");
    }

    #[test]
    fn dotfile_does_not_crash() {
        // .mkv has no real extension by rust's rules (dotfile), stem is ".mkv"
        let path = Path::new(".mkv");
        let media = from_filename(path);

        assert_eq!(media.title, "mkv");
        assert_eq!(media.extension, "");
    }
}

// organizes a flat list of Media into movies and series.
// no file reading, no parsing, just sorting things into buckets.

use crate::models::media::{Media, Series};

// splits scanned media into movies (no season/episode) and series (grouped by title)
pub fn group_media(media_list: Vec<Media>) -> (Vec<Media>, Vec<Series>) {
    let mut movies = Vec::new();
    let mut series_list: Vec<Series> = Vec::new();

    for media in media_list {
        if media.season.is_none() {
            movies.push(media);
            continue;
        }

        let existing_index = find_series_index(&series_list, &media.title);
        if existing_index.is_none() {
            series_list.push(Series {
                title: media.title.clone(),
                episodes: vec![media],
            });
        } else {
            let index = existing_index.unwrap();
            series_list[index].episodes.push(media);
        }
    }

    for series in series_list.iter_mut() {
        sort_episodes(&mut series.episodes);
    }

    (movies, series_list)
}

// finds a series in the list with a matching title (case insensitive)
fn find_series_index(series_list: &Vec<Series>, title: &str) -> Option<usize> {
    let lower_title = title.to_lowercase();

    for i in 0..series_list.len() {
        if series_list[i].title.to_lowercase() == lower_title {
            return Some(i);
        }
    }

    None
}

// sorts episodes by season number then episode number, lowest first
// sort_by takes a compare function and reorders the list using it
fn sort_episodes(episodes: &mut Vec<Media>) {
    episodes.sort_by(|a, b| {
        let a_key = (a.season.unwrap_or(0), a.episode.unwrap_or(0));
        let b_key = (b.season.unwrap_or(0), b.episode.unwrap_or(0));
        a_key.cmp(&b_key)
    });
}

// tests only run with `cargo test`, not included in the real app
#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::filename_parser::from_filename;
    use std::path::Path;

    #[test]
    fn group_media_groups_episodes_by_show() {
        let ep1 = from_filename(Path::new("Breaking.Bad.S01E01.mkv"));
        let ep2 = from_filename(Path::new("Breaking.Bad.S01E02.mkv"));
        let movie = from_filename(Path::new("Some Movie.mkv"));

        let (movies, series_list) = group_media(vec![ep1, ep2, movie]);

        assert_eq!(movies.len(), 1);
        assert_eq!(series_list.len(), 1);
        assert_eq!(series_list[0].episodes.len(), 2);
    }

    #[test]
    fn group_media_sorts_episodes_in_order() {
        // fed in out of order on purpose
        let ep3 = from_filename(Path::new("Show.S01E03.mkv"));
        let ep1 = from_filename(Path::new("Show.S01E01.mkv"));
        let ep2 = from_filename(Path::new("Show.S01E02.mkv"));

        let (_movies, series_list) = group_media(vec![ep3, ep1, ep2]);

        assert_eq!(series_list[0].episodes[0].episode, Some(1));
        assert_eq!(series_list[0].episodes[1].episode, Some(2));
        assert_eq!(series_list[0].episodes[2].episode, Some(3));
    }

    #[test]
    fn group_media_matches_title_case_insensitive() {
        let ep1 = from_filename(Path::new("breaking.bad.S01E01.mkv"));
        let ep2 = from_filename(Path::new("Breaking.Bad.S01E02.mkv"));

        let (_movies, series_list) = group_media(vec![ep1, ep2]);

        assert_eq!(series_list.len(), 1);
        assert_eq!(series_list[0].episodes.len(), 2);
    }
}

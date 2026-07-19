mod models;
mod services;

use std::path::Path;

fn main() {
    let p = Path::new("test_media");

    let found_files = services::scanner::walk_dir(p);

    let mut media_list = Vec::new();
    for path in found_files.iter() {
        let media = models::media::from_filename(path);
        media_list.push(media);
    }

    let (movies, series_list) = models::media::group_media(media_list);

    println!("movies ({}):", movies.len());
    for media in movies.iter() {
        print_media(media);
    }

    println!("series ({}):", series_list.len());
    for series in series_list.iter() {
        println!("show: {} ({} episodes)", series.title, series.episodes.len());
        for episode in series.episodes.iter() {
            print_media(episode);
        }
        println!("===");
    }
}

// prints one Media in a readable way instead of the raw debug dump
fn print_media(media: &models::media::Media) {
    println!("title:      {}", media.title);
    println!("year:       {}", option_to_text(&media.year_as_text()));
    println!("resolution: {}", option_to_text(&media.resolution));
    println!("codec:      {}", option_to_text(&media.codec));
    println!("source:     {}", option_to_text(&media.source));
    println!("extension:  {}", media.extension);
    println!("season:     {}", option_number_to_text(media.season));
    println!("episode:    {}", option_number_to_text(media.episode));
    println!("path:       {:?}", media.path);
    println!("---");
}

// turns Option<String> into plain text, "unknown" if its empty
fn option_to_text(value: &Option<String>) -> String {
    if value.is_none() {
        return "unknown".to_string();
    }
    value.clone().unwrap()
}

// turns Option<u32> into plain text, "unknown" if its empty
fn option_number_to_text(value: Option<u32>) -> String {
    if value.is_none() {
        return "unknown".to_string();
    }
    value.unwrap().to_string()
}

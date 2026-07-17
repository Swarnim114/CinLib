use dioxus::prelude::*;
use crate::app::state::{AppState, DbStatus};
use crate::domain::Movie;
use crate::services::{library, playback};

#[component]
pub fn LibraryGrid() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    
    // We fetch movies when the component mounts
    let movies = use_resource(move || async move {
        let state = app_state.read();
        match &state.db {
            DbStatus::Ready(pool) => library::get_movies(pool).await.unwrap_or_default(),
            _ => Vec::new(),
        }
    });

    rsx! {
        div { class: "library",
            header { class: "library__header",
                h2 { "Your Movies" }
            }
            
            match movies.read_unchecked().as_ref() {
                Some(items) if items.is_empty() => rsx! {
                    div { class: "library__empty",
                        p { "No movies found. Try rescanning your library." }
                    }
                },
                Some(items) => rsx! {
                    div { class: "movie-grid",
                        for movie in items {
                            MovieCard { movie: movie.clone() }
                        }
                    }
                },
                None => rsx! {
                    div { class: "library__loading",
                        div { class: "spinner" }
                        p { "Loading movies..." }
                    }
                }
            }
        }
    }
}

#[component]
fn MovieCard(movie: Movie) -> Element {
    let play_movie = {
        let path = movie.file_path.clone();
        move |_| {
            if let Err(e) = playback::play_media(&path) {
                tracing::error!("Failed to play {}: {}", path.display(), e);
            }
        }
    };

    let poster_url = movie.poster_path.as_ref()
        .map(|p| format!("file://{}", p.display()))
        .unwrap_or_else(|| "assets/images/placeholder.png".to_string());

    rsx! {
        div { class: "movie-card", onclick: play_movie,
            div { class: "movie-card__poster-wrapper",
                img {
                    class: "movie-card__poster",
                    src: "{poster_url}",
                    alt: "{movie.title}",
                    loading: "lazy"
                }
                div { class: "movie-card__overlay",
                    button { class: "btn-play-icon", "▶" }
                }
            }
            div { class: "movie-card__info",
                h3 { class: "movie-card__title", "{movie.title}" }
                if let Some(year) = movie.added_at.format("%Y").to_string().into() {
                    span { class: "movie-card__year", "{year}" }
                }
            }
        }
    }
}

pub mod grid;
pub mod state;

use crate::app::grid::LibraryGrid;
use crate::app::state::{AppState, DbStatus};
use crate::domain::LibraryKind;
use crate::infra;
use crate::services::{library, scanner};
use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use tracing::{error, info};

#[component]
pub fn Root() -> Element {
    let mut app_state = use_context_provider(|| Signal::new(AppState::new()));

    use_future(move || async move {
        let db_path = infra::app_dirs::db_path();
        match infra::db::init_pool(&db_path).await {
            Ok(pool) => {
                app_state.write().db = DbStatus::Ready(pool.clone());
                // Load libraries on startup
                if let Ok(libs) = library::get_all_libraries(&pool).await {
                    app_state.write().libraries = libs;
                }
            }
            Err(e) => {
                error!(error = %e, "db init failed");
                app_state.write().db = DbStatus::Failed(e.to_string());
            }
        }
    });

    let state = app_state.read();
    
    rsx! {
        style { {include_str!("../../assets/styles/base.css")} }
        
        match &state.db {
            DbStatus::Initialising => rsx! { Splash { message: "Initialising database…" } },
            DbStatus::Failed(_)    => rsx! { Splash { message: "Database error — check logs" } },
            DbStatus::Ready(_) => {
                if state.libraries.is_empty() {
                    rsx! { SetupScreen {} }
                } else {
                    rsx! { LibraryGrid {} }
                }
            }
        }
    }
}

#[component]
fn Splash(message: String) -> Element {
    rsx! {
        div { class: "splash",
            div { class: "splash__logo",
                span { class: "splash__icon", "🎬" }
                h1 { class: "splash__title", "CinLib" }
                p  { class: "splash__tagline", "Your local cinema, beautifully organised." }
            }
            p { class: "splash__note", "{message}" }
        }
    }
}

#[component]
fn SetupScreen() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();

    let add_folder = move |_| {
        spawn(async move {
            let pool = match &app_state.read().db {
                DbStatus::Ready(p) => p.clone(),
                _ => return,
            };

            if let Some(folder) = AsyncFileDialog::new().pick_folder().await {
                let name = folder.file_name();
                let path = folder.path().to_path_buf();
                
                // For now, default to Mixed kind
                match library::add_library(&pool, &name, path, LibraryKind::Mixed).await {
                    Ok(new_lib) => {
                        info!("Added library: {}", new_lib.name);
                        app_state.write().libraries.push(new_lib.clone());
                        
                        // Kick off a scan
                        if let Err(e) = scanner::scan_library(&pool, &new_lib).await {
                            error!("Scan failed: {}", e);
                        }
                    }
                    Err(e) => error!("Failed to add library: {}", e),
                }
            }
        });
    };

    rsx! {
        div { class: "setup",
            h2 { "Welcome to CinLib" }
            p { "Let's set up your first media library." }
            button {
                class: "btn-primary",
                onclick: add_folder,
                "Select Folder"
            }
        }
    }
}

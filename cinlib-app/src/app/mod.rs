pub mod state;

use crate::app::state::{AppState, DbStatus};
use crate::infra;
use dioxus::prelude::*;
use tracing::error;

#[component]
pub fn Root() -> Element {
    let mut app_state = use_context_provider(|| Signal::new(AppState::new()));

    use_future(move || async move {
        let db_path = infra::app_dirs::db_path();
        match infra::db::init_pool(&db_path).await {
            Ok(pool) => app_state.write().db = DbStatus::Ready(pool),
            Err(e)   => {
                error!(error = %e, "db init failed");
                app_state.write().db = DbStatus::Failed(e.to_string());
            }
        }
    });

    let status_msg = match app_state.read().db {
        DbStatus::Initialising  => "Initialising database…",
        DbStatus::Ready(_)      => "Ready",
        DbStatus::Failed(_)     => "Database error — check logs",
    };

    rsx! {
        style { {include_str!("../../assets/styles/base.css")} }
        div { class: "splash",
            div { class: "splash__logo",
                span { class: "splash__icon", "🎬" }
                h1 { class: "splash__title", "CinLib" }
                p  { class: "splash__tagline", "Your local cinema, beautifully organised." }
            }
            p { class: "splash__note", "{status_msg}" }
        }
    }
}

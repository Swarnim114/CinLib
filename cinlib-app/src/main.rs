pub mod app;
pub mod domain;
pub mod infra;
pub mod services;

use dioxus::prelude::*;
use tracing::info;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("CINLIB_LOG")
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(true)
        .compact()
        .init();

    info!(version = env!("CARGO_PKG_VERSION"), "starting CinLib");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("CinLib")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1280.0_f64, 800.0_f64))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(800.0_f64, 500.0_f64)),
        )
        .with_menu(None);

    LaunchBuilder::desktop().with_cfg(config).launch(app::Root);
}

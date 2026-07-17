use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub fn play_media<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    info!(file = %path.display(), "launching media player");

    // We'll use `xdg-open` on linux to launch the default media player,
    // or try `mpv` directly if xdg-open isn't available.
    if Command::new("xdg-open").arg(path).spawn().is_ok() {
        return Ok(());
    }

    if Command::new("mpv").arg(path).spawn().is_ok() {
        return Ok(());
    }

    if Command::new("vlc").arg(path).spawn().is_ok() {
        return Ok(());
    }

    anyhow::bail!("No suitable media player found to play {}", path.display())
}

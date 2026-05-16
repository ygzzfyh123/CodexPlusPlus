#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use anyhow::Result;
use codex_plus_core::launcher::{LaunchOptions, launch_and_inject};

#[tokio::main]
async fn main() -> Result<()> {
    let handle = launch_and_inject(LaunchOptions::default()).await?;
    handle.wait_for_codex_exit().await?;
    Ok(())
}

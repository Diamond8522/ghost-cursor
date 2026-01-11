mod brain;
mod ghost;
mod watcher;

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1b[35m👻 GHOST CURSOR: INITIALIZING ON KALI...\x1b[0m");

    let model_path = "./models/ghost-brain.gguf";
    let brain = Arc::new(brain::GhostBrain::new(model_path));

    println!("✅ Brain Loaded. Watching for terminal errors...");

    let watcher_brain = Arc::clone(&brain);
    
    tokio::task::spawn_blocking(move || {
        watcher::start_ghost_session(watcher_brain);
    }).await?;

    Ok(())
}

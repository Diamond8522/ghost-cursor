use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::thread;
use std::sync::Arc;
use crate::brain::GhostBrain;
use crate::ghost;

pub fn start_ghost_session(brain: Arc<GhostBrain>) {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24, cols: 80, pixel_width: 0, pixel_height: 0,
    }).expect("Failed to open PTY");

    let cmd = CommandBuilder::new("zsh");
    let _child = pair.slave.spawn_command(cmd).expect("Failed to spawn shell");

    let mut reader = pair.master.try_clone_reader().expect("Failed to get reader");
    let brain_clone = Arc::clone(&brain);

    thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        let mut captured_text = String::new();

        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 { break; }
            let chunk = String::from_utf8_lossy(&buffer[..n]);
            captured_text.push_str(&chunk);

            // COLOR LOGIC: Red for permission denied, Cyan for command typos
            if chunk.contains("not found") || chunk.contains("did you mean") || chunk.contains("error:") || chunk.contains("denied") {
                let ghost_color = if chunk.contains("denied") { "\x1b[31m" } else { "\x1b[32m" };
                let context = if captured_text.len() > 150 { &captured_text[captured_text.len()-150..] } else { &captured_text };

                print!("\r\n\x1b[35m👻 [Ghost is analyzing...]\x1b[0m");
                std::io::stdout().flush().unwrap();
                
                let advice = brain_clone.generate_advice(context);
                
                ghost::clear_ghost_overlay();
                print!("{}💡 Ghost Advice: {}\x1b[0m\r\n", ghost_color, advice.trim());
                std::io::stdout().flush().unwrap();
            }

            if captured_text.len() > 2000 { captured_text.drain(..1000); }
            std::io::stdout().write_all(&buffer[..n]).unwrap();
            std::io::stdout().flush().unwrap();
        }
    });

    let mut writer = pair.master.take_writer().expect("Failed to get writer");
    let mut stdin = std::io::stdin();
    let mut input_buffer = [0u8; 1024];
    while let Ok(n) = stdin.read(&mut input_buffer) {
        let _ = writer.write_all(&input_buffer[..n]);
        let _ = writer.flush();
    }
}

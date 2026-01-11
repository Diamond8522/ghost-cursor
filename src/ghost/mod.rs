use std::io::{self, Write};

pub fn clear_ghost_overlay() {
    print!("\x1B[2K\r"); 
    let _ = io::stdout().flush();
}

#[allow(dead_code)]
pub fn print_ghost_prefix() {
    print!("\x1b[35m👻 [Ghost]: \x1b[0m");
    let _ = io::stdout().flush();
}

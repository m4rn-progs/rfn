use crossterm::{cursor, event, execute, terminal, ExecutableCommand};

use std::io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Write};
mod draw;
mod mouse;
mod shutdown;
mod startup;
use shutdown::shutdown;
use startup::startup;
mod inputs;
mod modal;
mod properties;
fn main() {
    let mut ed = startup();
    while !ed.quit {
        draw::draw(&mut ed);
        inputs::inputs(&mut ed);
        mouse::scan(&mut ed);
    }
    shutdown();
}

use pipeviewer::{args::Args, read, stats, write};
use std::{io::Result, sync::mpsc, thread};
use term_size::dimensions;

fn main() -> Result<()> {
    let term_width = match dimensions() {
        Some((w, _)) => w,
        None => 80, // Use a default width if unable to determine
    };
    let args = Args::parse();

    let Args {
        input,
        output,
        silent,
    } = args;

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn(move || read::read_loop(&input, stats_tx));
    let stats_handle =
        thread::spawn(move || stats::stats_loop(silent, term_width, stats_rx, write_tx));
    let write_handle = thread::spawn(move || write::write_loop(&output, write_rx));

    // crash if any thread have crashed
    // `.jsoin()` return a `thread::Result<io::Result<()>>
    let read_io_result = read_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();

    // return error when any crash in this thread
    read_io_result?;
    write_io_result?;
    stats_io_result?;

    Ok(())
}

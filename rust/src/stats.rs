mod timer;

use std::{
    io::Result,
    sync::mpsc::{Receiver, Sender},
    time::Instant,
};
use timer::Timer;

pub fn stats_loop(
    silent: bool,
    term_width: usize,
    stats_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    loop {
        let buffer = stats_rx.recv().unwrap();
        let num_bytes = buffer.len();
        timer.update();
        let rps = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.is_ready {
            timer.is_ready = false;
            eprint!("\r{}", " ".repeat(term_width as usize));
            eprint!(
                "\r{:10.1}Mb\t{}s\t[{:.1}Mb/s]",
                total_bytes as f64 / 1_048_576.0,
                start.elapsed().as_secs(),
                rps as f64 / 1_048_576.0
            );
        }
        if write_tx.send(buffer).is_err() {
            break;
        }
        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}

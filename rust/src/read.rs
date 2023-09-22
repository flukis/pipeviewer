use crate::CHUNK_SIZE;

use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::sync::mpsc::Sender;

pub fn read_loop(input: &str, stats_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !input.is_empty() {
        Box::new(BufReader::new(File::open(input)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_bytes = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        // send this buffer to the stats thread
        if stats_tx.send(Vec::from(&buffer[..num_bytes])).is_err() {
            break;
        }
    }
    let _ = stats_tx.send(Vec::new());
    Ok(())
}

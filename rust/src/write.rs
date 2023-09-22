use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::sync::mpsc::Receiver;

pub fn write_loop(output: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !output.is_empty() {
        Box::new(BufWriter::new(File::create(output)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // receive from threads
        let buffer = write_rx.recv().unwrap();
        if buffer.is_empty() {
            break;
        }

        // do write to destination
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // stop program cleanly
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}

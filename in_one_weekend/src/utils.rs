use std::{
    cmp,
    io::{self, BufWriter, Write},
};

pub fn log_progress(progress: f64) -> io::Result<()> {
    debug_assert!((0.0..=1.0).contains(&progress));

    const BAR_WIDTH: u8 = 50;
    let pos = progress * BAR_WIDTH as f64;
    let pos: u8 = if pos <= (BAR_WIDTH - 1) as f64 {
        pos as u8
    } else {
        BAR_WIDTH
    };

    let mut std_err = BufWriter::new(io::stderr());

    std_err.write_all(b"[")?;

    for ele in 0..BAR_WIDTH {
        match ele.cmp(&pos) {
            cmp::Ordering::Less => std_err.write(b"=")?,
            cmp::Ordering::Equal => std_err.write(b">")?,
            cmp::Ordering::Greater => std_err.write(b" ")?,
        };
    }

    let percent = progress * 100.0;
    if 99.9 < percent {
        std_err.write_all(format!("] {percent:.0} % \r\n").as_bytes())?
    } else {
        std_err.write_all(format!("] {percent:.1} %\r").as_bytes())?
    }

    std_err.flush()
}

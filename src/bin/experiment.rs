use std::{fs::File, io::Write};

use indicatif::{ProgressIterator, ProgressStyle};
use path_oram::oram::PathORAM;

const N: usize = 1 << 20;

lazy_static::lazy_static! {
    static ref STYLE: ProgressStyle = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta}) -- {msg}",
    )
    .unwrap();
}

fn experiment<const Z: usize>(warmup: usize, access: usize) {
    println!("Running experiment with Z={}", Z);

    let mut oram = PathORAM::<u32, Z, N>::init().unwrap();

    for i in (0..N)
        .progress()
        .with_style(STYLE.clone())
        .with_message("Initial ORAM writes")
    {
        oram.access(i, Some(i as u32)).unwrap();
    }

    for i in (0..warmup)
        .progress()
        .with_style(STYLE.clone())
        .with_message("ORAM warmup")
    {
        oram.access(i % N, None).unwrap();
    }

    let mut file = File::create(format!("data/{}.txt", Z)).unwrap();
    writeln!(file, "-1,{}", warmup).unwrap();

    for i in (0..access)
        .progress()
        .with_style(STYLE.clone())
        .with_message("Recording ORAM stash sizes")
    {
        oram.access(i % N, None).unwrap();
        let stash_size = oram.stash.len();
        writeln!(file, "{},{}", i, stash_size).unwrap();
    }
}

pub fn main() {
    experiment::<2>(1_000_000, 1_000_000);
    experiment::<4>(3_000_000, 10_000_000);
    experiment::<6>(3_000_000, 10_000_000);
}

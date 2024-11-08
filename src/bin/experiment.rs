use std::{fs::File, io::Write};

use path_oram::oram::PathORAM;

const N: usize = 1 << 20;

fn experiment<const Z: usize>(warmup: usize, access: usize) {
    let mut oram = PathORAM::<u32, Z, N>::init().unwrap();
    // write to the oram first
    (0..N).for_each(|i| {
        oram.access(i, Some(i as u32)).unwrap();
    });

    log::info!("done intial write for z={}", Z);

    // warmup
    (0..warmup).for_each(|i| {
        oram.access(i % N, None).unwrap();
    });

    log::info!("done warmup for z={}", Z);

    let mut file = File::create(format!("data/{}.txt", Z)).unwrap();
    writeln!(file, "-1,{}", warmup).unwrap();

    // measure stash size for each access
    (0..access).for_each(|i| {
        oram.access(i % N, None).unwrap();
        let stash_size = oram.stash.len();
        writeln!(file, "{},{}", i, stash_size).unwrap();
    });

    log::info!("done experiment for z={}", Z);
}

pub fn main() {
    env_logger::init();
    let num_warmup = 1_000_000;
    let num_access = 1_000_000;
    experiment::<2>(num_warmup, num_access);
    experiment::<4>(num_warmup, num_access);
    experiment::<6>(num_warmup, num_access);
}

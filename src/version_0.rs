use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// from 0
// to 12_327_829_503

#[allow(dead_code)]
pub fn run () {
    let proven_base = Arc::new(AtomicU64::new(12_327_829_503));
    let proven_base_clone = Arc::clone(&proven_base);


    ctrlc::set_handler(move || {
        println!("\nStopped at: {}", proven_base_clone.load(Ordering::Relaxed));
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        run_steps_until_reaching_base(proven_base.load(Ordering::Relaxed));
        proven_base.fetch_add(1, Ordering::Relaxed);
        break
    }
}

fn run_steps_until_reaching_base(proven_base: u64) {
    let mut x: u64 = proven_base;
    while x >= proven_base {
        if x % 2 == 0 {
            x /= 2;
            continue;
        }

        x = x.checked_mul(3).unwrap_or_else(|| {
            eprintln!("Overflow happened on {proven_base}");
            panic!("End")
        });
        x += 1;
    }
}
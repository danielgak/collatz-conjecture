use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use num_bigint::BigUint;
use crate::version_0::CtrCPrint::{PrintAtomic, PrintMutex};
// from 0
// to 12_327_829_503

#[allow(dead_code)]
pub fn run() {
    let proven_base = Arc::new(AtomicU64::new(12_327_829_503));
    let proven_base_clone = Arc::clone(&proven_base);
    set_ctrc_handler(PrintAtomic(proven_base_clone));

    loop {
        run_steps_until_reaching_base(proven_base.load(Ordering::Relaxed));
        proven_base.fetch_add(1, Ordering::Relaxed);
        break;
    }
}

pub enum CtrCPrint {
    PrintAtomic(Arc<AtomicU64>),
    PrintMutex(Arc<Mutex<BigUint>>),
}

pub fn set_ctrc_handler(ccp: CtrCPrint) {
    ctrlc::set_handler(move || {
        match &ccp {
            PrintAtomic(value) => println!("\nStopped at: {}", value.load(Ordering::Relaxed)),
            PrintMutex(value) => println!("\nStopped at: {}", value.lock().unwrap())
        };

        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
}

pub fn run_steps_until_reaching_base(proven_base: u64) -> u64 {
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

    x
}

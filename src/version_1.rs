use lazy_static::lazy_static;
use num_bigint::{BigUint, ToBigUint};
use std::ops::{AddAssign, DivAssign, MulAssign, Rem};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref ONE: BigUint = 1.to_biguint().unwrap();
    static ref TWO: BigUint = 2.to_biguint().unwrap();
    static ref THREE: BigUint = 3.to_biguint().unwrap();
}

// from 12_327_829_503
// to 13_164_931_179

pub fn run() {
    let proven_base = Arc::new(Mutex::new(BigUint::from(12758089137u64)));
    let proven_base_clone = Arc::clone(&proven_base);

    ctrlc::set_handler(move || {
        println!(
            "\nStopped at: {}",
            proven_base_clone.lock().unwrap().clone()
        );
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        run_steps_until_reaching_base(&proven_base.lock().unwrap());
        proven_base.lock().unwrap().add_assign(&*ONE);
    }
}

pub fn run_steps_until_reaching_base(proven_base: &BigUint) -> BigUint {
    let mut x = proven_base.clone();
    loop {
        let rem = x.clone().rem(&*TWO);
        if rem == BigUint::ZERO {
            x.div_assign(&*TWO);
            if x < *proven_base {
                break;
            }

            continue;
        }

        x.mul_assign(&*THREE);
        x.add_assign(&*ONE);
    }

    x
}

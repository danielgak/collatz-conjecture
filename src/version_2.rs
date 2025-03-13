use num_bigint::{BigUint };
use std::ops::{AddAssign, DivAssign, MulAssign };
use std::sync::{Arc, Mutex};
use crate::version_0::CtrCPrint::PrintMutex;
use crate::version_0::set_ctrc_handler;

pub fn run() {
    let proven_base = Arc::new(Mutex::new(BigUint::from(13_164_931_179u64)));
    let proven_base_clone = Arc::clone(&proven_base);
    set_ctrc_handler(PrintMutex(proven_base_clone));

    loop {
        run_steps_until_reaching_base(&proven_base.lock().unwrap());
        proven_base.lock().unwrap().add_assign(1u8);
    }
}

pub fn run_steps_until_reaching_base(mut proven_base: &BigUint) -> BigUint {
    let mut x = proven_base.clone();
    loop {
        if isEven(&x) {
            x.div_assign(2u8);
            if x < *proven_base {
                break;
            }

            continue;
        }

        x.mul_assign(3u8);
        x.add_assign(1u8);
    }

    x
}

pub fn isEven(n: &BigUint) -> bool {
    n.bit(0) == false
}
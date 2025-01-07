use ctrlc;
use std::io::{Read, Write};
use std::sync::mpsc::channel;

fn main() {
    let mut proven_base: u64 = 10703654581;
    let (tx, rx) = channel();
    let send_stop_signal = move || tx.send(true).expect("Could not send signal on channel.");
    ctrlc::set_handler(send_stop_signal).expect("Error setting Ctrl-C handler");

    while rx.try_recv().is_err() {
        proven_base += 1;
        run_steps_until_reaching_base(proven_base);
    }

    println!("Stopped proving at {proven_base}");
}

fn run_steps_until_reaching_base(proven_base: u64) {
    let mut pointer: u64 = proven_base;
    while pointer >= proven_base {
        if pointer % 2 == 0 {
            pointer /= 2;
            continue;
        }

        pointer = match pointer.checked_mul(3) {
            Some(result) => result,
            None => {
                eprintln!("Overflow happened on {proven_base}");
                panic!("Bye!")
            }
        };
        pointer += 1;
    }
}

// eg. echo "1200152" | cargo run

fn debug_steps() {
    let stdout = std::io::stdout();
    let mut cout = std::io::BufWriter::new(stdout.lock());

    let mut buf = Vec::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_end(&mut buf).unwrap();
    let mut num = unsafe {
        std::str::from_utf8_unchecked(&buf)
            .trim()
            .parse::<u64>()
            .unwrap()
    };

    write!(cout, "{} ", num).unwrap();
    while num > 1 {
        if num & 1 != 0 {
            num *= 3;
            num += 1;
        } else {
            num /= 2;
        }
        write!(cout, "{} ", num).unwrap();
    }
}

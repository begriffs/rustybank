use std::sync::Mutex;
use std::sync::Arc;

use rand::prelude::*;

const N_ACCTS: usize = 10;
const N_THREADS: usize = 4;

fn disburse_random(accounts: &[Arc<Mutex<i64>>]) {
    let from = rand::thread_rng().gen_range(0, N_ACCTS);
    let to = loop {
        let to = rand::thread_rng().gen_range(0, N_ACCTS);
        if to != from {
            break to;
        }
    };

    println!("Attempting {} -> {}", from, to);

    let amt = {
        let mut from_account = accounts[from].lock().unwrap();

        let amt = rand::thread_rng().gen_range(0, *from_account + 1);
        *from_account -= amt;
        amt
    };

    let mut to_account = accounts[to].lock().unwrap();
    *to_account += amt;

    println!("{} -({})-> {} | {}", from, amt, to, *to_account);
}

fn main() {
    let mut accounts = vec![];
    let mut threads = vec![];

    for _ in 0..N_ACCTS {
        accounts.push(Arc::new(Mutex::new(10i64)));
    }

    for _ in 0..N_THREADS {
        threads.push({
            let accounts = accounts.clone();
            std::thread::spawn(move || for _ in 0..10000 {
                disburse_random(&accounts);
            })
        });
    }
    threads.into_iter().for_each(|t| { t.join().unwrap(); });

    let uaccounts = accounts.into_iter().map(|a| *a.lock().unwrap());

    for (i, a) in uaccounts.clone().enumerate() {
        println!("{}: {}", i, a);
    }

    println!("Final sum: {}", uaccounts.sum::<i64>());
}

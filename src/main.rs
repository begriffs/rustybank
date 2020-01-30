use std::sync::Mutex;
use std::sync::Arc;

use rand::prelude::*;

const N_ACCTS: usize = 10;
const N_THREADS: usize = 4;

fn disburse_random(accounts: &[Arc<Mutex<i64>>])
{
    let from = rand::thread_rng().gen_range(0, N_ACCTS);
    let to = loop {
        let to = rand::thread_rng().gen_range(0, N_ACCTS);
        if to != from {
            break to;
        }
    };

    println!("Attempting {} -> {}", from, to);

    let mut from_account = accounts[from].lock().unwrap();
    let mut to_account = accounts[to].lock().unwrap();
    let amt = rand::thread_rng().gen_range(0, *from_account);
    *from_account -= amt;
    *to_account += amt;

    println!("{} -{}> {}", from, amt, to);
}

fn main() {
    let accounts = vec![
        Arc::new(Mutex::new(0i64)); N_ACCTS
    ];
    let mut threads = vec![];

    for _ in 0..N_THREADS {
        threads.push({
            let accounts = accounts.clone();
            std::thread::spawn(move || {
                loop {
                    disburse_random(&accounts);
                }
            })
        });
    }
    threads.into_iter().for_each(|t| {
        t.join().unwrap();
    });
}

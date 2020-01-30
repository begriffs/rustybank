use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    let balance = Arc::new(Mutex::new(0i64));

    let thread = {
        let balance = balance.clone();
        std::thread::spawn(move || {
            loop {

                match balance.lock() {
                    Ok(mut balance) => {
                        *balance += 1;
                        println!("{}", balance);
                    },
                    Err(e) => eprintln!("NOOOO {:?}", e),
                }
            }
        })
    };

    loop {
        match balance.lock() {
            Ok(mut balance) => {
                *balance -= 1;
                println!("{}", balance);
            },
            Err(e) => eprintln!("NOOOO {:?}", e),
        }
    }
}
